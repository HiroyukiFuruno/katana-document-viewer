use super::super::{
    SpreadsheetAutoFilterArtifact, SpreadsheetCoordinate, SpreadsheetMaterializedCell,
    SpreadsheetOpenedSheet, SpreadsheetSheetArtifact, SpreadsheetViewerLimits,
    spreadsheet_engine_cell::SpreadsheetCellMaterializer,
    spreadsheet_engine_sheet::SpreadsheetSheetBuilder,
    spreadsheet_filter_xml::{SpreadsheetFilterCatalog, SpreadsheetWorksheetMetadata},
    spreadsheet_streaming::StreamingSpreadsheetSession,
};
use super::{SpreadsheetEngineBackend, SpreadsheetEngineError, SpreadsheetEngineSession};
use ironcalc::base::Model;

impl SpreadsheetEngineSession {
    pub(in crate::multi_format) fn open(
        bytes: Vec<u8>,
        name: &str,
        limits: SpreadsheetViewerLimits,
    ) -> Result<Self, SpreadsheetEngineError> {
        let streaming_required = {
            let _trace =
                super::super::debug_trace::DebugTrace::start("spreadsheet.streaming_detection");
            StreamingSpreadsheetSession::is_required(&bytes)?
        };
        let mut worksheet_metadata = {
            let _trace = super::super::debug_trace::DebugTrace::start("spreadsheet.filter_catalog");
            SpreadsheetFilterCatalog::read(&bytes, limits.max_sheets, !streaming_required)?
        };
        let filters = worksheet_metadata
            .iter_mut()
            .map(|metadata| metadata.auto_filter.take())
            .collect();
        if streaming_required {
            return Self::open_streaming(bytes, limits, filters);
        }
        Self::open_model(bytes, name, limits, filters, worksheet_metadata)
    }

    pub(super) fn open_model(
        bytes: Vec<u8>,
        name: &str,
        limits: SpreadsheetViewerLimits,
        filters: Vec<Option<SpreadsheetAutoFilterArtifact>>,
        worksheet_metadata: Vec<SpreadsheetWorksheetMetadata>,
    ) -> Result<Self, SpreadsheetEngineError> {
        let model = super::load_model(&bytes, name)?;
        Self::validate_worksheet_metadata(&model, &worksheet_metadata)?;
        let sheets = Self::build_model_sheets(&model, limits, worksheet_metadata)?;
        let mut session = Self {
            backend: SpreadsheetEngineBackend::Model(Box::new(model)),
            active_filters: Vec::new(),
            sheets,
            filters,
            limits,
        };
        {
            let _trace =
                super::super::debug_trace::DebugTrace::start("spreadsheet.persisted_filters");
            session.initialize_persisted_filters()?;
        }
        Ok(session)
    }

    fn validate_worksheet_metadata(
        model: &Model<'_>,
        metadata: &[SpreadsheetWorksheetMetadata],
    ) -> Result<(), SpreadsheetEngineError> {
        let names = model.workbook.get_worksheet_names();
        if names.len() != metadata.len()
            || names
                .iter()
                .zip(metadata)
                .any(|(model_name, sheet)| model_name != &sheet.name)
        {
            return Err(SpreadsheetEngineError::Import(
                "worksheet metadata does not match imported workbook sheets".into(),
            ));
        }
        Ok(())
    }

    fn build_model_sheets(
        model: &Model<'_>,
        limits: SpreadsheetViewerLimits,
        metadata: Vec<SpreadsheetWorksheetMetadata>,
    ) -> Result<Vec<SpreadsheetSheetArtifact>, SpreadsheetEngineError> {
        let merged_cells = metadata
            .into_iter()
            .map(|sheet| sheet.merged_cells)
            .collect();
        let _trace = super::super::debug_trace::DebugTrace::start("spreadsheet.sheet_artifacts");
        SpreadsheetSheetBuilder::build(
            model,
            limits.max_sheets,
            limits.max_logical_cells,
            merged_cells,
        )
    }

    fn open_streaming(
        bytes: Vec<u8>,
        limits: SpreadsheetViewerLimits,
        filters: Vec<Option<SpreadsheetAutoFilterArtifact>>,
    ) -> Result<Self, SpreadsheetEngineError> {
        let _trace = super::super::debug_trace::DebugTrace::start("spreadsheet.streaming_open");
        let streaming = StreamingSpreadsheetSession::open(bytes, limits)?;
        let sheets = streaming.sheets().to_vec();
        let mut session = Self {
            backend: SpreadsheetEngineBackend::Streaming(streaming),
            active_filters: Vec::new(),
            sheets,
            filters,
            limits,
        };
        {
            let _trace =
                super::super::debug_trace::DebugTrace::start("spreadsheet.persisted_filters");
            session.initialize_persisted_filters()?;
        }
        Ok(session)
    }

    #[cfg(test)]
    pub(in crate::multi_format) fn sheets(&self) -> &[SpreadsheetSheetArtifact] {
        &self.sheets
    }

    pub(in crate::multi_format) fn opened_sheets(&self) -> Vec<SpreadsheetOpenedSheet> {
        self.sheets
            .iter()
            .cloned()
            .zip(self.filters.iter().cloned())
            .map(|(sheet, auto_filter)| SpreadsheetOpenedSheet { sheet, auto_filter })
            .collect()
    }

    #[cfg(test)]
    pub(in crate::multi_format) fn auto_filters(&self) -> &[Option<SpreadsheetAutoFilterArtifact>] {
        &self.filters
    }

    pub(in crate::multi_format) fn auto_filter(
        &self,
        sheet_index: usize,
    ) -> Result<Option<&SpreadsheetAutoFilterArtifact>, SpreadsheetEngineError> {
        if sheet_index >= self.sheets.len() {
            return Err(SpreadsheetEngineError::SheetOutsideDocument {
                requested: sheet_index,
                sheet_count: self.sheets.len(),
            });
        }
        Ok(self.filters.get(sheet_index).and_then(Option::as_ref))
    }

    #[cfg(test)]
    pub(in crate::multi_format) fn auto_filter_mut(
        &mut self,
        sheet_index: usize,
    ) -> Result<Option<&mut SpreadsheetAutoFilterArtifact>, SpreadsheetEngineError> {
        let sheet_count = self.sheets.len();
        if sheet_index >= sheet_count {
            return Err(SpreadsheetEngineError::SheetOutsideDocument {
                requested: sheet_index,
                sheet_count,
            });
        }
        Ok(self.filters.get_mut(sheet_index).and_then(Option::as_mut))
    }

    pub(in crate::multi_format) fn materialize(
        &self,
        sheet_index: usize,
        coordinates: &[SpreadsheetCoordinate],
    ) -> Result<Vec<SpreadsheetMaterializedCell>, SpreadsheetEngineError> {
        self.validate_request(sheet_index, coordinates)?;
        match &self.backend {
            SpreadsheetEngineBackend::Model(model) => coordinates
                .iter()
                .copied()
                .map(|coordinate| {
                    SpreadsheetCellMaterializer::materialize(model, sheet_index, coordinate)
                })
                .collect(),
            SpreadsheetEngineBackend::Streaming(streaming) => streaming
                .materialize(sheet_index, coordinates)
                .map(|cells| {
                    cells
                        .into_iter()
                        .map(SpreadsheetMaterializedCell::without_borders)
                        .collect()
                }),
        }
    }
}

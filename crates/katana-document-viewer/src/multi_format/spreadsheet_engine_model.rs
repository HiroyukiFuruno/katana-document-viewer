use super::SpreadsheetEngineError;
use crate::multi_format::debug_trace::DebugTrace;
use ironcalc::base::Model;

const LANGUAGE: &str = "en";
const LOCALE: &str = "en";
const TIMEZONE: &str = "UTC";

pub(super) fn load_model(
    bytes: &[u8],
    name: &str,
) -> Result<Model<'static>, SpreadsheetEngineError> {
    let workbook = {
        let _trace = DebugTrace::start("spreadsheet.model_import");
        match ironcalc::import::load_from_xlsx_bytes(bytes, name, LOCALE, TIMEZONE) {
            Ok(workbook) => workbook,
            Err(error) => return Err(SpreadsheetEngineError::Import(error.to_string())),
        }
    };
    let mut model = {
        let _trace = DebugTrace::start("spreadsheet.model_init");
        Model::from_workbook(workbook, LANGUAGE).map_err(SpreadsheetEngineError::Model)?
    };
    {
        let _trace = DebugTrace::start("spreadsheet.model_evaluate");
        model.evaluate();
    }
    Ok(model)
}

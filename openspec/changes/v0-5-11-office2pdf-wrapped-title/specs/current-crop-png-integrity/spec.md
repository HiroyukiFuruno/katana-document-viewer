## ADDED Requirements

### Requirement: Native crop evaluation decodes both explicit PNG inputs

The native evaluator SHALL preserve existing SHA, dimensions, frame and clean-interaction provenance checks and SHALL decode the complete pixel payload of both crop and full PNG before visual scoring. A valid IHDR or an updated declaration hash SHALL NOT substitute for decodable image data.

#### Scenario: Complete external input
- **WHEN** the declared crop and full PNG contain valid decodable image data
- **THEN** the evaluator preserves both files and proceeds without changing the reference, geometry or score threshold

#### Scenario: Rehashed corrupt payload
- **WHEN** either declared PNG is header-only or contains corrupt IDAT CRC or zlib data and its manifest SHA is updated to match
- **THEN** native evaluation rejects it before scoring rather than reporting an accepted visual result

### Requirement: Image integrity is not a score category

Image-integrity success SHALL NOT establish semantic, interaction or performance scores or change the existing independent four-category acceptance requirements.

#### Scenario: Nonvisual evidence remains missing
- **WHEN** both PNGs decode but independent nonvisual evidence has not been provided
- **THEN** those categories remain not evaluated rather than inheriting PNG integrity or visual-score results

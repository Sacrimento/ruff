use crate::{verbatim_text, FormatNodeRule, PyFormatter};
use ruff_formatter::{write, Buffer, FormatResult};
use rustpython_parser::ast::PatternMatchValue;

#[derive(Default)]
pub struct FormatPatternMatchValue;

impl FormatNodeRule<PatternMatchValue> for FormatPatternMatchValue {
    fn fmt_fields(&self, item: &PatternMatchValue, f: &mut PyFormatter) -> FormatResult<()> {
        write!(f, [verbatim_text(item.range)])
    }
}

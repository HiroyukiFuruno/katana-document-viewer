pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub struct SpanOps;

impl SpanOps {
    pub fn start(span: proc_macro2::Span) -> Location {
        let start = span.start();
        Location {
            line: start.line,
            column: start.column + 1,
        }
    }

    pub fn end_line(span: proc_macro2::Span) -> usize {
        span.end().line
    }

    pub fn block_end_line(block: &syn::Block) -> usize {
        Self::end_line(block.brace_token.span.close())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::spanned::Spanned;

    #[test]
    fn span_ops_reports_one_based_column_for_token_start() -> Result<(), Box<dyn std::error::Error>>
    {
        let expr: syn::Expr = syn::parse_str("1 + 2")?;
        let location = SpanOps::start(expr.span());

        assert_eq!(location.line, 1);
        assert_eq!(location.column, 1);
        Ok(())
    }

    #[test]
    fn span_ops_end_line_includes_block_close_line() -> Result<(), Box<dyn std::error::Error>> {
        let block: syn::Block = syn::parse_str("{\n    let a = 1;\n}\n")?;
        let end_line = SpanOps::end_line(block.brace_token.span.close());
        let block_end_line = SpanOps::block_end_line(&block);

        assert_eq!(block_end_line, end_line);
        Ok(())
    }
}

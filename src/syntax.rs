use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct SyntaxParser;

pub fn parse_line(line: String) -> Result<SyntaxValue, pest::error::Error<Rule>> {
  SyntaxParser::parse(line);
}

pub enum SyntaxValue {}

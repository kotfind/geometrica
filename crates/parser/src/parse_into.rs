use peg::{error::ParseError, str::LineCol};
use types::lang::{Command, Definition, Expr, Statement};

pub trait ParseInto<T> {
    fn parse_into(self) -> Result<T, ParseError<LineCol>>;
}

impl<T> ParseInto<T> for T {
    fn parse_into(self) -> Result<T, ParseError<LineCol>> {
        Ok(self)
    }
}

impl<T> ParseInto<T> for &str
where
    String: ParseInto<T>,
{
    fn parse_into(self) -> Result<T, ParseError<LineCol>> {
        self.to_string().parse_into()
    }
}

impl ParseInto<Expr> for String {
    fn parse_into(self) -> Result<Expr, ParseError<LineCol>> {
        crate::expr(&self)
    }
}

impl ParseInto<Command> for String {
    fn parse_into(self) -> Result<Command, ParseError<LineCol>> {
        crate::command(&self)
    }
}

impl ParseInto<Definition> for String {
    fn parse_into(self) -> Result<Definition, ParseError<LineCol>> {
        crate::definition(&self)
    }
}

impl ParseInto<Vec<Definition>> for String {
    fn parse_into(self) -> Result<Vec<Definition>, ParseError<LineCol>> {
        crate::definitions(&self)
    }
}

impl ParseInto<Statement> for String {
    fn parse_into(self) -> Result<Statement, ParseError<LineCol>> {
        crate::statement(&self)
    }
}

impl ParseInto<Vec<Statement>> for String {
    fn parse_into(self) -> Result<Vec<Statement>, ParseError<LineCol>> {
        crate::script(&self)
    }
}

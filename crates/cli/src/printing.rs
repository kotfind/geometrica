use client::{ScriptResult, Table};
use core::fmt;
use std::fmt::{Display, Formatter};
use tabled::{
    builder::Builder,
    settings::{
        object::{Cell, Segment},
        Alignment, Modify, Span, Style, Width,
    },
};

pub struct ScriptResultPrinter<'a>(pub &'a ScriptResult);

impl Display for ScriptResultPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let res = self.0;

        for table in &res.results {
            writeln!(f, "{}\n", TablePrinter(table))?;
        }

        if let Some(err) = &res.error {
            writeln!(f, "error: {:?}", err)?;
        }

        Ok(())
    }
}

struct TablePrinter<'a>(&'a Table);

impl Display for TablePrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let from = self.0;

        // Create and fill table
        let mut to = Builder::new();

        to.push_record(from.header());

        if !from.is_empty() {
            for row in from.rows() {
                to.push_record(row);
            }
        } else {
            to.push_record(["*empty*"]);
        }

        // Apply styles
        let mut to = to.build();
        to.with(Style::rounded());

        if from.is_empty() {
            to.with(
                Modify::new(Cell::new(1, 0))
                    .with(Span::column(from.width()))
                    .with(Alignment::center()),
            );
        }

        if let Some(width) = termsize::get().map(|size| size.cols) {
            to.with(Modify::new(Segment::all()).with(Width::wrap(width as usize - from.width() * 6 /* XXX: magic constant that works for whatever reason */)));
        }

        write!(f, "{}", to)
    }
}

use std::fmt::Display;

use crate::Table;

/// Result of script ([`Vec<Statement>`]) execution.
///
/// Script execution may lead to one of the following results:
/// * It may fail entirely (e. g. because of a compilation error). Then `error` will be
///     `Some(...)` and `results` will be empty.
/// * On of the script's commands may fail. Then `error` will be `Some(...)` and `results` will
///     hold the results of the commands that were executed.
/// * Execution succeeded. Then `error` will be `None`.
///
/// Note: the "Result" in the name has noting to do with [`Result`] or [`anyhow::Result`].
#[derive(Debug)]
pub struct ScriptResult {
    // TODO (?): Vec<Table> -> Vec<StatementResult>
    pub results: Vec<Table>,
    pub error: Option<anyhow::Error>,
}

impl ScriptResult {
    pub fn ok(results: impl IntoIterator<Item = Table>) -> Self {
        Self {
            results: results.into_iter().collect(),
            error: None,
        }
    }

    pub fn ok_one(result: Table) -> Self {
        Self {
            results: vec![result],
            error: None,
        }
    }

    pub fn ok_none() -> Self {
        Self {
            results: Vec::new(),
            error: None,
        }
    }

    pub fn error(error: impl Into<anyhow::Error>) -> Self {
        Self {
            results: Vec::new(),
            error: Some(error.into()),
        }
    }

    pub fn partail_error(
        results: impl IntoIterator<Item = Table>,
        error: impl Into<anyhow::Error>,
    ) -> Self {
        Self {
            results: results.into_iter().collect(),
            error: Some(error.into()),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    pub fn is_fail(&self) -> bool {
        self.error.is_some()
    }

    pub fn context<C>(mut self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        if let Some(error) = self.error {
            self.error = Some(error.context(context));
        }
        self
    }

    pub fn with_context<C, F>(mut self, f: F) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        if let Some(error) = self.error {
            self.error = Some(error.context(f()));
        }
        self
    }
}

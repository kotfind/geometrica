//! This module handles serialization and deserialization
//! of the application's state (i.e. of [ExecScope]).

use crate::exec::ExecScope;

mod models;
mod to_stored;

impl ExecScope {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.to_stored())
            .expect("StoredExecScope should always be serializable")
    }
}

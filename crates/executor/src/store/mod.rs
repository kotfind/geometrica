//! This module handles serialization and deserialization
//! of the application's state (i.e. of [ExecScope]).

use thiserror::Error;

use crate::exec::{ExecError, ExecScope};

mod from_stored;
mod models;
mod to_stored;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to parse as json")]
    JsonParseError(#[from] serde_json::Error),

    #[error("corrupted data: {msg}")]
    CorruptedData { msg: String },
}

impl ExecScope {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.to_stored())
            .expect("StoredExecScope should always be serializable")
    }

    pub fn from_json(json: &str) -> Result<Self, ExecError> {
        let stored_exec_scope = serde_json::from_str(json).map_err(LoadError::JsonParseError)?;
        ExecScope::from_stored(stored_exec_scope)
    }
}

#[cfg(test)]
mod test {
    use types::core::Ident;

    use crate::exec::{Exec, ExecScope};

    #[test]
    fn to_json_and_back() {
        fn create_json() -> String {
            let mut scope = ExecScope::new();

            parser::definitions(
                r#"
                fact n:int -> int = if
                    n == 0 then 1
                    else n * (fact (n - 1))

                n = 5
                m = 1 + fact n
                t = m * 2
            "#,
            )
            .unwrap()
            .exec(&mut scope)
            .unwrap();

            scope.to_json()
        }

        fn check_json(json: &str) {
            let mut scope = ExecScope::from_json(json).expect("failed to parse json");

            // Check values
            assert_eq!(scope.get_item(&Ident::from("n")).unwrap(), 5.into());
            assert_eq!(scope.get_item(&Ident::from("m")).unwrap(), 121.into());
            assert_eq!(scope.get_item(&Ident::from("t")).unwrap(), 242.into());

            // Check if recalc works fine
            scope.set(&Ident::from("n"), 3.into()).unwrap();

            assert_eq!(scope.get_item(&Ident::from("n")).unwrap(), 3.into());
            assert_eq!(scope.get_item(&Ident::from("m")).unwrap(), 7.into());
            assert_eq!(scope.get_item(&Ident::from("t")).unwrap(), 14.into());
        }

        let json = create_json();
        check_json(&json);
    }
}

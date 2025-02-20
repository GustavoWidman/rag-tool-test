use rig::{
    completion::ToolDefinition,
    tool::Tool,
    vector_store::{VectorStoreError, VectorStoreIndexDyn},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct Args {
    lookup: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Lookup error")]
pub struct LookupError;

#[derive(Serialize)]
pub struct Lookup {
    #[serde(skip)]
    index: Box<dyn VectorStoreIndexDyn + Send + Sync>,
}

impl Lookup {
    pub fn new(index: impl VectorStoreIndexDyn + 'static) -> Self {
        Self {
            index: Box::new(index),
        }
    }

    fn search(&self, lookup: &str) -> Result<(f64, String, Value), VectorStoreError> {
        Ok(futures::executor::block_on(self.index.top_n(lookup, 1))?
            .into_iter()
            .next()
            .ok_or(VectorStoreError::MissingIdError(lookup.to_string()))?)
    }
}

impl Tool for Lookup {
    const NAME: &'static str = "lookup";

    type Error = LookupError;
    type Args = Args;
    type Output = Value;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "lookup",
            "description": "Looks up real and fictional concepts and returns the result, which may contain it's definition and possibly other information related to it.",
            "parameters": {
                "type": "object",
                "properties": {
                    "lookup": {
                        "type": "string",
                        "description": "The query to lookup"
                    },
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("[lookup] looking up \"{}\"", args.lookup);
        let result = self.search(&args.lookup).map_err(|_| LookupError)?.2;
        println!(
            "[lookup] result: {:?}",
            serde_json::to_string_pretty(&result)
        );
        Ok(result)
    }
}

use rig::providers::gemini;
use rig::{Embed, providers::gemini::completion::gemini_api_types::GenerationConfig};

use anyhow::Result;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use serde::{Deserialize, Serialize};

mod tools;
mod utils;

#[derive(Embed, Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
struct WordDefinition {
    id: String,
    word: String,
    #[embed]
    definitions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let client = gemini::Client::from_env();

    let embedding_model = client.embedding_model_with_ndims("text-embedding-004", 768);

    let embeddings = utils::embed(embedding_model.clone(),
        vec![
            WordDefinition {
                id: "doc0".to_string(),
                word: "flurbo".to_string(),
                definitions: vec![
                    "1. *flurbo* (name): A flurbo is a green alien that lives on cold planets.".to_string(),
                    "2. *flurbo* (name): A fictional digital currency that originated in the animated series Rick and Morty. Each flurbo is worth 10 USD, and you can have and/or give away a fraction of a flurbo (0.3 flurbos, for example).".to_string()
                ]
            },
            WordDefinition {
                id: "doc1".to_string(),
                word: "glarb-glarb".to_string(),
                definitions: vec![
                    "1. *glarb-glarb* (noun): A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.".to_string(),
                    "2. *glarb-glarb* (noun): A fictional creature found in the distant, swampy marshlands of the planet Glibbo in the Andromeda galaxy.".to_string()
                ]
            },
            WordDefinition {
                id: "doc2".to_string(),
                word: "linglingdong".to_string(),
                definitions: vec![
                    "1. *linglingdong* (noun): A term used by inhabitants of the far side of the moon to describe humans.".to_string(),
                    "2. *linglingdong* (noun): A rare, mystical instrument crafted by the ancient monks of the Nebulon Mountain Ranges on the planet Quarm.".to_string()
                ]
            },
        ])
        .await?;

    let vector_store = utils::VectorStore::new(
        InMemoryVectorStore::from_documents(embeddings),
        embedding_model.clone(),
    );

    let calculator_rag = client
        .agent("gemini-2.0-flash")
        .preamble("You are a helpful assistant. All algebraic operations must use the tools at your disposal. The \"lookup\" tool can not only be used to look up the definition of a word, but also to find any and all information regarding that word or concept. Use the \"lookup\" tool thoroughly to ensure you get the most accurate and relevant information. However, if you believe the information you are looking for is already in your context, do not use the \"lookup\" tool.")
        .tool(tools::Add)
        .tool(tools::Subtract)
        .tool(tools::Multiply)
        .tool(tools::Divide)
        .tool(tools::Lookup::new(vector_store.clone().index()))
        .dynamic_context(1, vector_store.index())
        .additional_params(serde_json::to_value(GenerationConfig {
            temperature: Some(0.0),
            ..Default::default()
        })?)
        .build();

    let mut agent = utils::MultiTurnAgent::new(calculator_rag);

    let query = "Calculate 5 - 2 = ?. Describe the result to me.";
    println!("Query #1: {}\n", query);
    let result = agent.multi_turn_prompt(query).await?;
    println!("\nResponse #1: {}\n\n", result);
    agent.clear_history().await;

    let query = "Calculate (2 + 3) / 10  = ?. Describe the result to me.";
    println!("Query #2: {}\n", query);
    let result = agent.multi_turn_prompt(query).await?;
    println!("\nResponse #2: {}\n\n", result);
    agent.clear_history().await;

    let query = "What does \"glarb-glarb\" mean?";
    println!("Query #3: {}\n", query);
    let result = agent.multi_turn_prompt(query).await?;
    println!("\nResponse #3: {}\n\n", result);
    agent.clear_history().await;

    // Then, once it has the definition, ask it to calculate the cost of a flurbo
    let query = "Somebody gave me two flurbos yesterday, and i already had 12 before that, but then, I had to give 10% of it to the government this afternoon, how many flurbos do i have left? And how many USD would I have if I converted what I have right now?";
    println!("Query #4: {}\n", query);
    let result = agent.multi_turn_prompt(query).await?;
    println!("\nResponse #4: {}", result);
    agent.clear_history().await;

    Ok(())
}

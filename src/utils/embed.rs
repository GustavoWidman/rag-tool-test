use anyhow::bail;
use rig::{
    Embed, OneOrMany,
    embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder},
};

/// because gemini only supports embedding one document at a time,
/// we have to use this util function with it, or else things start to break D:
pub async fn embed<T, M>(
    model: M,
    documents: Vec<T>,
) -> anyhow::Result<Vec<(T, OneOrMany<Embedding>)>>
where
    T: Embed + Send + Sync,
    M: EmbeddingModel,
{
    let mut embeddings = Vec::new();
    let documents_len = documents.len();

    for document in documents {
        let embedding = EmbeddingsBuilder::new(model.clone())
            .document(document)?
            .build()
            .await?
            .into_iter()
            .next();

        if let Some(embedding) = embedding {
            embeddings.push(embedding);
        }
    }

    if embeddings.len() != documents_len {
        bail!(
            "Document count mismatch, got {} documents but {} embeddings",
            documents_len,
            embeddings.len()
        );
    }

    Ok(embeddings)
}

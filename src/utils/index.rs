use rig::{
    embeddings::EmbeddingModel,
    vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore},
};
use serde::Serialize;

#[derive(Clone)]
pub struct VectorStore<D: Serialize + Clone, M: EmbeddingModel> {
    vector_store: InMemoryVectorStore<D>,
    model: M,
}

impl<D: Serialize + Clone, M: EmbeddingModel> VectorStore<D, M> {
    pub fn new(vector_store: InMemoryVectorStore<D>, model: M) -> Self {
        Self {
            vector_store,
            model,
        }
    }

    pub fn index(self) -> InMemoryVectorIndex<M, D> {
        self.vector_store.index(self.model)
    }
}

//! Model provider abstractions and concrete implementations.

pub mod mock;
pub mod openai;
pub mod provider;

pub use mock::MockProvider;
pub use openai::OpenAIProvider;
pub use provider::{ChatRequest, ChatResponse, ModelProvider, ProviderError};

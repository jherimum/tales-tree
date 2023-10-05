use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForkFragmentRequest {
    pub content: String,
    pub end: bool,
}

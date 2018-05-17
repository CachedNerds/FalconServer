#[get("/health")]
pub fn health() -> &'static str {
    "healthy"
}

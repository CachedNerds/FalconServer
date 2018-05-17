#[get("/ready")]
pub fn ready() -> &'static str {
    "ready"
}

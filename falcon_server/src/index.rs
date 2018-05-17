#[get("/")]
pub fn index() -> &'static str {
    "Falcon Server"
}

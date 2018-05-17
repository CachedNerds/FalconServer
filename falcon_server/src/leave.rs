#[post("/leave/<id>")]
pub fn leave(id: u32) -> String {
    format!("{}", id)
}

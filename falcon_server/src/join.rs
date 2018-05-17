#[post("/join/<id>")]
pub fn join(id: u32) -> String {
    format!("{}", id)
}

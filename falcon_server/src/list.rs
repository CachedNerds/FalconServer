use persist::read_list;

#[get("/list")]
pub fn list() -> String {
    let list = read_list();
    list
}

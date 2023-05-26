#[get("/")]
pub(crate) fn index() -> &'static str {
    "Hello, world!"
}

#[get("/manage")]
pub(crate) fn manage() -> &'static str {
    "todo"
}

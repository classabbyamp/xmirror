#[get("/mirrors")]
pub(crate) fn mirrors() -> &'static str {
    "todo"
}

#[get("/mirrors/<name>")]
pub(crate) fn get_mirror(name: &str) -> &'static str {
    "todo"
}

#[get("/regions")]
pub(crate) fn regions() -> &'static str {
    "todo"
}

#[get("/regions/<region>")]
pub(crate) fn get_region(region: &str) -> &'static str {
    "todo"
}

#[get("/tiers")]
pub(crate) fn tiers() -> &'static str {
    "todo"
}

#[get("/tiers/<tier>")]
pub(crate) fn get_tier(tier: &str) -> &'static str {
    "todo"
}

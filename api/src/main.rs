#[macro_use] extern crate rocket;

mod api;
mod legacy;
mod web;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![web::index, web::manage])
                   .mount("/raw", routes![legacy::tsv_list])
                   .mount("/api/v1", routes![
                        api::mirrors, api::get_mirror,
                        api::regions, api::get_region,
                        api::tiers, api::get_tier,
                    ])
}

/*

/ -> homepage
/manage -> management interface
/login -> login via oauth

/raw/mirrors.lst -> legacy endpoint

/api/v1/regions -> regions list
/api/v1/regions/<region> -> region info and mirror list in region
/api/v1/tiers -> tier list
/api/v1/tiers/<tier> -> tier info and mirror list in tier
/api/v1/mirrors -> mirrors list
/api/v1/mirrors/<name> -> get mirror info

 */

#[macro_use] extern crate rocket;

#[get("/")]
fn about() -> &'static str {
    "about page"
}

#[get("/test")]
fn test() -> &'static str {
    "test page"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![about, test])
}
#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::tokio::time::{sleep, Duration};
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/")]
async fn about() -> &'static str {
    "about page"
}

#[get("/test/<number>")]
async fn test(number: u32) -> &'static str {
    if &number > &5 {
        "the page number is more than 5"
    }
    else {
        "the page number is less than 5"
    }
}

#[get("/admin/<path..>")]
async fn admin(path: PathBuf) -> String {
    format!("no permission")
}

// #[catch(403)]
// fn not_found(req: &Request) -> String {
//     format!("Sorry, {} no permission", req.uri())
// }


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![about, test, admin])
    // rocket::build().register("/", catchers![not_found])
}
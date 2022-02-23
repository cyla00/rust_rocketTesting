#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

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

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "api_key_value" // HERE IS THE API KEY VALUE
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
        }
    }
}

#[get("/admin")]
async fn admin(key: ApiKey<'_>) -> String {
    format!("admin page")
}

#[get("/<_..>")]
async fn esilio() -> String {
    format!("no exist")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![about, test, admin, esilio])
}
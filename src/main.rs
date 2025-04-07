#[macro_use]
extern crate rocket;

use rocket::http::Cookie;
use rocket::http::Status;
use rocket::http::CookieJar;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;

#[derive(Debug)]
struct User {
    user_id: String,
    username: String,
}

#[derive(Debug)]
enum AuthError {
    MissingCookie,
    InvalidCookie,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let user_cookie = cookies.get("user_id");

        match user_cookie {
            Some(cookie) => {
                if cookie.value() == "abc-xyz-123" {
                    Outcome::Success(User {
                        user_id: "abc-xyz-123".to_string(),
                        username: "rando".to_string(),
                    })
                } else {
                    Outcome::Error((Status::Unauthorized, AuthError::InvalidCookie))
                }
            }

            None => Outcome::Error((Status::Unauthorized, AuthError::MissingCookie))
        }
    }
}


#[get("/")]
fn index(user: User) -> String {
    format!("Hello, {}! You're logged in with ID: {}", user.username, user.user_id)
}

#[get("/", rank = 2)]
fn index_guest() -> &'static str {
    "Hello, guest! Please log in"
}

#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> Redirect {
    cookies.add(Cookie::new("user_id", "abc-xyz-123"));
    Redirect::to("/")
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove("user_id");
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, index_guest, login, logout])
}

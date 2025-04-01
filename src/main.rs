#[macro_use] extern crate rocket;


#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/earth")]
fn earth() -> &'static str {
    "Hello, earth!"
}

#[get("/")]
fn index() -> &'static str {
    "This is the start!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![index, world, earth])
        .mount("/", routes![hello])
}

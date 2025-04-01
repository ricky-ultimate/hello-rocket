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

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {}. You claim to be {}, but is that really your age 🤔", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![index, world, earth])
}

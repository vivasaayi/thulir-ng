#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Hello, world! Ping Example"
}

#[get("/health")]
fn health() -> &'static str {
    "Hello, world! Health Example"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,ping,health])
}

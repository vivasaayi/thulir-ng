mod news;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    let a = "one";
    let b = "two";
    println!("{}-{}-{}", "Hello, world! Ping Example", a, b);
    let n1:news::News = news::all_news();
    return ""
}

#[get("/health")]
fn health() -> &'static str {
    return "Hello, world! Health Example"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,ping,health])
}

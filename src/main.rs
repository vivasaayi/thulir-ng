mod news;

use rocket::fs::FileServer;
use rocket::serde::{Serialize, json::Json};

#[macro_use] extern crate rocket;

#[get("/ping")]
async fn ping() -> Json<Vec<news::WordPressPost>> {
    let a = "one";
    let b = "two";
    println!("{}-{}-{}", "Hello, world! Ping Example", a, b);
    let n1:Vec<news::WordPressPost> = news::all_news().await.unwrap();
    Json(n1)
}

#[get("/health")]
fn health() -> &'static str {
    return "Hello, world! Health Example"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("web/build/"))
        .mount("/api", routes![ping,health])
}

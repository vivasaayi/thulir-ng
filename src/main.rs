mod news;

use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
async fn ping() -> &'static str {
    let a = "one";
    let b = "two";
    println!("{}-{}-{}", "Hello, world! Ping Example", a, b);
    let n1:Vec<news::WordPressPost> = news::all_news().await.unwrap();
    let str_data =serde_json::to_string(&n1).unwrap();
    str_data.leak()
}

#[get("/health")]
fn health() -> &'static str {
    return "Hello, world! Health Example"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("web/build/"))
        .mount("/api", routes![index,ping,health])
}

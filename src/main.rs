mod news;
mod wordpress;
extern crate markup5ever_rcdom as rcdom;

mod htmlparser;
use rcdom::{RcDom, Handle, SerializableHandle};

use rocket::fs::FileServer;
use rocket::serde::{Serialize, json::Json};
use markup5ever::interface::TreeSink;

#[macro_use] extern crate rocket;

#[get("/ping")]
async fn ping() -> &'static str {
    return "Ping!"
}

#[get("/health")]
fn health() -> &'static str {
    return "I am alive!"
}

#[get("/posts")]
async fn posts() -> Json<Vec<crate::wordpress::WordPressPost>> {
    let n1:Vec<crate::wordpress::WordPressPost> = news::all_news().await.unwrap();
    Json(n1)
}

#[get("/post/<post_id>")]
async fn post(post_id: String) -> Json<crate::wordpress::WordPressPost> {
    let post:crate::wordpress::WordPressPost = news::get_post_by_id(post_id).await;

    let dom:RcDom = htmlparser::parse_html(&post.title.rendered);


    Json(post)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("web/build/"))
        .mount("/api", routes![ping,health, post, posts])
}

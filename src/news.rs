use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub struct News {
    pub short_description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Rendered {
    rendered: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordPressPost {
    id: i32,
    date: String,
    date_gmt: String,
    guid: Rendered,
    modified: String,
    modified_gmt: String,
    slug: String,
    status: String,
    // type: String,
    link: String,
    title: Rendered,
    content: Rendered,
    excerpt: Rendered,
    author: i32,
    tags: Vec<String>,

}

async fn test_req() -> core::result::Result<Vec<WordPressPost>, &'static str > {
    println!("Making HTTP Request");
    let resp = reqwest::get("https://rajanpanneerselvam.com/wp-json/wp/v2/posts")
        .await.unwrap();

    let response_text = resp.text().await.unwrap();
    println!("{:#?}", response_text);

    let parsed_response: Vec<WordPressPost> = serde_json::from_str(&response_text)
        .unwrap();

    println!("Response Received");
    println!("{:#?}", parsed_response);

    return Result::Ok(parsed_response);
}

pub async fn all_news() -> core::result::Result<Vec<WordPressPost>, &'static str >{
    let posts = test_req().await.unwrap();
    println!("{:#?}", posts);

    return Result::Ok(posts);

    // let n1:News = News { short_description: String::from("AA")};
    // println!("all news");
    // return n1;
}
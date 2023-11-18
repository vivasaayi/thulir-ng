use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rendered {
    rendered: String
}

fn get_blog_base_url() -> String {
    let blog_url = "https://rajanpanneerselvam.com";

    return blog_url.to_string();
}

async fn fetch_all_posts() {

}

pub async fn fetch_post_by_id(post_id: String) -> core::result::Result<crate::wordpress::WordPressPost, &'static str >{
    let base_url = get_blog_base_url();
    let url = format!("{base_url}/wp-json/wp/v2/posts/{post_id}");

    println!("{}", url);

    let resp = reqwest::get(url)
        .await.unwrap();

    let response_text = resp.text().await.unwrap();

    let parsed_response: crate::wordpress::WordPressPost = serde_json::from_str(&response_text)
        .unwrap();


    return Result::Ok(parsed_response);
}
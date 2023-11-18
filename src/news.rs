pub struct News {
    pub short_description: String
}

async fn test_req() -> core::result::Result<Vec<crate::wordpress::WordPressPost>, &'static str > {
    println!("Making HTTP Request");
    let resp = reqwest::get("https://rajanpanneerselvam.com/wp-json/wp/v2/posts")
        .await.unwrap();

    let response_text = resp.text().await.unwrap();
    println!("{:#?}", response_text);

    let parsed_response: Vec<crate::wordpress::WordPressPost> = serde_json::from_str(&response_text)
        .unwrap();

    println!("Response Received");
    println!("{:#?}", parsed_response);

    return Result::Ok(parsed_response);
}

pub async fn get_post_by_id(post_id: String) -> core::result::Result<crate::wordpress::WordPressPost, &'static str >{
    let post = crate::wordpress::fetch_post_by_id(post_id);

    return post.await;
}

pub async fn all_news() -> core::result::Result<Vec<crate::wordpress::WordPressPost>, &'static str >{
    let posts = test_req().await.unwrap();
    println!("{:#?}", posts);

    return Result::Ok(posts);

    // let n1:News = News { short_description: String::from("AA")};
    // println!("all news");
    // return n1;
}
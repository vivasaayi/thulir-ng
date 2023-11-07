use std::collections::HashMap;

pub struct News {
    pub short_description: String
}

async fn test_req() -> Result<i32, &'static str > {
    println!("Making HTTP Request");
    let resp = reqwest::get("https://rajanpanneerselvam.com/wp-json/wp/v2/posts")
        .await.unwrap();

    // let response_text = resp.text().await.unwrap();
    // println!("{:#?}", response_text);

    let parsed_response = resp.json::<serde_json::Value>()
        .await.unwrap();


    println!("Response Received");
    println!("{:#?}", parsed_response);

    return Result::Ok(1);
}

pub async fn all_news() -> News{
    let result = test_req().await.unwrap();
    println!("{:#?}", result);

    let n1:News = News { short_description: String::from("AA")};
    println!("all news");
    return n1;
}
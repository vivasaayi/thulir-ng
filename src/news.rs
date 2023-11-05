
pub struct News {
    pub short_description: String
}

pub fn all_news() -> News{
    let n1:News = News { short_description: String::from("AA")};
    println!("all news");
    return n1;
}
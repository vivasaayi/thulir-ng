7  |   async fn test_req() {
|  _____________________-
8  | |     let resp = reqwest::get("https://httpbin.org/ip")
9  | |         .await?
| |               ^ cannot use the `?` operator in an async function that returns `()`
10 | |         .json::<HashMap<String, String>>()
...  |
13 | |     println!("{:#?}", resp);
14 | | }
| |_- this function should return `Result` or `Option` to accept `?`


------------
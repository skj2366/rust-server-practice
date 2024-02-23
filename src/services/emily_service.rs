pub fn connect_check() -> String {
    let s: String = String::from("connect complete emily_service");
    s
}

pub async fn test_emily() {
    let response = reqwest::get("https://127.0.0.1:4430/api/v1/emily/test-get").await;

    match response {
        Ok(body) => {
            let data = body.text().await;
            println!("{:?}", data);
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}

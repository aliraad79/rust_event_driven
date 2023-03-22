use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..10 {
        let data = json!({
                "title": "test",
                "description": "Descriptions is long",
        })
        .to_string();

        let res = reqwest::Client::new()
            .post("http://127.0.0.1:8000/task")
            .body(data)
            .send()
            .await
            .unwrap();

        println!("{:#?}", res);
    }
    Ok(())
}

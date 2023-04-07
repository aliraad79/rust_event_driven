use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..500 {
        let task = Task {
            title: String::from("test"),
            description: String::from("Descriptions is long"),
        };

        let res = reqwest::Client::new()
            .post("http://127.0.0.1:8080/task")
            .json(&task)
            .send()
            .await;
        match res {
            Ok(ok_res) => println!("{:#?}", ok_res),
            Err(_err) => println!("Err on request to server"),
        }
    }
    Ok(())
}

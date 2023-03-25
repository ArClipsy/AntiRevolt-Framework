use reqwest::Client;
use serde_json::Value;
use tokio::task::spawn;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut tasks = vec![];

    let token = "session token";
    let users_added = ["users here", "for example", "add as many", "as youd like"];
    let group_chat_name = "gc name here";


    for _ in 0..1000 {

        let token = token.to_owned();
        let client = Client::new();
        let users = users_added.to_owned();
        let gc_name = group_chat_name.to_owned();

        let task = spawn(async move {
            for j in 0..2 {
                let response_status = client.post("https://api.revolt.chat/channels/create")
                    .json(&serde_json::json!({
                        "name": gc_name,
                        "users": users
                    }))
                    .header("x-session-token", &token)
                    .send()
                    .await?;

                if response_status.status().is_success() {
                    println!("Made a groupchat! :: {}", response_status.status())
                } else {
                    let retry_after = response_status
                        .json::<Value>()
                        .await?
                        .get("retry_after")
                        .unwrap_or(&serde_json::Value::Null)
                        .as_u64()
                        .unwrap_or(0);
            
                    std::thread::sleep(std::time::Duration::from_millis(retry_after));
                }
            }

            Ok::<(), reqwest::Error>(())
        });

        tasks.push(task);
    }

    join_all(tasks).await;
    

    Ok(())
}

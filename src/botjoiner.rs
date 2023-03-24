use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let host = Client::new();

    let bot_response_ids = host
        .get("https://rvlt.gg/_next/data/QlVaRnSbNl2oUZ08PZJhF/discover/bots.json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let bots_array = bot_response_ids["pageProps"]["bots"].as_array().unwrap();
    let bot_array_length = bots_array.len();
    let token = "session token here";
    let sid = "server id here";

    for i in 0..bot_array_length {
        let bot_id = bots_array[i]["_id"].as_str().unwrap();
        let response_status = host
            .post(format!("https://api.revolt.chat/bots/{}/invite", bot_id))
            .header(
                "x-session-token",
                token,
            )
            .json(&serde_json::json!({"server":sid}))
            .send()
            .await?;

        if response_status.status().is_success() {
            println!("Added A Bot! :: {}", response_status.status())
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

    Ok(())
}

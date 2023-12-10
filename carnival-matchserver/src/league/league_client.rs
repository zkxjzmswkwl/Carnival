use shaco::rest::RESTClient as Shaco;

pub async fn change_status(lcu: &mut Shaco) {
    lcu.put("/lol-chat/v1/me".to_string(),
        serde_json::json!({
            "statusMessage": "Test status update jaiosdjasoidjasoid"
        })
    ).await;
}

pub async fn get_conversations(lcu: &mut Shaco) -> Option<serde_json::Value> {
     match lcu.get("/lol-chat/v1/conversations".to_string()).await {
        Ok(conversations) => Some(conversations),
        Err(e) => {
            eprintln!("{e}");
            None
        }
     }
}
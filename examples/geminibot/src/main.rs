use russenger::dotenv;
use russenger::prelude::*;
use serde::Deserialize;
use serde::Serialize;

const URL: &str =
    "https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent?key=";

#[derive(Serialize, Deserialize, Clone)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Candidate {
    content: Content,
}

#[derive(Serialize, Deserialize, Clone)]
struct Body {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Response {
    candidates: Vec<Candidate>,
}

async fn ask_gemini(text: String) -> Result<Response, reqwest::Error> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("pls check your env file");
    let api_url = format!("{URL}{api_key}");
    let body = Body {
        contents: vec![Content {
            role: "user".to_owned(),
            parts: vec![Part { text }],
        }],
    };
    let response = reqwest::Client::new()
        .post(api_url)
        .json(&body)
        .send()
        .await?;

    match response.json().await {
        Ok(response) => Ok(response),
        Err(err) => panic!("{err:?}"),
    }
}

create_action!(Main, |res: Res, req: Req| async move {
    res.send(GetStartedModel::new(Payload::default())).await;
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "AskGemini".to_owned(),
            payload: Payload::new(HelloWorld, None),
        }],
    ))
    .await;
});

create_action!(HelloWorld, |res: Res, req: Req| async move {
    let text = "Hello, I'm Gemini";
    res.send(TextModel::new(&req.user, text)).await;
    req.query.set_action(&req.user, AskGemini).await;
});

create_action!(AskGemini, |res: Res, req: Req| async move {
    let text: String = req.data.get_value();
    match ask_gemini(text).await {
        Ok(response) => {
            for part in response.candidates[0].content.parts.clone() {
                res.send(TextModel::new(&req.user, &part.text)).await;
            }
        }
        Err(err) => {
            res.send(TextModel::new(&req.user, &err.to_string())).await;
        }
    };
});

russenger_app!(Main, HelloWorld, AskGemini);

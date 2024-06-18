use russenger::dotenv;
use russenger::models::RussengerUser;
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

#[action]
async fn Main(res: Res, req: Req) {
    res.send(GetStartedModel::new(Payload::default())).await;
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "AskGemini".into(),
            payload: Payload::new(HelloWorld, None),
        }],
    ))
    .await;
}

#[action]
async fn HelloWorld(res: Res, req: Req) {
    let text = "Hello, I'm Gemini";
    res.send(TextModel::new(&req.user, text)).await;
    req.query.set_action(&req.user, AskGemini).await;
}

#[action]
async fn AskGemini(res: Res, req: Req) {
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
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser], &conn);
    russenger::actions![Main, HelloWorld, AskGemini];
    russenger::launch().await;
}


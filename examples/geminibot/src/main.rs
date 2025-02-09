use russenger::prelude::*;

mod gemini {
    use serde::Deserialize;
    use serde::Serialize;
    const URL: &str =
        "https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent?key=";

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Part {
        pub text: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Content {
        pub role: String,
        pub parts: Vec<Part>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Candidate {
        pub content: Content,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Body {
        pub contents: Vec<Content>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Response {
        pub candidates: Vec<Candidate>,
    }

    pub async fn ask_gemini(text: String) -> Result<Response, reqwest::Error> {
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
}

async fn index(res: Res, req: Req) -> Result<()> {
    res.send(GetStartedButtonModel::new(Payload::default()))
        .await?;
    res.send(PersistentMenuModel::new(
        &req.user,
        [Button::Postback {
            title: "AskGemini".into(),
            payload: Payload::new("/hello_world", None),
        }],
    ))
    .await?;

    Ok(())
}

async fn hello_world(res: Res, req: Req) -> Result<()> {
    let text = "Hello, I'm Gemini";
    res.send(TextModel::new(&req.user, text)).await?;
    res.redirect("/ask_gemini").await?;

    Ok(())
}

async fn ask_gemini(res: Res, req: Req) -> Result<()> {
    let text: String = req.data.get_value();
    match gemini::ask_gemini(text).await {
        Ok(response) => {
            for part in response.candidates[0].content.parts.clone() {
                res.send(TextModel::new(&req.user, &part.text)).await?;
            }
        }
        Err(err) => {
            res.send(TextModel::new(&req.user, &err.to_string()))
                .await?;
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    App::init()
        .await?
        .attach(
            Router::new()
                .add("/", index)
                .add("/hello_world", hello_world)
                .add("/ask_gemini", ask_gemini),
        )
        .launch()
        .await?;

    Ok(())
}

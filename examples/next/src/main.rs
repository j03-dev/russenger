use const_data::DATA;
use russenger::{models::RussengerUser, prelude::*};

mod const_data;

#[action]
async fn index(res: Res, req: Req) -> Result<()> {
    let mut elements = Vec::new();

    for (i, d) in DATA.iter().enumerate() {
        let element = GenericElement::new(
            d.title,    // Title
            d.subtitle, // Subtitle
            d.image,    // Image URL
            vec![
                // Buttons
                Button::WebUrl {
                    title: format!("Button {}", i),
                    url: d.image.to_string(),
                },
            ],
        );
        elements.push(element);
    }

    let generic = GenericModel::new(&req.user, elements, req.data.get_page());
    res.send(generic).await?; // Send only 10 elements
    res.send(NextModel(&req.user, req.data, "/")).await?; // send next 10 elements
    Ok(())
}

#[russenger::main]
async fn main() -> Result<()> {
    let conn = Database::new().await?.conn;
    migrate!([RussengerUser], &conn);
    let mut app = App::init().await?;
    app.add("/", index).await;
    launch(app).await.ok();
    Ok(())
}

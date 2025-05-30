use const_data::DATAS;
use russenger::prelude::*;

mod const_data;

async fn index(res: Res, req: Req) -> Result<()> {
    let mut elements = Vec::new();

    for (i, d) in DATAS.iter().enumerate() {
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

#[tokio::main]
async fn main() -> Result<()> {
    App::init()
        .await?
        .attach(Router::new().add("/", index))
        .launch()
        .await?;
    Ok(())
}

use const_data::DATA;
use russenger::{models::RussengerUser, prelude::*};

mod const_data;

#[action]
async fn Main(res: Res, req: Req) {
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
    res.send(generic).await; // Send only 10 element
    Main.next(res, req).await; // Send next 10 element
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser], &conn);
    
    russenger::actions![Main];
    russenger::launch().await;
}

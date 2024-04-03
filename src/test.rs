use dotenv::dotenv;

use crate::core::request::Req;
use crate::core::response::Res;
use crate::Action;

use crate::query::Query;

static USER_TEST: &str = "TEST";

struct TestAction;

#[crate::async_trait]
impl Action for TestAction {
    fn path(&self) -> String {
        "TestAction".into()
    }

    async fn execute(&self, _: Res, _: Req) {}
}

#[rocket::async_test]
async fn test_migrate() {
    dotenv().ok();
    let query = Query::new().await;
    let result = query.migrate().await;
    assert!(result);
}

#[rocket::async_test]
async fn test_create_user() {
    dotenv().ok();
    let query = Query::new().await;
    let result = query.create(USER_TEST).await;
    assert!(result);
}

#[rocket::async_test]
async fn test_set_action() {
    dotenv().ok();
    let query = Query::new().await;
    let result = query.set_action(USER_TEST, TestAction).await;
    assert!(result);
}

#[rocket::async_test]
async fn get_action() {
    dotenv().ok();
    let query = Query::new().await;
    let result = query.get_action(USER_TEST).await;
    assert_eq!(Some("TestAction".to_string()), result);
}

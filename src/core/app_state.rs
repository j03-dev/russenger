use crate::models::User;

pub struct AppState {
	pub user_conn: User,
}

impl AppState {
	pub async fn init() -> Self {
		let user_conn: User = User::new().await;
		Self {user_conn }
	}
}
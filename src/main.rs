pub mod db;

#[tokio::main]
async fn main() {
    let client = db::PrismaClient::new().await;

    // Required values are passed in as separate arguments
    let user = client
        .user()
        .create_one(
            db::User::username().set("user0".to_string()),
            db::User::display_name().set("User 0".to_string()),
            // Optional arguments can be added in a Vec as the last parameter
            vec![],
        )
        .exec()
        .await;
}

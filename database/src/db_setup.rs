use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn database_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    let db: DatabaseConnection = Database::connect(&database_url).await?;
    Ok(db)
}

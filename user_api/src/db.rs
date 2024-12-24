use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini dans .env");
    Database::connect(database_url).await.expect("Erreur de connexion à la base de données")
}

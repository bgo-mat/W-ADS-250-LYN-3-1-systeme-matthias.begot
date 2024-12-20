use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url ="mysql://root:Lavieestbelle!44@db:3306/user_db";
    Database::connect(database_url).await.expect("Erreur de connexion à la base de données")
}

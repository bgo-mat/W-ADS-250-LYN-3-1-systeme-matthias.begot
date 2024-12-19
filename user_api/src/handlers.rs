use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, Set, IntoActiveModel, ModelTrait, ActiveModelTrait};
use crate::models::{Entity as User, ActiveModel};
use sea_orm::ActiveValue::NotSet;
use bcrypt::{hash, DEFAULT_COST};

#[derive(serde::Deserialize)]
pub struct CreateUserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn create_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    json: web::Json<CreateUserData>,
) -> impl Responder {
    let hashed_password = match hash(&json.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Erreur de hachage du mot de passe: {}", err)),
    };

    let new_user = ActiveModel {
        id: NotSet,
        name: Set(json.name.clone()),
        email: Set(json.email.clone()),
        password: Set(hashed_password),
    };

    match User::insert(new_user).exec(db.get_ref()).await {
        Ok(res) => {
            let inserted_user = User::find_by_id(res.last_insert_id).one(db.get_ref()).await.unwrap();
            HttpResponse::Ok().json(inserted_user)
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Erreur: {}", err)),
    }
}


pub async fn get_users(
    db: web::Data<sea_orm::DatabaseConnection>,
) -> impl Responder {
    let users = User::find().all(db.get_ref()).await.unwrap();
    HttpResponse::Ok().json(users)
}

pub async fn get_user_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user = User::find_by_id(user_id).one(db.get_ref()).await.unwrap();
    if let Some(u) = user {
        HttpResponse::Ok().json(u)
    } else {
        HttpResponse::NotFound().body("User non trouvé")
    }
}

pub async fn update_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
    json: web::Json<UpdateUserData>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user = User::find_by_id(user_id).one(db.get_ref()).await.unwrap();
    if let Some(mut u) = user.map(|u| u.into_active_model()) {
        if let Some(name) = &json.name {
            u.name = Set(name.clone());
        }
        if let Some(email) = &json.email {
            u.email = Set(email.clone());
        }
        if let Some(pw) = &json.password {
            let hashed_password = match hash(pw, DEFAULT_COST) {
                Ok(h) => h,
                Err(err) => return HttpResponse::InternalServerError().body(format!("Erreur de hachage du mot de passe: {}", err)),
            };
            u.password = Set(hashed_password);
        }

        let res = u.update(db.get_ref()).await;
        match res {
            Ok(updated_user) => HttpResponse::Ok().json(updated_user),
            Err(err) => HttpResponse::BadRequest().body(format!("Erreur: {}", err)),
        }
    } else {
        HttpResponse::NotFound().body("User non trouvé")
    }
}


pub async fn delete_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user = User::find_by_id(user_id).one(db.get_ref()).await.unwrap();
    if let Some(u) = user {
        let res = u.delete(db.get_ref()).await;
        match res {
            Ok(_) => HttpResponse::Ok().body("User supprimé"),
            Err(err) => HttpResponse::BadRequest().body(format!("Erreur: {}", err)),
        }
    } else {
        HttpResponse::NotFound().body("User non trouvé")
    }
}

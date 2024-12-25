use actix_web::web;
use crate::user::handlers as user_handlers;
use crate::open_ai_api::handlers as call_openai;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/user")
                .route("", web::post().to(user_handlers::create_user))
                .route("", web::get().to(user_handlers::get_users))
                .route("/{id}", web::get().to(user_handlers::get_user_by_id))
                .route("/{id}", web::put().to(user_handlers::update_user))
                .route("/{id}", web::delete().to(user_handlers::delete_user))
        )
        .service(
            web::resource("/openai")
                .route(web::post().to(call_openai::call_openai_handler))
        );
}

use super::handlers::*;
use actix_web::web;

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/courses")
    .route("/", web::post().to(post_new_course))
    .route("/{user_name}", web::get().to(get_courses)));
}
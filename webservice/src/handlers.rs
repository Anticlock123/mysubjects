use std::result;

use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::db_access::*;

use super::models::{Course,CreateCourse};
use super::errors::MyError;


pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse,MyError> { 
   // Err(MyError::DBError(String::from("error")))
    let var = post_new_course_db(&app_state.db, new_course.into())
    .await;
    //var.map(|course| HttpResponse::Ok().json(course));
    match var {
        Ok(_) => {var.map(|course| HttpResponse::Ok().json(course))},
        Err(_) => {Err(MyError::DBError(String::from("error")))}
    }
    //  post_new_course_db(&app_state.db, new_course.into())
    //  .await
    //  .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses(
    app_state: web::Data<AppState>,
    params: web::Path<(String,)>
) ->Result<HttpResponse,MyError> {
    let name = String::try_from(params.0.clone()).unwrap();
    get_courses_for_db(&app_state.db, name)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive( Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub score1: String,
    pub score2: String,
    pub score3: String,
    pub score4: String,
    pub score5: String,
    pub key: String,
    pub rank: i32,
}
#[derive(Deserialize,Debug, Clone)]
pub struct CreateCourse {
    pub name: String,
    pub score1: String,
    pub score2: String,
    pub score3: String,
    pub score4: String,
    pub score5: String,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            name: course.name.clone(),
            score1: course.score1.clone(),
            score2: course.score2.clone(),
            score3: course.score3.clone(),
            score4: course.score4.clone(),
            score5: course.score5.clone(),
           
        }
    }
}
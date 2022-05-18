use std::io::StdoutLock;

use super::models::*;
use sqlx::postgres::PgPool;
use super::errors::MyError;
use pedersencommit::*;


pub fn course_to_student(stu: &Course) -> test::Student {
    let mut student_one = test::Student{
        id: stu.id,
        name: stu.name.clone(),
        score1: stu.score1.parse::<u64>().unwrap(),
        score2: stu.score2.parse::<u64>().unwrap(),
        score3: stu.score3.parse::<u64>().unwrap(),
        score4: stu.score4.parse::<u64>().unwrap(),
        score5: stu.score5.parse::<u64>().unwrap(),
        key: stu.key.clone(),

    };
    student_one
}
pub  fn pederden_quick_sort( nums: &mut Vec<Course>, left: usize, right: usize){
    if left > right {
        return;
    }
    let mut l=left;
    let mut r=right;
    while l < r {
        while l<r && nums[r].score5<=nums[left].score5 {
            r -= 1;
        }
        while l<r && nums[l].score5 >= nums[left].score5 {
            l += 1;
        }
        let mut student_one = course_to_student(&nums[l]);
        let mut student_two = course_to_student(&nums[r]);
        let flag:bool = test::pedersen_test(student_one,student_two);
        println!("{}",flag);
        if flag == true{
        nums.swap(l,r);}
    }
    let mut student_one1 = course_to_student(&nums[left]);
    let mut student_two2 = course_to_student(&nums[l]);
    if test::pedersen_test(student_one1,student_two2) == true{
    nums.swap(left, l);}
    if l > 1 {
        pederden_quick_sort(nums,left, l-1);
    }
    pederden_quick_sort(nums,r+1,right);
}
pub async fn get_courses_for_db(pool: &PgPool,name: String) -> Result<Course, MyError>{
    // let rows = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1"#, teacher_id
    // )
    // .fetch_all(pool)
    // .await
    // .unwrap();
    let mut rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course"#
    )
    .fetch_all(pool)
    .await?;
    
    let mut students: Vec<Course> = rows.clone();
    let r = rows.len()-1;
    pederden_quick_sort(&mut students,0,r);
    println!("rows:{:?}",rows);
    
    // sqlx::query!(
    //     r#"TRUNCATE TABLE course"#
    // ).fetch_all(pool).await?;
    // sqlx::query!(
    //     r#"TRUNCATE course RESTART IDENTITY"#
    // ).fetch_all(pool).await?;
    
    let mut index = 0;
    println!("{}",students.len());
    for i in 0..students.len(){
        students[i].rank=i as i32 + 1;
        post_course_db(pool,&students[i]).await;
        if students[i].name == name{
            index = i;
        }
    }
    
    Ok(students[index].clone())
}
pub async fn post_course_db(pool: &PgPool, new_course: &Course)->Result<(),MyError>{
    sqlx::query_as!(
        Course,
        r#"UPDATE course SET rank=$1 WHERE NAME=$2"#,
        new_course.rank,
        new_course.name,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
    
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) ->Result<Course, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE name = $1 limit 1"#, new_course.name
        
    ).fetch_all(pool)
    .await?;
    if rows.len() != 0 {
        let pk = rows[0].key.clone();
        let stu = course_to_student(&rows[0]);
        let bad_val = new_course.score5.parse::<u64>().unwrap();
        let check = test::pedersen_power(&stu,pk,bad_val);
        if check == true {
            Ok(rows[0].clone())
        }else{
        Err(MyError::DBError(String::from("EXIST")))
        }
    }
    else{

    let key:String= test::get_rng_point();
    
    println!("{}",key);
    let pm = 0;
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (name, score1, score2, score3, score4, score5,key,rank) VALUES($1, $2,$3,$4,$5,$6,$7,$8) RETURNING  id,name,score1,score2,score3,score4,score5,key,rank"#,
        new_course.name,
        new_course.score1,
        new_course.score2,
        new_course.score3,
        new_course.score4,
        new_course.score5,
        key,
        pm,
    )
    .fetch_one(pool)
    .await?;
    println!("数据库：{}",rows.len() );
    println!("{:?}",row);
    
    Ok(row)
}

}
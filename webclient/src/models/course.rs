use super::super::errors::MyError;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Deserialize, Serialize)]
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
#[wasm_bindgen]
pub async fn get_courses(name: String) -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
   
    let url = format!("http://localhost:3000/courses/{}",name);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().ok_or("no window exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let course: Course = json.into_serde().unwrap();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no global document"); 

    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left div not exists");
    
    let c = &course;
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;

        let td= document.create_element("td")?;
        td.set_text_content(Some(format!("{}",c.rank).as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.score5.as_str()));
        tr.append_child(&td)?;

       // left_tbody.last_child();
        match  left_tbody.last_child() {
            Some(tt) => {left_tbody.remove_child(&tt);
                left_tbody.append_child(&tr);},
            None => {
            left_tbody.append_child(&tr)?;}
        }
      //  left_tbody.remove_child(&tt);
       // left_tbody.append_child(&tr)?;


    Ok(resp.json()?)

}
use js_sys::Promise;
use wasm_bindgen::prelude::*;
// use super::super::db_client::*;


#[wasm_bindgen(catch)]
pub async fn add_course(name: String, score1: String, score2: String,
                        score3: String, score4: String,
                    score5: String) -> Result<Promise, JsValue>{
    
    
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
 

    let str_json = format!(
        r#"
         {{
             "name": "{}",
             "score1": "{}",
             "score2": "{}",
             "score3": "{}",
             "score4": "{}",
             "score5": "{}"
         }}
         "#,
         name,score1,score2,score3,score4,score5
    );
    opts.body(Some(&JsValue::from_str(str_json.as_str())));
    let url = "http://localhost:3000/courses/";

    let request= Request::new_with_str_and_init(&url,&opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().ok_or("no wondow".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  //  Err(resp_value)
    
    assert!(resp_value.is_instance_of::<Response>());
    let respvalue= resp_value.clone();
    let resp: Response = resp_value.dyn_into().unwrap();
    let var= resp.status();
    match var {
        500 => {Err(respvalue)},
        _ => {

            Ok(resp.json()?)}
    }
   // Ok(resp.json()?)  
}
//}

// #[wasm_bindgen]
// pub async fn get_score() -> Result<(), JsValue> {
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     let url = "http://localhost:3000/courses/1";

//     let request = Request::new_with_str_and_init(&url, &opts)?;

//     request.headers().set("Accept", "application/json")?;

//     let window = web_sys::window().ok_or("no window exists".to_string())?;
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
//     assert!(resp_value.is_instance_of::<Response>());

//     let resp: Response = resp_value.dyn_into().unwrap();
//     let json = JsFuture::from(resp.json()?).await?;

//     let course: Vec<Course> = json.into_serde().unwrap();
    
//     Ok()

//    // Ok(course)
//     // let window = web_sys::window().expect("no global window");
//     // let document = window.document().expect("no global document");

//     // let left_tbody = document
//     //     .get_element_by_id("left-tbody")
//     //     .expect("left div not exists");

//     // let course: Vec<Course> = models::course::get_courses().await.unwrap();

//     // for c in course.iter() {
//     //     let tr = document.create_element("tr")?;
//     //     tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;

//     //     let td= document.create_element("td")?;
//     //     td.set_text_content(Some(format!("{}",c.id).as_str()));
//     //     tr.append_child(&td)?;

//     //     let td = document.create_element("td")?;
//     //     td.set_text_content(Some(c.name.as_str()));
//     //     tr.append_child(&td)?;

//     //     let td = document.create_element("td")?;
//     //     td.set_text_content(Some(c.score5.as_str()));
//     //     tr.append_child(&td)?;

//     //     // let td = document.create_element("td")?;
//     //     // td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
//     //     // tr.append_child(&td)?;

//     //     // let td=document.create_element("td")?;
//     //     // let btn = document.create_element("button")?;
//     //     // btn.set_attribute("class", "btn-danger btn-sm")?;
//     //     // btn.set_text_content

//     //     left_tbody.append_child(&tr)?;

//     // }
//     // Ok(())

    
// }

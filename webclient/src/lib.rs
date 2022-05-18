mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(format!("Hello, {}!",s).as_str());
}

pub mod errors;
pub mod models;


use models::course::Course;

// #[wasm_bindgen(start)]
// pub async fn main() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global window");
//     let document = window.document().expect("no global document");

//     let left_tbody = document
//         .get_element_by_id("left-tbody")
//         .expect("left div not exists");

//     let course: Vec<Course> = models::course::get_courses(String::from("NULL")).await.unwrap();

//     for c in course.iter() {
//         let tr = document.create_element("tr")?;
//         tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;

//         let td= document.create_element("td")?;
//         td.set_text_content(Some(format!("{}",c.id).as_str()));
//         tr.append_child(&td)?;

//         let td = document.create_element("td")?;
//         td.set_text_content(Some(c.name.as_str()));
//         tr.append_child(&td)?;

//         let td = document.create_element("td")?;
//         td.set_text_content(Some(c.score5.as_str()));
//         tr.append_child(&td)?;

//         // let td = document.create_element("td")?;
//         // td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
//         // tr.append_child(&td)?;

//         // let td=document.create_element("td")?;
//         // let btn = document.create_element("button")?;
//         // btn.set_attribute("class", "btn-danger btn-sm")?;
//         // btn.set_text_content

//         left_tbody.append_child(&tr)?;

//     }
//     Ok(())

    
// }

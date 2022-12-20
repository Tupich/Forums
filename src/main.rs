#[macro_use] extern crate rocket;
pub use rocket::response::status;
pub use rocket::serde::{Serialize,Serializer, Deserialize, json::Json};
pub use std::fs::File;
pub use rocket::Data;
use db::{Forum, write_forum, _forums, read_forum};
pub mod db;

#[get("/forum")]
fn forum() -> Json<Forum>{
	let dbz = _forums().unwrap();
	Json(dbz)
}

#[launch]
fn rocket() -> _ {	
    rocket::build().mount("/nigos", routes![forum, upload_forum])
}


#[post("/posting", data = "<input>")]
fn upload_forum(input: Json<Forum>){
	let new_forum = read_forum(input);
	write_forum(new_forum);
}









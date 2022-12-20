pub use rocket::serde::{Serialize,Serializer, Deserialize, json::Json};
pub use std::fs;

static DATABAZZE: &str = "db.json";

#[derive(Serialize, Deserialize)]
pub struct Forum{
    pub name: String,
    pub id: i64,
    pub descrip: String,
    pub rank: String,
}

pub fn _forums() -> Result<Forum, serde_json::Error>{
	let data = fs::read_to_string(DATABAZZE).unwrap();
	let forum: Result<Forum, serde_json::Error> = serde_json::from_str(&data);
	forum
}

pub fn write_forum(forum: Forum){
	let data = serde_json::to_string(&forum).unwrap();
	fs::write(DATABAZZE, data);
}

pub fn read_forum(var: Json<Forum>) -> Forum {
	var.into_inner()
}

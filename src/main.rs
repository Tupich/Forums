#[macro_use] extern crate rocket;
pub use rocket::serde::json::Json;

pub use serde::ser::{Serialize, SerializeStruct, Serializer};



#[get("/forum")]
fn forum() -> Json<Forum>{
    let huy = Forum{name:"Форум пидарасов".to_string(), id: 10, descrip: "МЫ ПИДОРЫ".to_string(), rank: "Легенда".to_string()};
    Json(huy)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![forum])
}


struct Forum{
    name: String,
    id: i32,
    descrip: String,
    rank: String,
}


impl Serialize for Forum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Forum", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("descrip", &self.descrip)?;
        s.serialize_field("rank", &self.rank)?;
        s.end()
    }
}



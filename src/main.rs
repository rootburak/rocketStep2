#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: usize,
    name: String,
    email: String,
}
#[get("/user/json/<id>")]
fn get_user_json(id: usize) -> Json<User> {
    let user = User {
        id,
        name: "Ahmet".to_string(),
        email: "ahmet@örnek.com".to_string(),
    };
    Json(user)
}

#[get("/")]
fn index() -> &'static str {
    "merhaba"
}

#[put("/user/<id>",data="<user>")]
fn update_user(id:usize,user:String) -> String {
    format!("{} ID'li kullanıcı güncellendi: {}", id, user)
}

#[launch]
fn rocket() -> _ {
rocket::build()
    .mount("/", routes![index,update_user,get_user_json])

}

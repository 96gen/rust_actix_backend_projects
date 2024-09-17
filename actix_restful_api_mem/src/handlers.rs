use actix_web::{web, Responder, HttpResponse};
use uuid::Uuid;
use crate::models::{User, UserDTO};
use crate::state::AppState;


//新增user的功能
pub async fn create_user(state: web::Data<AppState>, user: web::Json<UserDTO>) -> impl Responder {
    let new_user = User {
        id: Uuid::new_v4(), //產生一個新的UUID
        name: user.name.clone(),
        email: user.email.clone(),
    };
    //lock取得users的讀寫權，可以得到Result
    //unwrap從Result中取得值，如果Result是Error，會發生panic，讓程式終止
    //insert新增資料
    state.users.lock().unwrap().insert(new_user.id, new_user.clone());

    //回應剛剛新增的資料
    HttpResponse::Created().json(new_user)
}

//取得所有user的功能
pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    //values取得users的所有資料
    //cloned將users的值全都複製一份
    //collect將cloned的值全都轉換為Vec
    let users_list: Vec<User> = users.values().cloned().collect();

    //回應剛剛取得的資料
    HttpResponse::Ok().json(users_list)
}

//取得單一user的功能
pub async fn get_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let users = state.users.lock().unwrap();
    //let Some(user)，用來確認Option是否有值，如果有值就回傳Some，並將值傳給user，沒有就回傳None
    //users.get(&user_id)，根據user_id來取得對應的user，如果有值就回傳Option<&User>，沒有就回傳None
    if let Some(user) = users.get(&user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

//更新單一user的功能
pub async fn update_user(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    user_data: web::Json<UserDTO>,
) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&user_id) {
        user.name = user_data.name.clone();
        user.email = user_data.email.clone();
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

//刪除單一user的功能
pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    //remove刪除資料，並回傳Option
    //is_some判斷Option是否有值，有值就回傳true，沒有就回傳false
    if users.remove(&user_id).is_some() {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}
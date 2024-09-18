use serde::{Deserialize, Serialize};

//Serialize提供序列化功能，可以轉換為JSON、XML等格式
//Deserialize提供反序列化功能，可以從JSON、XML等格式轉換回來
//Clone允許使用.clone()複製
#[derive(Serialize, Deserialize, Clone)]
//建立Todo
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize,Deserialize)]
//接收前端傳來的資料
pub struct TodoDTO {
    pub title: String,
    pub completed: bool,
}
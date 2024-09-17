use serde::{Deserialize, Serialize};
use uuid::Uuid;

//賦予User特性
//Serialize提供序列化功能，可以將User轉換為JSON、XML等格式
//Deserialize提供反序列化功能，可以將JSON、XML等格式轉換為User
//Clone允許User可以使用.clone()複製
#[derive(Serialize, Deserialize, Clone)]
//建立User
pub struct User {
    pub id: Uuid, //使用UUID作為primary key
    pub name: String, //名稱
    pub email: String, //電子郵件
}

#[derive(Serialize,Deserialize)]
//建立UserDTO，用來接收前端傳來的資料
pub struct UserDTO {
    pub name: String,
    pub email: String,
}
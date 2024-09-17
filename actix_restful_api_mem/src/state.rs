use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::User;

//建立AppState，用來儲存User的資料
pub struct AppState {
    //Mutex允許多執行緒存取同一個資料，並且在同一時間只能有一個執行緒存取同一個資料
    pub users: Mutex<HashMap<Uuid, User>>,
}
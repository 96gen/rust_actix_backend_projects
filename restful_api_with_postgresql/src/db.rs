use deadpool_postgres::Pool;
use tokio_postgres::{NoTls};
use dotenv::dotenv;
use std::env;


//建立資料庫連接池
pub fn create_pool() -> Pool {
    //從.env讀取環境變數
    dotenv().ok();
    //使用.env中的DATABASE_URL變數，如果不存在就跳出錯誤
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //將db_url轉換成Config的資料型態
    let config = db_url.parse().unwrap();
    //建立連接管理器，NoTls表示不使用TLS
    let manager = deadpool_postgres::Manager::new(config, NoTls);
    //建立連接池
    deadpool_postgres::Pool::builder(manager)
        //最大連接數為16
        .max_size(16)
        .build()
        .unwrap()
}
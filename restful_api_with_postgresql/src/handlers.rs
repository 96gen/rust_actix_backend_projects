use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Client, Pool};
use tokio_postgres::Statement;

use crate::models::{Todo, TodoDTO};

async fn get_db_client(pool: &Pool) -> Result<Client, HttpResponse> {
    //從連接池取得一個資料庫連接
    pool.get().await
        //出錯時回傳500和訊息
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to get database connection"))
}

async fn prepare_sql(client: &Client, query: &str) -> Result<Statement, HttpResponse> {
    //準備SQL語句
    client.prepare(query).await.map_err(|_| HttpResponse::InternalServerError().body("Failed to prepare SQL statement"))
}

//新增todo
pub async fn add_todo(pool: web::Data<Pool>, todo: web::Json<TodoDTO>) -> impl Responder {
    //從連接池取得一個資料庫連接
    let client = get_db_client(&pool).await.unwrap();
    //準備SQL語句，用來新增資料並返回新增的記錄
    let sql = prepare_sql(&client, "INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING id, title, completed").await.unwrap();
    //執行SQL語句並取得返回的內容
    let row = client.query_one(&sql, &[&todo.title, &todo.completed]).await.unwrap();

    //將返回的記錄轉換為Todo
    let new_todo = Todo {
        id: row.get(0),
        title: row.get(1),
        completed: row.get(2),
    };

    //回傳新增的todo
    HttpResponse::Created().json(new_todo)
}

//取得所有todo
pub async fn get_todos(pool: web::Data<Pool>) -> impl Responder {
    //從連接池取得一個資料庫連接
    let client = get_db_client(&pool).await.unwrap();
    //準備SQL語句，從todos資料表中取得所有記錄的id、title和completed欄位
    let sql = prepare_sql(&client, "SELECT id, title, completed FROM todos").await.unwrap();
    //執行SQL語句並取得返回的內容
    let rows = client.query(&sql, &[]).await.unwrap();

    //將返回的多筆記錄轉換為Todo
    let todos: Vec<Todo> = rows.iter().map(|row| Todo {
        id: row.get(0),
        title: row.get(1),
        completed: row.get(2),
    }).collect();

    HttpResponse::Ok().json(todos)
}

//取得單一todo
pub async fn get_todo(pool: web::Data<Pool>, todo_id: web::Path<i64>) -> impl Responder {
    //從連接池取得一個資料庫連接
    let client = get_db_client(&pool).await.unwrap();
    //準備SQL語句，根據id從todos資料表中取得對應的記錄
    let sql = prepare_sql(&client, "SELECT id, title, completed FROM todos WHERE id = $1").await.unwrap();

    //執行SQL語句並取得返回的內容
    match client.query_one(&sql, &[&todo_id.into_inner()]).await {
        Ok(row) => {
            let todo = Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get(2),
            };
            HttpResponse::Ok().json(todo)
        },
        Err(_) => HttpResponse::NotFound().body("Todo not found"),
    }
}

//修改todo
pub async fn update_todo(pool: web::Data<Pool>, updated_todo: web::Json<TodoDTO>, todo_id: web::Path<i64>) -> impl Responder {
    //從連接池取得一個資料庫連接
    let client = get_db_client(&pool).await.unwrap();
    //準備SQL語句，根據id修改todos資料表中對應的記錄
    let sql = prepare_sql(&client, "UPDATE todos SET title = $1, completed = $2 WHERE id = $3").await.unwrap();
    let id = todo_id.into_inner();

    //執行SQL語句
    match client.execute(&sql, &[&updated_todo.title, &updated_todo.completed, &id]).await {
        Ok(_) => {
            //更新成功後，查詢更新後的todo資料
            let select_sql = prepare_sql(&client, "SELECT id, title, completed FROM todos WHERE id = $1").await.unwrap();

            match client.query_one(&select_sql, &[&id]).await {
                Ok(row) => {
                    let todo = Todo {
                        id: row.get(0),
                        title: row.get(1),
                        completed: row.get(2),
                    };
                    HttpResponse::Ok().json(todo)
                },
                Err(_) => HttpResponse::NotFound().body("Todo not found after update"),
            }
        },
        Err(_) => HttpResponse::NotFound().body("Todo not found"),
    }
}

//刪除todo
pub async fn delete_todo(pool: web::Data<Pool>, todo_id: web::Path<i64>) -> impl Responder {
    //從連接池取得一個資料庫連接
    let client = get_db_client(&pool).await.unwrap();

    //準備SQL語句，根據id刪除todo
    let sql = prepare_sql(&client, "DELETE FROM todos WHERE id = $1").await.unwrap();

    match client.execute(&sql, &[&todo_id.into_inner()]).await {
        Ok(_) => {
            HttpResponse::Ok().body("Todo deleted")
        },
        Err(_) => HttpResponse::NotFound().body("Todo not found"),
    }
}
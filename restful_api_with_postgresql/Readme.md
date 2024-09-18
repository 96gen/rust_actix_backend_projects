# 使用PostgreSQL資料庫的RESTful API

---

使用PostgreSQL資料庫，完成簡易的RESTful API。

本RESTful API專案有五種功能
- 新增Todo，POST http://127.0.0.1:8080/todos
- 查看全部的Todo，GET http://127.0.0.1:8080/todos
- 查看單一Todo，GET http://127.0.0.1:8080/todos/{id}
- 修改單一Todo的資料，PUT http://127.0.0.1:8080/todos/{id}
- 刪除Todo，DELETE http://127.0.0.1:8080/todos/{id}

其中新增和修改Todo，需要傳送Request Body，範例為
```json
{
    "title": "Test Title",
    "completed": false
}
```

---

## 如何啟動
```bash
cargo run
```

---

## 測試
```bash
cargo test
```
正常情況下，結果是這樣的。

```bash
running 5 tests
test tests::test_update_todo ... ok
test tests::test_create_todo ... ok
test tests::test_get_todo ... ok
test tests::test_get_todos ... ok
test tests::test_delete_todo ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
```

# 使用記憶體資料庫完成RESTful API

---

使用HashMap當作資料庫，完成簡易的RESTful API。

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
test tests::test_get_users ... ok
test tests::test_create_user ... ok
test tests::test_get_user ... ok
test tests::test_delete_user ... ok
test tests::test_update_user ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

---

本RESTful API專案有五種功能
- 新增User，POST http://127.0.0.1:8080/users
- 查看全部的User，GET http://127.0.0.1:8080/users
- 查看單一User，GET http://127.0.0.1:8080/users/{uuid}
- 修改單一User的資料，PUT http://127.0.0.1:8080/users/{uuid}
- 刪除User，DELETE http://127.0.0.1:8080/users/{uuid}

其中新增和修改User，需要傳送Request Body，範例為
```json
{
    "name": "Test User",
    "email": "test@example.com"
}
```
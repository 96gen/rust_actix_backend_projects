# Actix Hello World

---

## 如何啟動
```bash
cargo run
```

有三種路徑可以取得回應
- GET http://127.0.0.1:8080/ 固定回應Hello world!
- POST http://127.0.0.1:8080/echo 會根據RequestBody的內容，做出不同的回覆，Hello [RequestBody的內容]
- GET http://127.0.0.1:8080/hey 固定回應Hey there!

---

## 測試
```bash
cargo test
```
輸入以上內容，就能測試程式是否正確運作。

成功時的回應如下：
```
running 3 tests
test tests::test_manual_hello ... ok
test tests::test_echo ... ok
test tests::test_hello ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
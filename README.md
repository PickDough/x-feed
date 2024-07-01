## Usage

```bash
docker compose up # start all the containers
```

### POST `message`

```bash
curl --location 'localhost:3000/message' \
--header 'Content-Type: application/json' \
--data '{"title": "rust", "text": "is too hard"}'
```

### GET `message`

### http://localhost:3000/message 
P.S. Я не знаю, як зазвичай тестуються `http streaming` ендпоінти, наприклад, у `Postman` дані не відображались, але якщо відкрити просто у `Firefox`, то вони відображатимуться та оновлюватимуться, а коли завершити сервіс, то вони всі покажуться як завершена відповідь.

### Bot configuratiuon
```rust
/// A bot to spam the hell out of the feed api.
struct Cli {
    /// The url of the feed api.
    url: String,
    /// The time to wait between messages.
    interval_ms: u64,
}
```
Example:
```bash
./spam-bot -u http://api:3000/message -i 5000
```

### Cockroach Panel

### http://localhost:8080 
### `Login: roach, Password: roach`
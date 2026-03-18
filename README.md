# Card Game API with Rust, Rocket, JWT Auth, and MySQL

This repository is a beginner-friendly backend API for a card game.

It is written to be useful in two ways:

- it gives you a working project you can run
- it teaches what Rust, Rocket, APIs, JSON, authentication, and MySQL are while you use it

If you feel like you “know nothing” about Rust, Rocket, or APIs yet, that is okay. This README is written for that situation.

## 1. What this project is

This project is a **web API**.

A web API is a program that:

1. listens for HTTP requests
2. does some work
3. sends back a response, usually JSON

In this specific project, the API lets a game client:

- register a player account
- log in
- get a JWT bearer token
- view cards and rewards
- gain XP
- unlock cards
- claim rewards

This is **not** the game client or game UI.

It is the **server-side backend** that stores and returns game data.

## 2. What “backend API” means

Imagine a game launcher, website, or mobile app.

That app usually should not directly manage the database itself.

Instead, it sends requests like:

- “Create this player”
- “Log this player in”
- “Give this player 250 XP”
- “Show me the cards this player unlocked”

The backend API receives those requests and decides what should happen.

That is the job of this repository.

## 3. What technologies this project uses

This project uses:

- `Rust` for the programming language
- `Rocket` for the web framework
- `mysql_async` for talking to MySQL
- `MySQL` for the database
- `jsonwebtoken` for JWT tokens
- `argon2` for password hashing
- `Docker Compose` for starting the API and database together

## 4. What Rust is

Rust is a programming language.

People use Rust when they want:

- speed
- safety
- strong compiler checks
- fewer runtime surprises

Rust catches many mistakes **before the program runs**.

That is one reason it is popular for backend systems.

### Rust ideas you will see in this project

You do not need to memorize these immediately. Just know what they roughly mean.

- `struct`: a custom data type, similar to an object or record
- `enum`: one type that can be one of several variants
- `mod`: a module, used to organize code into files/folders
- `impl`: methods for a type
- `Result<T, E>`: a success value or an error value
- `async`: code that waits efficiently on things like network or database calls

Example:

```rust
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub xp: i32,
    pub level: i32,
}
```

That means “here is a type named `UserProfile` with those fields.”

## 5. What Rocket is

Rocket is a **web framework** for Rust.

A web framework helps you:

- define routes like `GET /health`
- parse JSON requests
- return JSON responses
- share app state
- start the server

Rocket makes this look clean and readable.

Example:

```rust
#[get("/health")]
async fn health() -> &'static str {
    "ok"
}
```

That means:

- this function handles `GET /health`
- when called, it returns `"ok"`

## 6. What an API route is

A **route** is an endpoint, or URL path, your server responds to.

Examples:

- `GET /health`
- `POST /api/auth/register`
- `POST /api/auth/login`
- `GET /api/catalog/cards`

Common HTTP verbs:

- `GET`: ask for data
- `POST`: create something or trigger an action
- `PUT`: replace something
- `PATCH`: partially update something
- `DELETE`: remove something

This project mainly uses `GET` and `POST`.

## 7. What JSON is

JSON is a text format used to send structured data.

Example JSON request:

```json
{
  "username": "player_one",
  "password": "hunter22"
}
```

Example JSON response:

```json
{
  "user_id": "123",
  "username": "player_one",
  "xp": 0,
  "level": 1
}
```

The API mostly communicates using JSON.

## 8. What JWT authentication is

JWT means **JSON Web Token**.

In simple terms:

1. a user registers or logs in
2. the API creates a signed token
3. the client sends that token on future requests
4. the API checks the token before allowing access

This project uses JWT bearer tokens.

That means protected requests should send a header like:

```text
Authorization: Bearer YOUR_TOKEN_HERE
```

## 9. What MySQL is

MySQL is the database.

The database stores things like:

- users
- password hashes
- cards
- rewards
- unlocked cards
- reward inventory

Without the database, the API cannot persist player progress.

## 10. High-level request flow

When a request comes in, this is the big picture:

1. Rocket matches the URL to a route
2. Rocket parses the input JSON
3. auth is checked if the route is protected
4. a service runs business logic
5. a repository runs SQL queries
6. the API returns JSON

That pattern is the core design of this project.

## 11. Project structure

```text
src\
  api\
    error.rs
  db\
    client.rs
    schema.rs
  features\
    auth\
      guard.rs
      jwt.rs
      models.rs
      routes.rs
      service.rs
    catalog\
      models.rs
      repository.rs
      routes.rs
      service.rs
    health\
      routes.rs
    progression\
      models.rs
      repository.rs
      routes.rs
      service.rs
    users\
      models.rs
      repository.rs
      routes.rs
      service.rs
  app_state.rs
  config.rs
  main.rs

Dockerfile
docker-compose.yml
.env.example
Cargo.toml
```

## 12. How the code is organized

This project uses a **feature-based structure**.

That means code is grouped by business feature instead of putting everything in one giant file.

For example:

- auth code is in `src\features\auth`
- user profile code is in `src\features\users`
- progression code is in `src\features\progression`

Inside each feature, you will usually see:

- `models.rs`: request/response types
- `routes.rs`: Rocket endpoints
- `service.rs`: business logic
- `repository.rs`: database queries

### Why this is useful

It makes the code easier to:

- read
- change
- test
- extend

## 13. Important files explained

### `Cargo.toml`

This is the Rust project manifest.

It declares:

- project name
- Rust edition
- dependencies

### `src\main.rs`

This is the entry point.

It:

- loads configuration
- creates the database connection wrapper
- creates the JWT manager
- bootstraps the database schema
- launches Rocket

### `src\config.rs`

This reads environment variables like:

- database host
- database port
- JWT secret
- app port

### `src\db\client.rs`

This creates MySQL connections.

### `src\db\schema.rs`

This handles database bootstrap:

- creating the database
- creating tables
- seeding starter cards and rewards

### `docker-compose.yml`

This starts:

- the API container
- the MySQL container

## 14. Prerequisites

You have two ways to run this project.

### Easiest option: Docker

Install:

- Docker Desktop

That is enough to run both the API and MySQL together.

### Local development option

Install:

- Rust
- Cargo
- MySQL

To verify Rust:

```powershell
rustc --version
cargo --version
```

## 15. How to run with Docker Compose

From the repository root:

```powershell
docker compose up --build
```

What this does:

1. builds the API container
2. starts MySQL
3. starts the API
4. waits for the API process to run

The API should be reachable at:

```text
http://localhost:8000
```

The MySQL port should be:

```text
localhost:3306
```

To stop everything:

```powershell
docker compose down
```

To also delete the database volume:

```powershell
docker compose down -v
```

## 16. How to run locally without Docker

First set environment variables.

Example in PowerShell:

```powershell
$env:APP_HOST="0.0.0.0"
$env:APP_PORT="8000"
$env:DATABASE_HOST="127.0.0.1"
$env:DATABASE_PORT="3306"
$env:DATABASE_NAME="card_game"
$env:DATABASE_USER="root"
$env:DATABASE_PASSWORD="change-me-root-password"
$env:DB_CONNECT_RETRIES="3"
$env:DB_CONNECT_DELAY_SECS="1"
$env:JWT_SECRET="change-me-for-production"
$env:JWT_EXPIRATION_SECS="86400"
```

Then run:

```powershell
cargo run
```

## 17. What startup should look like

When the app starts, it now prints messages like:

```text
Loading application configuration...
Configuration loaded. Preparing database at 127.0.0.1:3306 / card_game
Starting database bootstrap...
Bootstrapping database (attempt 1/3)...
```

If startup fails, you should now see a clear error instead of “nothing happened”.

## 18. Why the browser may show “nothing”

There are a few common reasons:

### Reason 1: the API is not actually running

If nothing is listening on port `8000`, the browser cannot connect.

### Reason 2: the API failed before Rocket started

This happens if the database is unavailable or misconfigured.

If MySQL is not reachable, the API stops **before** binding the web server.

### Reason 3: you opened the wrong URL

The best first URL to test is:

```text
http://localhost:8000/health
```

Not every API has a fancy browser homepage.

This project’s “first test” route is `/health`.

## 19. Troubleshooting startup

### Problem: `No connection could be made because the target machine actively refused it`

That means the API could not reach MySQL.

Check:

- is MySQL running?
- is it listening on port `3306`?
- are your environment variables correct?
- if using Docker Compose, did you run `docker compose up --build` instead of only `docker compose build`?

### Problem: Docker starts only MySQL but not the API

Usually that means:

- the API container build failed
- or the API process exited during startup

Run:

```powershell
docker compose up --build
```

and watch the output.

### Problem: browser says connection refused

That usually means nothing is listening on `localhost:8000`.

### Problem: `/` does not show what you expected

Try:

```text
http://localhost:8000/health
```

## 20. Environment variables

These are loaded from `src\config.rs`.

| Variable | Meaning | Default |
|---|---|---|
| `APP_HOST` | Host the server binds to | `0.0.0.0` |
| `APP_PORT` | Port the server listens on | `8000` |
| `DATABASE_HOST` | MySQL host | `127.0.0.1` |
| `DATABASE_PORT` | MySQL port | `3306` |
| `DATABASE_NAME` | Database name | `card_game` |
| `DATABASE_USER` | MySQL username | `root` |
| `DATABASE_PASSWORD` | MySQL password | `change-me-root-password` |
| `DB_CONNECT_RETRIES` | Startup retry count | `20` |
| `DB_CONNECT_DELAY_SECS` | Delay between retries | `3` |
| `JWT_SECRET` | Secret used to sign JWTs | `change-me-for-production` |
| `JWT_EXPIRATION_SECS` | Token lifetime in seconds | `86400` |

## 21. API endpoints

### Health

`GET /health`

Use this first.

Example:

```powershell
curl http://localhost:8000/health
```

### Register

`POST /api/auth/register`

Example:

```powershell
curl -X POST http://localhost:8000/api/auth/register `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"player_one\",\"password\":\"hunter22\"}"
```

### Login

`POST /api/auth/login`

Example:

```powershell
curl -X POST http://localhost:8000/api/auth/login `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"player_one\",\"password\":\"hunter22\"}"
```

### Catalog

`GET /api/catalog/cards`

`GET /api/catalog/rewards`

### Protected player routes

These require:

- a valid JWT
- the token’s user id must match the `user_id` in the URL

Examples:

- `GET /api/users/{user_id}`
- `GET /api/users/{user_id}/collection`
- `GET /api/users/{user_id}/rewards`
- `POST /api/users/{user_id}/progress`
- `POST /api/users/{user_id}/cards/{card_id}/unlock`
- `POST /api/users/{user_id}/rewards/{reward_id}/claim`

Protected request example:

```powershell
curl http://localhost:8000/api/users/USER_ID `
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 22. Full beginner test flow

If you want to test the whole API step by step:

1. run `docker compose up --build`
2. open `http://localhost:8000/health`
3. register a user with `POST /api/auth/register`
4. copy the returned `user_id`
5. copy the returned `access_token`
6. call `POST /api/users/{user_id}/progress`
7. call `GET /api/users/{user_id}/collection`
8. call `GET /api/users/{user_id}/rewards`

That is the main gameplay loop for this starter backend.

## 23. How authentication works in the code

The auth feature lives in:

```text
src\features\auth\
```

Important files:

- `jwt.rs`: creates and verifies JWTs
- `guard.rs`: Rocket request guard for protected routes
- `routes.rs`: register/login endpoints
- `service.rs`: hashing, validation, login logic

## 24. How progression works in the code

The progression feature lives in:

```text
src\features\progression\
```

It handles:

- XP gain
- level calculation
- automatic unlocks
- reward inventory updates

## 25. How the database schema works

The schema creates these tables:

- `users`
- `cards`
- `rewards`
- `user_cards`
- `user_rewards`

In simple terms:

- `users` stores players
- `cards` stores the full card catalog
- `rewards` stores the full reward catalog
- `user_cards` stores which player unlocked which card
- `user_rewards` stores each player’s reward inventory

## 26. How to extend this project

If you want to add a new feature, follow the same pattern.

Example: add “packs”.

Create:

```text
src\features\packs\
  mod.rs
  models.rs
  repository.rs
  routes.rs
  service.rs
```

Then:

1. define request/response types in `models.rs`
2. add SQL in `repository.rs`
3. add game rules in `service.rs`
4. add Rocket endpoints in `routes.rs`
5. expose the module in `src\features\mod.rs`
6. mount routes in `src\main.rs`
7. update `src\db\schema.rs` if new tables are needed

## 27. How to learn this repository without getting overwhelmed

Use this order:

1. read `Cargo.toml`
2. read `src\main.rs`
3. read `src\config.rs`
4. read `src\features\auth`
5. read `src\features\users`
6. read `src\features\progression`
7. read `src\db\schema.rs`

That order goes from “top-level app startup” to “deeper details”.

## 28. Commands you will use most

Run tests:

```powershell
cargo test
```

Run the app locally:

```powershell
cargo run
```

Run with Docker:

```powershell
docker compose up --build
```

Stop Docker services:

```powershell
docker compose down
```

## 29. Final summary

This repository is a starter backend for a card game using:

- Rust
- Rocket
- JWT authentication
- MySQL
- Docker Compose

If you are learning, focus on this idea:

- **routes receive requests**
- **services make decisions**
- **repositories talk to the database**
- **Rocket connects everything together**

If you want, I can also add a second beginner guide next that explains the code file-by-file with examples from this exact project.

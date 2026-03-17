# Card Game API with Rocket and MySQL

This project is a beginner-friendly Rust API for a card game.

It lets you:

- register accounts
- log in with username and password
- receive JWT bearer tokens
- list available cards and rewards
- grant player progress
- unlock cards
- claim rewards
- store everything in MySQL
- run the whole stack with Docker Compose

If you are new to Rust, new to APIs, or new to Rocket, this README is meant to teach the project from the ground up.

## 1. What this project is

An API is a program that waits for HTTP requests and sends JSON responses back.

This API is built with:

- `Rust`: the programming language
- `Rocket`: the web framework
- `jsonwebtoken`: JWT token signing and verification
- `argon2`: secure password hashing
- `mysql_async`: the MySQL driver
- `MySQL`: the database
- `Docker Compose`: the tool that runs multiple containers together

In plain English:

1. A client sends a request like “create a user” or “give this player XP”.
2. Rocket receives the request.
3. Rocket can also check a bearer token before protected routes run.
4. A route calls a service.
5. The service applies game rules.
6. A repository talks to MySQL.
7. The API returns JSON.

## 2. What Rust is doing here

Rust is a compiled systems language focused on speed, correctness, and memory safety.

Important ideas you will see in this codebase:

- `struct`: a custom data type, like `UserProfile`
- `impl`: methods attached to a type
- `mod`: a module, used to split code into files and folders
- `Result<T, E>`: a value that is either success or error
- `async`: work that can pause while waiting on I/O, like a database call
- ownership and borrowing: Rust’s way of making memory-safe code without a garbage collector

You do **not** need to master all of Rust before using this project. Start with this mental model:

- routes receive requests
- services contain business logic
- repositories contain SQL access
- models represent request and response data

## 3. What Rocket is doing here

Rocket is the web framework. It gives you a clean way to define HTTP endpoints.

Examples from this project:

- `#[get("/health")]` means “this function handles `GET /health`”
- `#[post("/auth/login", data = "<request>")]` means “this function handles `POST /auth/login` with JSON input”
- `State<AppState>` lets route handlers access shared application state
- `Json<T>` converts between Rust types and JSON
- a request guard like `AuthenticatedUser` can reject requests before your handler runs

Rocket also starts the web server and listens on a host and port.

The server is assembled in `src\main.rs`:

- config is loaded from environment variables
- the database is prepared
- shared state is created
- route groups are mounted
- Rocket launches the API

## 4. Project structure

```text
src\
  api\
    error.rs              # shared API error type and JSON error responses
  db\
    client.rs             # MySQL connection logic
    schema.rs             # database creation, tables, and seed data
  features\
    auth\                 # registration, login, JWTs, route guards
    health\               # health endpoint
    users\                # fetch user profiles
    catalog\              # list cards and rewards
    progression\          # XP gain, unlocks, and inventory
  app_state.rs            # shared state passed to Rocket
  config.rs               # environment-based configuration
  main.rs                 # app startup and route mounting

docker-compose.yml        # runs API + MySQL
Dockerfile                # container build for the API
.env.example              # sample environment values
Cargo.toml                # Rust dependencies
```

## 5. How the code is organized

This project uses a modular, feature-first structure.

Each feature folder usually contains:

- `models.rs`: Rust structs for requests and responses
- `routes.rs`: Rocket endpoints
- `service.rs`: business rules
- `repository.rs`: database operations

That pattern keeps the project easier to grow than putting everything in one file.

## 6. Prerequisites

You have two main ways to run the project:

- easiest: Docker Compose
- manual: Rust + MySQL installed separately

### Option A: easiest path with Docker

Install:

- Docker Desktop

That is enough to run both the API and the database together.

### Option B: local development without Docker

Install:

- Rust from `https://www.rust-lang.org/tools/install`
- a MySQL server instance
- optionally a REST client like Postman, Insomnia, or just `curl`

To verify Rust:

```powershell
rustc --version
cargo --version
```

## 7. How to run with Docker Compose

From the project root:

```powershell
docker compose up --build
```

What happens:

1. Docker builds the Rust API image.
2. Docker starts MySQL.
3. Docker starts the API.
4. On startup, the API creates the database if needed.
5. The API creates tables if needed.
6. The API inserts starter cards and rewards if the catalog is empty.

The API will be available at:

```text
http://localhost:8000
```

The MySQL port will be:

```text
localhost:3306
```

To stop everything:

```powershell
docker compose down
```

To stop everything and remove the database volume too:

```powershell
docker compose down -v
```

## 8. How to run locally without Docker

First, copy the example environment values and adjust them for your machine.

You can set them in PowerShell like this:

```powershell
$env:APP_HOST="0.0.0.0"
$env:APP_PORT="8000"
$env:DATABASE_HOST="127.0.0.1"
$env:DATABASE_PORT="3306"
$env:DATABASE_NAME="card_game"
$env:DATABASE_USER="root"
$env:DATABASE_PASSWORD="change-me-root-password"
$env:DB_CONNECT_RETRIES="20"
$env:DB_CONNECT_DELAY_SECS="3"
$env:JWT_SECRET="change-me-for-production"
$env:JWT_EXPIRATION_SECS="86400"
```

Then run:

```powershell
cargo run
```

The app will:

- connect to MySQL
- create the database if needed
- create tables
- seed starter data
- start Rocket

## 9. Environment variables

These are loaded in `src\config.rs`.

| Variable | Meaning | Default |
|---|---|---|
| `APP_HOST` | Host Rocket binds to | `0.0.0.0` |
| `APP_PORT` | Port Rocket listens on | `8000` |
| `DATABASE_HOST` | MySQL host | `127.0.0.1` |
| `DATABASE_PORT` | MySQL port | `1433` |
| `DATABASE_NAME` | App database name | `card_game` |
| `DATABASE_USER` | MySQL username | `root` |
| `DATABASE_PASSWORD` | MySQL password | `Your_strong_password123` |
| `DB_CONNECT_RETRIES` | Startup retry count | `20` |
| `DB_CONNECT_DELAY_SECS` | Delay between retries | `3` |
| `JWT_SECRET` | Secret used to sign bearer tokens | `change-me-for-production` |
| `JWT_EXPIRATION_SECS` | Token lifetime in seconds | `86400` |

## 10. API endpoints

### Health

`GET /health`

Checks that the API is running and the database is reachable.

Example:

```powershell
curl http://localhost:8000/health
```

### Authentication

`POST /api/auth/register`

Create a player account and immediately receive a bearer token.

```powershell
curl -X POST http://localhost:8000/api/auth/register `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"player_one\",\"password\":\"hunter22\"}"
```

`POST /api/auth/login`

Log in with username and password and receive a fresh bearer token.

```powershell
curl -X POST http://localhost:8000/api/auth/login `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"player_one\",\"password\":\"hunter22\"}"
```

### Users

`GET /api/users/{user_id}`

Fetch a user profile. This route is protected and requires an `Authorization: Bearer ...` header.

### Catalog

`GET /api/catalog/cards`

List all cards in the catalog.

`GET /api/catalog/rewards`

List all rewards in the catalog.

### Progression and unlocks

All routes in this section are protected and require a valid bearer token for the same `user_id` in the URL.

`POST /api/users/{user_id}/progress`

Give a player XP. If they cross a level threshold, they automatically unlock any cards and rewards assigned to those levels.

```powershell
curl -X POST http://localhost:8000/api/users/USER_ID/progress `
  -H "Authorization: Bearer YOUR_JWT_TOKEN" `
  -H "Content-Type: application/json" `
  -d "{\"xp_gained\":250}"
```

`GET /api/users/{user_id}/collection`

List the cards a player has unlocked.

`GET /api/users/{user_id}/rewards`

List the rewards a player has claimed or unlocked.

`POST /api/users/{user_id}/cards/{card_id}/unlock`

Unlock a specific card manually.

`POST /api/users/{user_id}/rewards/{reward_id}/claim`

Claim a reward manually.

```powershell
curl -X POST http://localhost:8000/api/users/USER_ID/rewards/reward-gold-100/claim `
  -H "Authorization: Bearer YOUR_JWT_TOKEN" `
  -H "Content-Type: application/json" `
  -d "{\"quantity\":2}"
```

## 11. Example beginner workflow

Here is a full flow:

1. Start the stack with `docker compose up --build`
2. Register with `POST /api/auth/register`
3. Copy the returned `user_id`
4. Copy the returned `access_token`
5. Call `POST /api/users/{user_id}/progress` with the bearer token
6. Check `GET /api/users/{user_id}/collection`
7. Check `GET /api/users/{user_id}/rewards`

That is the core gameplay loop of this starter API.

## 12. How the database works

The database code lives in:

- `src\db\client.rs`
- `src\db\schema.rs`

### Connection layer

`client.rs` creates MySQL connections using Tiberius.

It builds a MySQL connection config, opens a TCP connection, and hands that socket to the TDS driver.

### Startup bootstrap

`schema.rs` does three things:

1. connect to MySQL
2. create the application database if it does not exist
3. create tables and seed starter content

This means the project is easy to boot on a clean machine.

### Tables

The schema contains:

- `users`
- `cards`
- `rewards`
- `user_cards`
- `user_rewards`

In plain English:

- `users` stores players and their password hashes
- `cards` stores the master card catalog
- `rewards` stores the master reward catalog
- `user_cards` stores which player unlocked which card
- `user_rewards` stores each player’s reward inventory

## 13. How the request flow works

Take this request:

`POST /api/users/{user_id}/progress`

The flow is:

1. Rocket matches the route in `progression\routes.rs`
2. The `AuthenticatedUser` request guard reads the bearer token
3. The JWT is verified with the configured secret
4. Rocket deserializes JSON into `GrantProgressRequest`
5. The route creates `ProgressionService`
6. The service validates input
7. The service loads the user
8. The service calculates the new level from XP
9. The repository updates the user in MySQL
10. The repository loads all cards and rewards unlocked by the new levels
11. The repository writes unlock records
12. Rocket serializes the response as JSON

That same pattern is used across the app.

## 14. How Rust files talk to each other

Rust modules are declared with `mod`.

For example, in `main.rs`:

```rust
mod api;
mod app_state;
mod config;
mod db;
mod features;
```

That tells Rust to load those modules from the matching files and folders.

Inside feature folders, `mod.rs` exposes what the rest of the app should use.

That is why route mounting in `main.rs` can say:

```rust
.mount("/api", auth::routes())
.mount("/api", users::routes())
```

instead of knowing every file inside the `users` feature.

## 15. How Rocket route handlers work

A route handler is just an async Rust function with Rocket attributes.

Example pattern:

```rust
#[get("/users/<user_id>")]
async fn get_user(
    auth: AuthenticatedUser,
    state: &State<AppState>,
    user_id: &str,
) -> Result<Json<UserProfile>, ApiError> {
    // ...
}
```

Important parts:

- `#[get(...)]`: route definition
- `AuthenticatedUser`: Rocket guard that requires a valid JWT
- `State<AppState>`: shared app state
- `user_id: &str`: a value taken from the URL
- `Json<UserProfile>`: JSON response body
- `Result<..., ApiError>`: success or error

## 16. How services and repositories differ

This is one of the most important architecture ideas in the project.

### Services

Services answer:

- what rules should happen?
- what is valid?
- what business logic comes first?

Example:

- XP must be greater than zero
- level is calculated from XP
- unlocked cards and rewards should be granted when level thresholds are crossed

### Repositories

Repositories answer:

- what SQL should run?
- how do we fetch rows?
- how do we insert or update records?

Example:

- insert into `users`
- select from `cards`
- update `user_rewards`

Keeping those separate makes the code easier to test and expand.

## 17. How errors work

Shared API errors are defined in `src\api\error.rs`.

They map internal failures to clean HTTP responses like:

- `401 Unauthorized`
- `403 Forbidden`
- `400 Bad Request`
- `404 Not Found`
- `409 Conflict`
- `500 Internal Server Error`

This is better than crashing or returning random text.

## 18. How authentication works

This project uses JWT bearer tokens.

The auth code lives in:

- `src\features\auth\jwt.rs`
- `src\features\auth\guard.rs`
- `src\features\auth\service.rs`
- `src\features\auth\routes.rs`

The flow is:

1. A user registers or logs in with username and password.
2. The password is hashed with Argon2 before storage.
3. The API signs a JWT with `JWT_SECRET`.
4. The client sends that token in the `Authorization` header.
5. Rocket runs the `AuthenticatedUser` guard on protected routes.
6. If the token is valid, the request continues.
7. If the token is invalid or expired, the API returns `401`.

Example protected request:

```powershell
curl http://localhost:8000/api/users/USER_ID `
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

Treat `JWT_SECRET` like a secret. In real deployments, set a strong random value and do not commit a real production secret.

## 19. How to expand the API

The cleanest way to grow the project is to add a new feature module.

For example, imagine you want to add `packs`.

### Step 1: create a feature folder

Create:

```text
src\features\packs\
  mod.rs
  models.rs
  repository.rs
  routes.rs
  service.rs
```

### Step 2: add request and response models

Put JSON input/output structs in `models.rs`.

### Step 3: add repository methods

Write SQL queries in `repository.rs`.

### Step 4: add service logic

Put pack-opening rules in `service.rs`.

### Step 5: add Rocket routes

Define endpoints in `routes.rs`.

### Step 6: expose the module

Update `src\features\mod.rs`:

```rust
pub mod packs;
```

### Step 7: mount the routes

Update `src\main.rs`:

```rust
use features::{catalog, health, packs, progression, users};

.mount("/api", packs::routes())
```

### Step 8: add tables if needed

Update `src\db\schema.rs` to create any new SQL tables.

That is the basic pattern for almost every expansion.

## 20. How to add new game rules

Typical changes go here:

- progression formulas: `src\features\progression\service.rs`
- new unlock thresholds: `src\db\schema.rs` seed data or future admin tools
- new card fields: `catalog\models.rs` plus SQL schema and mapping code
- new reward types: `rewards` table seed data and any business logic that uses them

If you change the schema, make sure you update:

- table creation SQL
- Rust models
- row mapping code
- any endpoints returning that data

## 21. How to test the project

Run:

```powershell
cargo test
```

Right now the project includes unit tests for progression level math.

A good next step would be adding:

- service-level tests for unlock behavior
- repository integration tests against MySQL
- API tests for important routes
- auth integration tests for protected endpoints

## 22. Good next improvements

If you want to turn this into a more production-ready project, good next steps are:

- migrations instead of startup schema SQL
- connection pooling
- structured logging
- OpenAPI or Swagger docs
- pagination
- admin endpoints for card and reward management
- pack opening mechanics
- match history and leaderboards

## 23. If you are brand new, start here

If all of this still feels like a lot, use this order:

1. run `docker compose up --build`
2. call `GET /health`
3. call `POST /api/auth/register`
4. call `POST /api/users/{user_id}/progress`
5. read `src\main.rs`
6. read the `auth` feature folder
7. read one more feature folder from top to bottom
8. make one tiny change, like adding a new seeded card

That is the fastest way to learn this codebase.

## 24. Summary

This project is a modular Rocket-based game API with:

- Rust for the application language
- Rocket for HTTP routing and JSON handling
- JWT bearer token authentication for player routes
- MySQL for persistence
- Docker Compose for easy startup

If you want to grow it, follow the same pattern:

- models
- routes
- services
- repositories
- schema updates

That keeps the code organized as the game becomes bigger.


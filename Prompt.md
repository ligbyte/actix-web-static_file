# New Requirement

Please add JWT Authentication support for this application to protect all `/api/v1/*` routes,
allowing only requests with valid JWT tokens to access these routes. Please provide code examples for implementing this functionality.

The JWT algorithm is HS256, with the secret key being `my_secret_key`. The JWT token should contain the user's ID and expiration time (exp) claim.

# New Requirement

Please add SQLite database support for this application, using `Diesel` as the ORM tool. Requirements:

- Database file name is `app.sqlite`, stored in the `./db/` directory.
- Please create an `account` table with typical fields such as nick, password, email, phone, status, created_at, updated_at, etc. Save the create table statement in the `db/schema.ddl` file.
- Please provide code examples for creating a database connection pool.
- Create an account_repo module with basic CRUD operation code examples.
- Create an account_controller module with code examples for handling HTTP requests.



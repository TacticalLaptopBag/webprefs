# webprefs

This is a lightweight key-value store, intended to be used with simple applications that wish to store data online.

- [webprefs](#webprefs)
  - [Docker](#docker)
    - [Docker Compose](#docker-compose)
  - [API](#api)
    - [`/login`](#login)
      - [POST](#post)
      - [GET](#get)
      - [PUT](#put)
      - [DELETE](#delete)
    - [`/refresh`](#refresh)
      - [POST](#post-1)
    - [`/logout`](#logout)
      - [POST](#post-2)
    - [`/user`](#user)
      - [POST](#post-3)
    - [`/user/<id>`](#userid)
      - [GET](#get-1)
    - [`/prefs`](#prefs)
      - [GET](#get-2)
    - [`/prefs/scopes`](#prefsscopes)
      - [GET](#get-3)
    - [`/prefs/<scope>`](#prefsscope)
      - [GET](#get-4)
    - [`/prefs/<scope>/<key>`](#prefsscopekey)
      - [GET](#get-5)
      - [POST](#post-4)
      - [PUT](#put-1)
      - [DELETE](#delete-1)
  - [Developer Setup](#developer-setup)
    - [Dependencies](#dependencies)
      - [Debian/Ubuntu](#debianubuntu)
    - [Diesel](#diesel)

## Docker

A Docker image is available on Docker Hub as [tacticallaptopbag/webprefs][docker-hub].

* Ensure you set `JWT_SECRET` or else your installation will be very insecure.
* It's also recommended to set `CORS_ALLOWED_ORIGINS` to be a comma-separated list of valid origins.
* Create a Docker volume named `webprefs-data` to store webprefs data. Otherwise, all user data will be lost every time the Docker container is restarted.
* If you want to host a web app using the backend, you should bind mount your web files to `/app/web`
* All web traffic is expected to be on internal port 80. Ensure you expose this to a port of your liking.
```bash
docker run \
  -e JWT_SECRET=debug-key \  # Change this!
  -e CORS_ALLOWED_ORIGINS=\* \  # You should change this, but it's not necessary
  -p 8080:80 \  # Change 8080 to the port you want to use to connect to webprefs
  -v webprefs-data:/app/data \  # Ensure you create a webprefs-data volume!
  tacticallaptopbag/webprefs
```

### Docker Compose

If you use Docker Compose, download the example [docker-compose.yml](./docker-compose.yml),
run through its config, and then run
```bash
docker compose up
```


## API

All routes are prefixed with `/api/v1`.
All successful responses contain JSON in the following format:
```json
{
    "message": "Success message"
}
```
All error responses contain JSON in the following format:
```json
{
    "error": "Error message"
}
```

### `/login`

#### POST

Expects `x-www-form-urlencoded` (JavaScript HttpParams) data in the following format:
```json
{
    "username": "user",
    "password": "pass",
}
```
On success, sets `access_token` and `refresh_token` cookies to JWT tokens.

#### GET

Returns data about the currently logged in user, if valid token cookies are provided.
```json
{
    "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    "username": "user"
}
```

#### PUT

Updates the currently logged in user's password, if valid token cookies are provided.

Expects `x-www-form-urlencoded` (JavaScript HttpParams) data in the following format:
```json
{
    "old_password": "oldpass",
    "password": "newpass"
}
```

#### DELETE

Deletes the currently logged in user and invalidates login tokens

### `/refresh`

#### POST

Refreshes the currently logged in user's token, if a valid refresh token cookie is provided.

### `/logout`

#### POST

Invalidates the currently logged in user's token cookies,
and clears the `access_token` and `refresh_token` cookies.

### `/user`

#### POST

Expects `x-www-form-urlencoded` (JavaScript HttpParams) data in the following format:
```json
{
    "username": "user",
    "password": "pass",
}
```
Creates a new user with the given username and password.

### `/user/<id>`

#### GET

Gets information about the user with the given ID.
Does not require a login.

### `/prefs`

#### GET

Retrieves all prefs for the currently logged in user in the following format:
```json
{
    "prefs": [
        {
            "pref_key": "key",
            "pref_scope": "scope",
            "pref_value": "value"
        },
        ...
    ]
}
```

### `/prefs/scopes`

#### GET

Retrieves all scopes for the currently logged in user in the following format:
```json
{
    "scopes": [
        "scope1",
        "scope2",
        ...
    ]
}
```

### `/prefs/<scope>`

#### GET

Retrieves all prefs for the given scope
```json
{
    "prefs": [
        {
            "pref_key": "key",
            "pref_scope": "scope",
            "pref_value": "value"
        },
        ...
    ]
}
```

### `/prefs/<scope>/<key>`

#### GET

Retrieves the current value of the given pref key in the given scope.
```json
{
    "value": "prefvalue"
}
```
Note that the value can be null. If the key does not exist, a 404 is returned.

#### POST

Creates a pref entry with the given key in the given scope.
If one already exists, this endpoint will return an error.

Expects `x-www-form-urlencoded` (JavaScript HttpParams) data in the following format:
```json
{
    "value": "prefvalue"
}
```
The value may be null.

#### PUT

Updates a pref entry with the given key in the given scope.
If one does not exist, this endpoint will return an error.

Expects `x-www-form-urlencoded` (JavaScript HttpParams) data in the following format:
```json
{
    "value": "prefvalue"
}
```
The value may be null.

#### DELETE

Deletes a pref entry with the given key in the given scope.
If one does not exist, this endpoint will return an error.

## Developer Setup

### Dependencies

Requires sqlite3 to compile.

#### Debian/Ubuntu
```bash
sudo apt update && sudo apt install libsqlite3-dev
```

### Diesel

Install `diesel_cli` using either [cargo-binstall] or `cargo install`:
```bash
cargo install cargo-binstall
cargo binstall diesel_cli
```
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

Apply migrations:
```bash
diesel migration run
```


<!-- links -->
[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall
[docker-hub]: https://hub.docker.com/r/tacticallaptopbag/webprefs

<br/>
<p align="center">
  <h3 align="center">Tabby Web (API only)</h3>

  <p align="center">
    Tabby Web API for Config Sync writen in Rust
    <br/>
    <br/>
  </p>
</p>

![License](https://img.shields.io/github/license/Clem-Fern/rtabby-web-api) 

## About The Project / Disclamer

**This project has been made on educational purpose. It is not a fork of Eugeny/tabby-web and not affiliated to @Eugeny Tabby terminal project. You can't expect any support from there while using this project.**

As tabby web public instance app.tabby.sh has been discontinued. I decided to publish this as it provides a light, quick and easy way to deploy your own tabby config sync service. However, keep in mind that you used it at your own risk.

## Getting Started

Run your own instance with docker compose.

### Prerequisites

* Linux (AMD64/x86_64/Arm64) with docker engine.
  * Arm64 -> 0.4.0 and later [#14](https://github.com/Clem-Fern/rtabby-web-api/issues/14)

### Installation

Create a directory which will contain your `docker-compose.yml` and `config` volume.
```sh
mkdir -p rtabby-web-api/config
cd rtabby-web-api
```

rtabby-web-api store tabby's configuration in a database. You can choose between mysql or sqlite database. Third-party login will also be stored in database.

* Mysql
  ```sh
  # pwd /../../rtabby-web-api
  wget https://raw.githubusercontent.com/Clem-Fern/rtabby-web-api/master/docker-compose.yml
  ```

* Sqlite
  ```sh
  # pwd /../../rtabby-web-api
  wget https://raw.githubusercontent.com/Clem-Fern/rtabby-web-api/master/docker-compose-sqlite.yml -O docker-compose.yml
  ```

### Configuration

1. Create `config` directory. It will be used to store your config and certificate(not mandatory)

    ```sh
    # pwd /../../rtabby-web-api
    mkdir config
    touch config/users.yml
    # otherwise users.yml file will be created at first start 
    ```

2. Tabby uses a token to authenticate user. You have to create your own user with his token in `users.yml` to be able to use the sync service.

    ```yaml
    users:
    #...
        - name: 'You'
          token: 'token'
    #...
    ```
    Token must be a valid and unique uuid v4. You can create one [here](https://www.uuidgenerator.net/version4).

    rTabby supports OAuth2 providers like Github, Gitlab, Google or Microsoft. You can enable them by adding OAuth client and secret through env var in your `docker-compose.yml`.
    OAuth login callback is `/login/{provider}/callback`.

    ```yml
    environment:
      - DATABASE_URL=mysql://tabby:tabby@db/tabby
      #- GITHUB_APP_CLIENT_ID=
      #- GITHUB_APP_CLIENT_SECRET=
      #- GITLAB_APP_CLIENT_ID=
      #- GITLAB_APP_CLIENT_SECRET=
      #- GOOGLE_APP_CLIENT_ID=
      #- GOOGLE_APP_CLIENT_SECRET=
      #- MICROSOFT_APP_CLIENT_ID=
      #- MICROSOFT_APP_CLIENT_SECRET=
    ``` 

    When using OAuth prividers, browse to `http://<rtabby instance>/login` to authenticate and create your user and token.

3. (Optional) SSL/TLS

    Place your key and certificate into `config` directory. Then add the following lines in `docker-compose.yml` :
    ```yaml
          ports:
            - "8080:8080"
          environment:
            - DATABASE_URL=mysql://tabby:tabby@db/tabby
            - SSL_CERTIFICATE=cert.pem
            - SSL_CERTIFICATE_KEY=cert.key
          volumes:
            - ./config:/config
    ```

4. Miscellaneous
    
    rtabby-web-api get his configurations from env vars. Available tweaks :

    | ENV VAR | DESCRIPTION | EXAMPLE | DEFAULT |
    |---------|-------------|---------|---------|
    |DATABASE_URL|Url to database|sqlite:///config/db.sqlite|-|
    |CONFIG_FILE|Url to configuration file (Optional)|my_config.yml|users.yml|
    |BIND_ADDR|Address listening on (Optional)|0.0.0.0|0.0.0.0|
    |BIND_PORT|Port listening on (Optional)|8989|8080|
    |SSL_CERTIFICATE|Server certificate (Optional)|cert.pem|None|
    |SSL_CERTIFICATE_KEY|Server certificate private key(Optional)|private.key|None|    
    |CLEANUP_USERS|Delete configurations own by unknown user (Be careful)(Optional)|true|false|
    |HTTPS_CALLBACK|Third party login, enable https on callback uri(Optional)|true|false|  

## Usage

* To deploy
  ```sh
  docker compose up -d
  ```

* To shut down your deployment:
  ```sh
  docker compose down
  ```

## Contributing

Build dependencies:
  * Docker 
  * libmysqlclient for the Mysql backend (diesel depend on this)
  * Rust 1.65 or later
  * Diesel-rs to interact with migrations and schemas

Feel free to fork, request some features, submit issue or PR. Even give me some tips if you want, to help improve my code and knowledge in Rust ;)

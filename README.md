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

To run your own instance with docker compose.

### Prerequisites

* Linux (AMD64/x86_64) with docker engine.

### Installation

* Using rtabby-web-api image from Github Docker Repository **(recommended)**
  ```sh
  mkdir -p rtabby-web-api/config
  cd rtabby-web-api
  wget https://raw.githubusercontent.com/Clem-Fern/rtabby-web-api/master/docker-compose.yml
  ```

* From source
  ```sh
  git clone https://github.com/Clem-Fern/rtabby-web-api
  cd rtabby-web-api
  ```
  
  Edit `docker-compose.yml` to use local context build instead of the published image
  ```yaml
  # Uncomment build line
  build: .
  # Comment image
  # image: rtabby-web-api
  ```


### Configuration

1. Create `config` directory. It will be used to store your config and certificate(not mandatory)

    ```sh
    # pwd
    # ./rtabby-web-api
    mkdir config

    # Only from source installation and optional
    # users.yml file will be created at first start 
    # cp users.exemple.yml config/users.yml
    ```

2. Add some user into `users.yml`.

    ```yaml
    users:
    #...
        - name: 'You'
          token: 'token'
    #...
    ```
    Token must be a valid and unique uuid v4. You can create one [here](https://www.uuidgenerator.net/version4).

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
    |CONFIG_FILE|Url to configuration file (Optional)|my_config.yml|users.yml|
    |BIND_ADDR|Address listening on (Optional)|0.0.0.0|0.0.0.0|
    |BIND_PORT|Port listening on (Optional)|8989|8080|
    |SSL_CERTIFICATE|Server certificate (Optional)|cert.pem|None|
    |SSL_CERTIFICATE_KEY|Server certificate private key(Optional)|private.key|None|    

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

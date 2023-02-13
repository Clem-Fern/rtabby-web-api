# Contribute / Development

## Dependencies
Rust + Cargo + Diesel-cli (libmysql)
```
# Ex. Debian
sudo apt update
sudo apt install default-libmysqlclient-dev
cargo install diesel_cli --no-default-features --features mysql
```

## Run in development 
Create a .env file with DATABASE_URL pointing to your mariadb server
```
echo 'DATABASE_URL=mysql://tabby:tabby@db/tabby' >> .env        # change DATABASE_URL 
cp users.exemple.yml users.yml
cargo run -F dotenv     # Use dotenv feature to load the .env
```

## Quick start mariadb server docker
```
docker run -d --name tabby-mariadb --env MARIADB_USER=tabby --env MARIADB_PASSWORD=tabby --env MARIADB_RANDOM_ROOT_PASSWORD=yes  mariadb:latest
```
# Contribute / Development

## Dependencies
Rust + Cargo + Diesel-cli (libmysql, libsqlite3)
```
# Ex. Debian
sudo apt update
sudo apt install default-libmysqlclient-dev libsqlite3 
cargo install diesel_cli --no-default-features --features mysql,sqlite
```

libtirpc-dev required when building with mysql-bundle feature.

## Run in development 
Create a .env file with DATABASE_URL pointing to your mariadb server
```
echo 'DATABASE_URL=mysql://tabby:tabby@db/tabby' >> .env        # change DATABASE_URL 
cp users.exemple.yml users.yml
cargo run -F dotenv     # Use dotenv feature to load the .env
```

## Quick start mariadb server docker
```
docker run -d --name tabby-mariadb --env MARIADB_USER=tabby --env MARIADB_PASSWORD=tabby --env MARIADB_DATABASE=tabby --env MARIADB_RANDOM_ROOT_PASSWORD=yes -p 3306:3306 mariadb:latest
```


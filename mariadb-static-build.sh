#!/bin/bash

# https://github.com/Diggsey - https://github.com/sgrif/mysqlclient-sys/issues/17
curl -LO https://downloads.mariadb.com/Connectors/c/connector-c-3.3.3/mariadb-connector-c-3.3.3-src.tar.gz
tar xzf mariadb-connector-c-3.3.3-src.tar.gz
mkdir lib
mkdir build
cd build
sed 's/STRING(STRIP ${extra_dynamic_LDFLAGS} extra_dynamic_LDFLAGS)//' -i ../mariadb-connector-c-3.3.3-src/mariadb_config/CMakeLists.txt
sed 's/LIST(REMOVE_DUPLICATES extra_dynamic_LDFLAGS)//' -i ../mariadb-connector-c-3.3.3-src/mariadb_config/CMakeLists.txt
LDFLAGS=-L/usr/local/musl/lib cmake -DOPENSSL_USE_STATIC_LIBS=1 -DWITH_SSL=/usr/local/musl -DWITH_CURL=0 ../mariadb-connector-c-3.3.3-src
make mariadbclient
cp libmariadb/libmariadbclient.a ../lib/libmysqlclient.a
#!/bin/bash

mkdir zlib
curl -LO https://zlib.net/current/zlib.tar.gz
tar xzf zlib.tar.gz --strip-components=1 -C zlib
cd zlib
./configure --prefix=/ --static
make
make install
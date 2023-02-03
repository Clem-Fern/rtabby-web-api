#!/bin/bash

curl -LO http://zlib.net/zlib-1.2.13.tar.gz
tar xzf zlib-1.2.13.tar.gz
cd zlib-1.2.13
./configure --prefix=/ --static
make
make install
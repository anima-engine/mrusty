#!/bin/bash

# kcov source
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz

# kcov build
tar xzf master.tar.gz
mkdir kcov-master/build
cd kcov-master/build
cmake ..
make

cd ../..

kcov-master/build/src/kcov --verify \
  --exclude-pattern=/.cargo \
  --exclude-pattern=mruby-1.2.0 \
  --exclude-path=/usr/local/include/mruby \
  --include-pattern=src target/kcov target/debug/mrusty-*

kcov-master/build/src/kcov --verify \
  --coveralls-id=$TRAVIS_JOB_ID \
  --exclude-pattern=/.cargo \
  --exclude-pattern=mruby-1.2.0 \
  --exclude-path=/usr/local/include/mruby \
  --include-pattern=src target/kcov target/debug/lib-*

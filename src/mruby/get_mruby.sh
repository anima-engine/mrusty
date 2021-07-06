# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# This script generates mruby-out.tar which is the mruby repository compiled to
# C files.
#
# Dependencies:
# * Ruby
# * Bison
# * compile, linker & archiver
# * unzip

VERSION=3.0.0
CURRENT=$PWD

# Checks is /tmp/mruby needs cleaning or creation.

if [ -d /tmp/mruby ]; then
  rm -rf /tmp/mruby/*
else
  mkdir /tmp/mruby
fi

cd /tmp/mruby

wget https://github.com/mruby/mruby/archive/$VERSION.zip
unzip -u $VERSION.zip

mkdir -p mruby-out/src/mrblib
mkdir -p mruby-out/src/mrbgems

cd mruby-$VERSION

# remove mruby-io and bin dep
sed -i.bak '/stdlib-io/d' mrbgems/default.gembox
sed -i.bak '/mruby-bin-/d' mrbgems/default.gembox

# Add mruby-error dep
sed -i.bak '/Generate mruby-config command/a conf.gem :core => "mruby-error"' mrbgems/default.gembox

# minirake compiles the compiler and rb files to C.

./minirake

# Adds all .h files from include.

cp -vR include ../mruby-out
cp build/host/presym ../mruby-out/
cp build/host/include/mruby/presym/*.h ../mruby-out/include/mruby/presym

# Adds src and C-compiled mrblib.

cp src/*.c ../mruby-out/src
cp src/*.h ../mruby-out/src
cp src/*.pi ../mruby-out/src
cp src/presym ../mruby-out/src
cp build/host/mrblib/mrblib.c ../mruby-out/src/mrblib/mrblib.c

# Copies gems that are included in libmruby.
for fullpath in $(find build/host/mrbgems -type d -name 'mruby-*')
do
    mgemname=$(basename $fullpath)
    cp -R mrbgems/$mgemname ../mruby-out/src/mrbgems
    cp -R build/host/mrbgems/$mgemname ../mruby-out/src/mrbgems
done
cp build/host/mrbgems/gem_init.c ../mruby-out/src/mrbgems

cd ..

tar -cvf $CURRENT/mruby-out.tar mruby-out

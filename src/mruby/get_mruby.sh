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

VERSION=1.4.1
CURRENT=$PWD
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

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

# Use only supported gems.
cat $DIR/mrusty.gembox > mrbgems/default.gembox

# minirake compiles the compiler and rb files to C.

./minirake

# Adds all .h files from include.

cp -R include ../mruby-out

# Adds src and C-compiled mrblib.

cp src/*.c ../mruby-out/src
cp src/*.h ../mruby-out/src
cp build/host/mrblib/mrblib.c ../mruby-out/src/mrblib/mrblib.c

# Removes incompatible files.

find mrbgems -type f ! -name "*.c" -and ! -name "*.h" -and ! -name "*.def" -delete
find mrbgems -type d -empty -delete
find build/host/mrbgems -type f ! -name "*.c" -and ! -name "*.h" -delete
find build/host/mrbgems -type d -empty -delete

# Removes incompatible gems.

rm -rf mrbgems/mruby-bin*
rm -rf build/host/mrbgems/mruby-bin*

rm -rf mrbgems/mruby-test
rm -rf build/host/mrbgems/mruby-test

rm -rf mrbgems/mruby-io*
rm -rf build/host/mrbgems/mruby-io*

rm -rf mrbgems/mruby-socket*
rm -rf build/host/mrbgems/mruby-socket*

# Copies all gems.

cp -R mrbgems/* ../mruby-out/src/mrbgems
cp -R build/host/mrbgems/* ../mruby-out/src/mrbgems

# String needs additional mruby header.

sed -i -e $'s/#include "common.h"/#include "mruby.h"\\\n#include "common.h"/g' include/mruby/string.h

# Copies header files so they will be available in top level includes.

find include/mruby -type f -name '*.h' -exec cp {} ../mruby-out/include \;

# Copies ext header files required by gems.

mkdir -p ../mruby-out/include/mruby/ext
find mrbgems -path '*/ext/*' -name '*.h' -exec cp {} ../mruby-out/include/mruby/ext \;

cd ..

tar -cf $CURRENT/mruby-out.tar mruby-out

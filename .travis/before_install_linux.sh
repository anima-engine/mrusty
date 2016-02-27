#!/bin/bash

# get mruby 1.2.0 sources
wget https://github.com/mruby/mruby/archive/1.2.0.zip
unzip 1.2.0.zip

# add -fPIC flag to cc
sed -i "24s/.*/  conf.cc do |cc| cc.flags << \'-fPIC\' end/" mruby-1.2.0/build_config.rb

# build
cd mruby-1.2.0
./minirake

# add headers and lib to paths
LD_LIBRARY_PATH=$PWD/build/host/lib:$LD_LIBRARY_PATH
export LD_LIBRARY_PATH
C_INCLUDE_PATH=$PWD/include:$C_INCLUDE_PATH
export C_INCLUDE_PATH

# restore dir
cd ..

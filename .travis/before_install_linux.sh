#!/bin/bash

# install Ruby & Bison
sudo apt-get install -y ruby bison

# get mruby 1.2.0 sources
wget https://github.com/mruby/mruby/archive/1.2.0.zip
unzip 1.2.0.zip

# add -fPIC flag to cc
sed -i "24s/.*/  conf.cc do |cc| cc.flags << \'-fPIC\' end/" mruby-1.2.0/build_config.rb

# build
cd mruby-1.2.0
./minirake

# copy headers and lib
sudo cp -R include/* /usr/local/include
sudo cp build/host/lib/libmruby.a /usr/local/lib

# restore dir
cd ..

extern crate gcc;

fn main() {
    gcc::compile_library("libmrbe.a", &["src/mrb_ext.c"]);
}

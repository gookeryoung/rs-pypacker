use std::env;
use std::ffi;

fn main() {
    let args: Vec<ffi::OsString> = env::args_os().collect();
    dbg!(args);

    if args.len() < 2 {
        println!("[*] 选择当前路径作为项目源目录");
    }

    println!("Hello, world!");
}

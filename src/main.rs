use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("[*] args={:?}", args);

    if args.len() < 2 {
        println!("[*] 选择当前路径作为项目源目录");
    } else {
        println!("args[0]={}, args[1]={}", args[0], args[1]);
    }
}

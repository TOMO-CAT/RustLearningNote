// FromStr 特型为任何实现了 FromStr 特型的类型提供一个 from_str 方法, 尝试从字符串解析这个类型的值
use std::str::FromStr;
// 可以通过 args 函数获取命令行参数
use std::env;

fn main() {
    let mut numbers = Vec::new();
    
    // skip(1) 跳过迭代器第一个值
    for arg in env::args().skip(1) {
        // from_str 并不是 u64 的方法, 而是一个与 u64 相关联的函数, 类似于 C++ 的 static 函数
        // from_str 返回的不是 u64, 而是 Result<u64, ParseIntError> 类型
        // Rust 无异常, 所有错误都用 panic / Result 处理
        // expect 检查结果是否成功, 结果是 Err(e) 则直接打印 e 并退出程序, 结果是 Ok(v) 则返回 v
        numbers.push(i32::from_str(&arg).expect("error parsing argument"));
    }

    // cargo run -- 7 8 10 23
    if numbers.len() == 0 {
        // eprintln 写入 stderr
        eprintln!("Usage: sum <number>...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d += *m;
    }

    // {:?}
    println!("the sum of {:?} is {}", numbers, d);
}

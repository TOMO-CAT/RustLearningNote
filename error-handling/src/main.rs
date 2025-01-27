#[test]
fn panic() {
    // Rust 既可以在 panic 时展开调用栈 (默认方案), 也可以中止进程, 默认行为是:
    // 1. 把一条错误消息打印到终端
    // 2. 展开 (drop) 所有的调用栈
    // 3. 终止进程并返回一个非零状态码
    //
    // panic 是基于线程的, 一个线程 panic 其他线程可以继续做自己的事情
    //
    // 中止: 调用栈展开是默认的 panic 处理行为, 但在两种情况下 Rust 不会试图展开调用栈
    // 1. Rust 在试图清理第一个 panic 时, `.drop()` 方法触发了第二个 panic, 那么这个 panic 就是致命的
    // 2. 如果使用 -C panic=abort, 那么程序中的第一个 panic 会立即中止进程
}

#[test]
fn result() {
    // Rust 没有异常, 函数执行失败会返回如下的类型
    // fn get_weather(location: LatLng) -> Result<WeatherReport, io:Error>
    // 这意味着它要么返回一个成功结果 Ok(WeatherReport), 要么返回一个错误结果 Err(error_value)

    // 捕获错误
    // match get_weather(location) {
    //     Ok(report) => println!("The weather is {}", report),
    //     Err(err) => {
    //         println!("Error: {}", err);
    //     }
    // }

    // 尽管 match 是 Result 最彻底的处理方式, 但是太复杂了, Result<T, E> 针对一些常用场景提供了多个有用的方法
    fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("除数不能为零")
        } else {
            Ok(a / b)
        }
    }
    let result = divide(4, 2);
    assert!(result.is_ok() && !result.is_err()); // 已成功
    assert_eq!(result.ok(), Some(2)); // 成功值
    assert_eq!(result.err(), None); // 错误值
    assert_eq!(result.unwrap_or(100), 2); // 如果成功则返回成功值,
    assert_eq!(result.unwrap_or_else(|_err| 100), 2); // 如果成功则返回成功值, 否则返回闭包的返回值
    assert_eq!(result.unwrap(), 2); // 如果成功则返回成功值, 否则 panic
    assert_eq!(result.expect("除法失败"), 2); // 如果成功则返回成功值, 否则 panic 并打印错误消息

    // result.as_ref() -> Result<&T, &E>
    // result.as_mut() -> Result<&mut T, &mut E>
}

#[test]
// 打印错误
fn print_error() {
    use std::error::Error;
    use std::io;
    let err: Box<dyn Error> = Box::new(io::Error::new(io::ErrorKind::NotFound, "File not found"));

    println!("error: {}", err);
    println!("error: {:?}", err);
    println!("error: {}", err.to_string());

    // err.source() 返回错误来源, 如果没有来源则返回 None
}

#[test]
// 传播错误
fn propagate_error() {
    // 我们可以让错误沿着调用栈向上传播
    // let weather = get_weather(hometown)?;

    // 在返回 Option 类型的函数中, 我们也可以使用 ? 解包某个值
    // let weather = get_weather(hometown).ok()?;
}

#[test]
// // 自定义错误类型
fn self_defined_error() {
    use thiserror::Error;
    #[derive(Error, Debug, Clone)]
    #[error("{message:}({line:}, {column})")]
    pub struct JsonError {
        pub message: String,
        pub line: usize,
        pub column: usize,
    }

    let err = JsonError {
        message: "Invalid JSON syntax".to_string(),
        line: 3,
        column: 15,
    };
    assert_eq!(format!("{}", err), "Invalid JSON syntax(3, 15)");
}

fn main() {
    println!("Hello, world!");
}

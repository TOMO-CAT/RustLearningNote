#[test]
fn example() {
    // Rust 是表达式语言, if 和 match 可以生成值, 也可以作为参数传给函数
    let temperature = 73;
    let status = if temperature > 32 {
        "hot"
    } else if temperature < 0 {
        "cold"
    } else {
        "just right"
    };
    assert_eq!(status, "hot");
}

#[test]
// 块与分号
fn block_and_semicolon() {
    // Rust 中的分号是有实际意义的, 相当于 `()` 结尾
    let msg = {
        let x = 10;
        x + 1 // 返回值将存入 msg
    };
    assert_eq!(msg, 11);
}

#[test]
fn declaration() {
    {
        let name;
        // 由于 name 只会初始化一次, 所以不需要声明为 mut
        if true {
            name = "Alice";
        } else {
            name = "Bob";
        }
        assert_eq!(name, "Alice");
    }

    {
        // 遮蔽 (shadowing)
        // for line in file.lines() {
        //     let line = line?;
        // }
        //
        // 等价于
        //
        // for line_result in file.lines() {
        //     let line = line_result?;
        // }
    }
}

#[test]
fn match_expression() {
    // if 和 match 一样所有分支必须生成相同类型
    let x = 1;
    let _y = match x {
        1 => 2,
        2 => 3,
        _ => 4,
    };
}

#[test]
// if let 表达式
fn if_let_expression() {
    // 给定的 expr 要么匹配 pattern, 这时候运行 block1; 要么无法匹配, 这时候会运行 block2
    // if let pattern = expr {
    //     block1
    // } else {
    //     block2
    // }

    // 对于从 Option 或 Rusult 中获取数据是个好方法
    // if let Some(cookie) = request.session_cookie {
    //     return restore_session(cookie);
    // }
    // if let Err(err) = show_cheesy_anti_robot_task() {
    //     log_robot_attempt(err);
    //     politely_accuse_user_of_being_a_robot();
    // } else {
    //     session.mark_as_human();
    // }

    // if let 表达式不是必需的, 凡是能用 if let 做到的, match 也能做到, 它等价于
    // match expr {
    //     pattern => block1,
    //     _ => block2,
    // }
}

#[test]
// while 循环
fn while_loop() {
    // 四种循环表达式
    //
    // while condition {
    //     block
    // }
    //
    // expr 不等于 pattern 时退出循环
    // while let pattern = expr {
    //     block
    // }
    //
    // loop {
    //     block
    // }
    //
    // for pattern in iterable {
    //     block
    // }

    // 在 loop 循环体中, 可以在 break 后跟上一个表达式, 此表达式会成为此 loop 的值
    // let answer = loop {
    //     if let Some(line) = next_line() {
    //         if line.starts_with("answer: ") {
    //             break line; // 返回 line
    //         }
    //     } else {
    //         break "answer: nothing";
    //     }
    // };

    // break 可以带上 for 循环标签
    'search: for i in 0..10 {
        for j in 0..10 {
            if i * j == 42 {
                break 'search;
            }
        }
    }
}

#[test]
// 发散函数
fn divergent_function() {
    // `!` 表示函数不能正常返回
    fn _exit(code: i32) -> ! {
        std::process::exit(code)
    }
}

#[test]
// 比目鱼语法 ::<T> 代替 <T>, 否则会被错误识别成 `<` 运算符
fn turbofish() {
    // 错误: return Vec<i32>::with_capacity(1000);
    // 正确: return Vec::<i32>::with_capacity
}

#[test]
fn slice() {
    let v = vec![1, 2, 3, 4, 5];
    // `..` 运算符: 排除结束值 (半开放)
    let v1 = &v[1..3];
    assert_eq!(v1, &[2, 3]);
    // `..=` 运算符: 包含结束值 (封闭)
    let v2 = &v[1..=3];
    assert_eq!(v2, &[2, 3, 4]);
}

#[test]
// 闭包
fn closure() {
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    assert_eq!(is_even(4), true);
}

fn main() {
    println!("Hello, world!");
}

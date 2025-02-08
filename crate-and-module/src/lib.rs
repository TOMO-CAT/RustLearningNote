// 附着到整个 lib.rs 文件
#![allow(unused_imports)]
// rust 看到 mod math; 就会去找 math.rs 或者 math/mod.rs 文件
pub mod foo;
pub mod math;

// 定义嵌套的模块, 相当于 namespace, 对于一些简单的模块是可行的
pub mod geometry {
    pub mod shapes {
        pub fn circle_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }

        pub fn rectangle_area(width: f64, height: f64) -> f64 {
            width * height
        }
    }
}

// 导入模块 (同时导入 std::fs 和 std::fs::File)
#[allow(unused_imports)]
use std::fs::{self, File};

// 导入所有的语法项
#[allow(unused_imports)]
use std::io::prelude::*;

// 导入并设置别名
use std::io::Result as IOResult;

// 关键字 super 和 crate 在路径中有特殊的含义: super 表示父模块, crate 表示当前模块所在的 crate

// :: 表示绝对路径
// use ::image::Pixels;

// self 表示 . 相对路径
// use self::image::Pixels;

// super 表示 .. 相对路径
// use super::image::Pixels;

// 虽然 use 只是一个别名, 但是我们也可以公开它们
// pub use self::leaves::Leaf;

// 条件编译
#[cfg(target_os = "android")]
pub fn hello() {
    println!("Hello, Android!");
}

// 测试除零异常
#[test]
#[allow(unconditional_panic)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    let _ = 1 / 0;
}

// 对于大型项目, 应该将单测放在 tests 模块中
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

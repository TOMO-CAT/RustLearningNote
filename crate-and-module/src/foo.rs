// 公开结构体字段
pub struct Foo {
    pub x: i32,
    pub y: i32,
}

// 常量
pub const ROOM_TEMPERATURE: f32 = 20.0;

// 静态变量
// mut 静态变量本质上是非线程安全的, 安全代码根本不能使用它们
pub static ROOM_HUMIDITY: f32 = 50.0;

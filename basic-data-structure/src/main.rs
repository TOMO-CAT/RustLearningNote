#[test]
fn interger() {
    // 数值转换
    // 1. 范围内转换
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    // 2. 超出目标范围时截断, 转换生成的值等于原始值对 2^N 取模的值
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    // 检查算法: 返回结果的 Option 值, 如果数学意义上正确的结果可以表示为该类型的值就返回 Some(v), 否则返回 None
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    // 回绕算法: 返回取模后的值
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    // 饱和算法: 返回该类型能表示的最大值或最小值
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32767_i16).saturating_sub(10), -32768);
    // 溢出算法: 返回一个元组 <result, overflowed>, 后者表示是否溢出
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}

#[test]
fn float() {
    // IEEE 定义常量
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    // 按 IEEE 的规定, 精确等于 5.0
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
}

#[test]
fn tuples() {
    // 切割字符串
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(5);
    assert_eq!(head, "I see");
    assert_eq!(tail, " the eigenvalue in thine eye");
}

#[test]
fn reference() {
    // &T: 不可变的共享引用
    // &mut T: 可变的独占的引用
}

#[test]
fn box_var() {
    // Box<T>: 在堆上分配一个 T 类型的值, 并返回一个指向它的 Box<T> 类型的智能指针
    let b = Box::new(5);
    assert_eq!(*b, 5);
    // Box<T> 是一个智能指针, 它实现了 Deref 和 Drop trait
    // Deref trait 允许我们使用 * 运算符来解引用 Box<T> 类型的值
    // Drop trait 允许我们定义在 Box<T> 类型的值被销毁时应该执行的代码
}

#[test]
fn array() {
    // 数组
    // [T;N] 表示 N 个值的数组, 每个值类型为 T
    // 数组上的所有实用方法(遍历元素, 搜索, 排序, 填充, 过滤等) 都是作为切片而非数组的方法提供的
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    assert_eq!(lazy_caterer[3], 7);

    // 向量
    // Vec<T> 是一个可调整大小的 T 类型元素的数组, 它是在堆上分配的
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
    // 基于迭代器生成的值构建一个向量
    let mut v: Vec<i32> = (0..5).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
    // 在索引为 3 的元素处塞入 35
    v.insert(3, 35);
    assert_eq!(v, vec![0, 1, 2, 35, 3, 4]);
    // 删除元素
    v.remove(1);
    assert_eq!(v, vec![0, 2, 35, 3, 4]);
    // 弹出并返回元素
    assert_eq!(v.pop(), Some(4));
    // 遍历
    for i in v {
        println!("{}", i);
    }

    // 切片
    // 切片总是通过引用传递
    // 对切片的引用是一个胖指针, 包含指向切片第一个元素的指针和切片中元素的数量

    // 引用和切片的区别
    // 普通引用是指向单个值的非拥有型指针; 而切片引用是指向内存中一套连续值的非拥有型指针
}

#[test]
fn string() {
    // 原始字符串, 用 r 标记
    let default_win_install_path = r"C:\Program Files\MyProg";
    println!("{}", default_win_install_path);
    println!(r###"
        This raw string started with 'r###'.
        Therefore it does not end until we reach a quote mark ('"')
        followed by three hash marks ('###').
    "###);

    // 字节串
    // 带有 b 前缀的字符串字面量都是字节串, 它是 u8 值(字节)
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // 创建 String 的方法
    // 1. .to_string() 会将 &str 转换成 String, 这会复制字符串
    // 2. .to_owned() 同上, 这种命名风格适用于另一些类型
    // 3. format!() 宏会返回一个新的 String
    // 4. 字符串的数组、切片和向量都有两个方法 (.concat() 和 .join(sep)), 他们会返回一个新的 String
}

fn main() {
}

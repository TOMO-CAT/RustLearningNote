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
fn boolean() {
    // boolean 转整型
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

#[test]
fn char() {
    // 字符串字面量
    assert_eq!("hello".chars().nth(0), Some('h'));
    assert_eq!("hello".chars().nth(1), Some('e'));
    assert_eq!("hello".chars().nth(2), Some('l'));
    assert_eq!("hello".chars().nth(3), Some('l'));

    // u8 是唯一能通过 as 运算符转换为 char 的类型, 因为除 u8 以外的整型都可能包含 Unicode 码点之外的值
    assert_eq!(230u8 as char, 'æ');
    // 作为替代方案, 可以使用标准库 std::char::from_u32 函数
    assert_eq!(std::char::from_u32(230), Some('æ'));
}

#[test]
fn tuples() {
    // 切割字符串
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(5);
    assert_eq!(head, "I see");
    assert_eq!(tail, " the eigenvalue in thine eye");

    // 零元组 (单元类型)
    let _unit: () = ();

    // 单值元组 (必须在值后面添加逗号, 以区分单值元组和括号表达式)
    let _single_val_tuples: (i32,) = (3,);
}

// 引用
// 1. Rust 引用永远不为空
// 2. 共享引用和可变引用不能同时存在
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
    // 1. 切片是数组或向量中的一个区域
    // 2. 切片总是通过引用传递
    // 3. 对切片的引用是一个胖指针, 包含指向切片第一个元素的指针和切片中元素的数量
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    // 引用和切片的区别
    // 普通引用是指向单个值的非拥有型指针; 而切片引用是指向内存中一套连续值的非拥有型指针
    fn print(n: &[f64]) {
        for ele in n {
            println!("{}", ele);
        }
    }
    print(&v[0..2]); // 打印 v 前两个元素
    print(&a[2..]); // 打印从 a[2] 开始的元素
    print(&sv[1..3]); // 打印 v[1] 和 v[2]
    print(&sa); // 打印数组 a
}

#[test]
fn string() {
    // 原始字符串, 用 r 标记
    // 会包含所有原始字符串里的反斜杠和空白字符
    let default_win_install_path = r"C:\Program Files\MyProg";
    println!("{}", default_win_install_path);
    // 如果原始字符串中包含 `"`, 那么我们可以在原始字符串的开头和结尾添加 # 标记
    println!(
        r###"
        This raw string started with 'r###'.
        Therefore it does not end until we reach a quote mark ('"')
        followed by three hash marks ('###').
    "###
    );
    // 如果一个字符串的一行以 \ 结尾, 那么就会舍弃其后的换行符和前导空格
    println!(
        "It was a bright, cold day in April, and \
        there were four of us——\
        more or less."
    );

    // 字节串
    // 1. 带有 b 前缀的字符串字面量都是字节串, 它是 u8 值(字节)
    // 2. 字节串不能包含任意 Unicode 字符, 它们只能使用 ASCII 和 \xHH 转移字符
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // 创建 String 的方法
    // 1. .to_string() 会将 &str 转换成 String, 这会复制字符串
    // 2. .to_owned() 同上, 这种命名风格适用于另一些类型
    // 3. format!() 宏会返回一个新的 String
    // 4. 字符串的数组、切片和向量都有两个方法 (.concat() 和 .join(sep)), 他们会返回一个新的 String
    // 5. 每个 String 在堆上分配了自己的缓冲区, 不会和任何 String 共享, String 变量超过作用域时缓冲区自动释放 (除非被移动)
    // 6. String 更像是 vec<T>, &str 更像是 &[T]
    let noodles = "noodles".to_string();
    let _oodles = &noodles[1..]; // &str

    // String 或 &str 的 .len() 方法会返回长度 (但是这个长度是以字节而不是字符为单位)
    assert_eq!("🐍".len(), 4);
    assert_eq!("🐍".chars().count(), 1);
}

#[test]
fn alias() {
    // 类似于 C++ 的 typedef, type 关键字可以为现有类型声明一个新名称
    type Bytes = Vec<u8>;
    let _foo: Bytes = vec![1, 2, 3];
}

fn main() {}

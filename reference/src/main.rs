#[test]
fn example() {
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    // 共享引用传递
    fn show(table: &Table) {
        // artist: &String
        // works: &Vec<String>
        for (artist, works) in table {
            println!("works by {}:", artist);
            for work in works {
                println!("  {}", work);
            }
        }
    }

    let mut table = Table::new();
    table.insert("Beethoven".to_string(), vec!["Symphony No. 9".to_string()]);
    table.insert(
        "Mozart".to_string(),
        vec!["The Marriage of Figaro".to_string()],
    );
    table.insert(
        "Bach".to_string(),
        vec!["The Well-Tempered Clavier".to_string()],
    );
    show(&table);

    // 可变引用 &mut T (读作 ref mute T)
    fn sort_works(table: &mut Table) {
        for works in table.values_mut() {
            works.sort();
        }
    }
    sort_works(&mut table);
}

#[test]
fn deref() {
    // 引用通过 `&` 创建, 再通过 `*` 显式解引用
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    // `.` 运算符会隐式解引用
    let mut v = vec![1, 2, 3];
    v.sort(); // 隐式借用对 v 的可变引用
    (&mut v).sort(); // 显式借用对 v 的可变引用
}

#[test]
fn ref_assign() {
    // C++ 的引用一旦完成初始化就无法再指向别处
    // 但是 Rust 可以修改引用变量指向的地方
    let x = 10;
    let y = 20;
    let mut r = &x; // r 一开始指向 x
    if true {
        r = &y; // r 现在指向 y
    }
    assert!(*r == 20);
}

#[test]
fn multiple_ref() {
    // Rust 允许对引用进行引用
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 321, y: 123 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 123);
    assert_eq!(rrr.x, 321);
}

#[test]
fn ref_compare() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    // 比较运算符也可以“看穿”任意数量的引用
    assert!(rrx <= rry);
    assert!(rrx == rry);
}

#[test]
fn expression_ref() {
    // C/C++ 只允许将 & 运算符应用于某些特定种类的表达式
    // Rust 允许将 & 运算符应用于任何表达式
    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }
    let r = &factorial(6); // r 是一个对 usize 的引用
    assert_eq!(r + &1009, 1729); // 数学运算符可以看穿 “一层” 引用
}

#[test]
fn ref_safety() {
    // 引用安全
    //
    // 1. 不能借用局部变量的引用并将其移出变量的作用域
    {
        let _r;
        {
            let x = 1;
            _r = &x; // 借用 x
        }
        // 编译错误: 试图读取局部变量 `x` 所占的内存
        // 1. 对 x 的引用的生命周期不能超过 x 本身
        // 2. 引用的生命周期必须完全涵盖 r 的生命周期
        // assert_eq(*_r, 1);
    }

    // 2. 将引用作为函数参数
    static mut STASH: &i32 = &128; // 静态变量必须初始化
    fn f(p: &'static i32) {
        // f 函数签名必须指出 p 是生命周期为 'static 的引用
        unsafe {
            // 只能在 unsafe 块中访问可变静态变量
            STASH = p;
        }
    }
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}

#[test]
fn pass_ref_to_function() {
    // 把引用传递给函数时不需要指出生命周期, 只需要在定义函数和类型时关心生命周期即可
    // Rust 会自动为你推断生命周期
    fn g<'a>(p: &'a i32) {
        println!("p = {}", p);
    }
    let x = 10;
    g(&x);
}

#[test]
// 返回引用
fn return_value_of_ref() {
    // 函数通常会接收某个数据结构的引用, 然后返回对该结构某个部分的引用
    //
    // 当函数以单个引用作为参数并返回单个引用时, Rust 会假定两者具有相同的生命周期
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }
        s
    }
    let s;
    {
        let parabola = [1, 3, 2, 4, 5];
        s = smallest(&parabola);
        println!("s = {}", s);
    }
    // 编译错误: 指向了已被丢弃的数组的元素
    // assert_eq!(*s, 1);
}

#[test]
// 包含引用的结构体
fn struct_with_ref_field() {
    // 当一个引用类型出现在另一个类型的定义中时, 必须写出它的生命周期
    struct S<'a> {
        r: &'a i32,
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
        println!("s.r = {}", s.r);
    }
    // 编译错误: 从已被丢弃的 s 中读取值
    // assert_eq!(*s.r, 10);
}

#[test]
// 不同的生命周期
fn different_lifetimes() {
    // x 和 y 都有各自独立的生命周期
    #[allow(dead_code)]
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    println!("{}", r);
}

#[test]
// 省略生命周期参数
fn omit_lifetimes() {
    // 如果函数的参数只有一个生命周期, 那么 Rust 就会假设返回值具有同样的生命周期
    fn _first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[2])
    }
    // 等价于
    fn _first_third2<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
        (&point[0], &point[2])
    }

    // 如果函数是某个类型的方法, 并且具有引用类型的 self 参数, 那么 Rust 就会假定返回值的生命周期和 self 参数相同
    #[allow(dead_code)]
    struct StringTable {
        elements: Vec<String>,
    }
    impl StringTable {
        #[allow(dead_code)]
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
        // 完整写出生命周期的函数声明
        // 这意味着 Rust 假定无论你借用的是什么, 本质上都是从 self 借用的
        // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {}
    }
}

#[test]
// 共享引用与可变引用
fn mut_ref() {
    {
        let mut x = 10;
        let r1 = &x;
        let r2 = &x;
        // x += 10; // 错误: x 已经被借出, 不能被赋值或者被移动值
        // let m = &mut x; // 错误: 不能把 x 接入为可变引用, 因为它涵盖在已解除的不可变引用 r1 和 r2 的生命周期内
        println!("r1 = {} r2 = {}", r1, r2);
    }

    {
        let mut y = 20;
        let m1 = &mut y;
        // let m2 = &mut y; // 错误: 不能有多个可变引用
        println!("m1 = {}", m1);
    }

    {
        let mut w = (107, 109);
        let r = &w;
        let r0 = &r.0;
        // let m1 = &mut r.1;  // 错误: 不能把共享引用重新借入为可变引用
        println!("{}", r0);
    }

    {
        let mut v = (136, 139);
        let m = &mut v;
        let m0 = &mut m.0; // 正确: 从可变引用中借入可变引用
        *m0 = 137;
        let r1 = &m.1; // 正确: 从可变引用中借入共享引用, 并且不能和 m0 重叠
        println!("{}", r1);
    }
}

fn main() {}

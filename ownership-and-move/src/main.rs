#[test]
fn example() {
    // *******************************************************
    // composers 拥有一个向量, 向量拥有自己的元素
    // 每个元素都是一个 Person 结构体, 每个结构体有自己的字段
    // 字符串字段拥有自己的文本
    //
    // 拥有者及其拥有的那些值形成了一棵树, 当该变量超过作用域时,
    // 整棵树都将随之消失
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    // 如果不加 `&` 话, 循环结束后 composers 将不再可用, 因为所有权已转移
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

#[test]
fn movement() {
    // Rust 中对于大多数类型而言, 变量赋值、将其传给函数或从函数返回这样的操作都不会复制值, 而是移动值
    let s = vec!["don".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = t;
    // 如果需要显式地复制, 则需要使用向量的 clone 方法
    let _copy = u.clone();

    // 将一个值转移给 "已初始化" 的变量, 那么 Rust 就会丢弃该变量的先前值
    #[allow(unused_assignments)]
    let mut s = "Cat".to_string();
    s = "Dog".to_string(); // 这里丢弃了原始值 "Cat"
    println!("{}", s);
}

#[test]
fn control_flow() {
    fn print_vec_idx(vec: Vec<i32>, idx: usize) {
        println!("index: {}, Value: {}", idx, vec[idx]);
    }

    // 由于只会命中 if 中其中一个分支, 所以你可以在两个分支中都使用 x
    let x = vec![10, 20, 30];
    if true {
        print_vec_idx(x, 1);
    } else {
        print_vec_idx(x, 2);
    }
    // 编译报错: x 已经被移动过, 这里是未初始化状态
    // print_vec_idx(x, 3);

    // 编译错误: 禁止在循环中进行变量移动
    // let y = vec![10, 20, 30];
    // for i in 0..3 {
    //     print_vec_idx(y, i);
    // }
}

#[test]
fn move_vector() {
    // 构造向量
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 从向量中随机抽取元素
    // 编译错误: 无法将单独一个元素移动出向量
    // let _third = v[2];

    // 如果我们真的想将单独一个元素移动出向量, 通常有三种做法
    //
    // 1. 从向量末尾弹出一个值
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");
    // 2. 将向量中指定索引处的值和最后一个值交换, 并将前者移动出来
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    // 3. 把要取出来的值和另一个值交换
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    // 查看向量中还存在什么元素
    assert_eq!(v, vec!["101", "104", "substitute"]);

    // 在循环中消耗向量所有元素
    let v2 = ["cat".to_string(), "dog".to_string(), "fish".to_string()];
    for mut s in v2 {
        s.push_str("!");
        println!("{}", s);
    }

    // 如果真的有动态移除的需求, 可以将类型更改为能动态跟踪自己是否有值的类型
    #[allow(dead_code)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    let first_name = composers[0].name.take().expect("no name");
    assert_eq!(first_name, "Palestrina");
}

#[test]
fn copy() {
    // Rust 中, 基本数据类型是按值拷贝的, 而不是按引用拷贝的
    // 对于复合类型, 如果其所有字段都是 Copy 的, 那么它也是 Copy 的
    //
    // 1. 标准的 Copy 类型包括所有机器整数类型、浮点数类型、char 和 bool
    // 2. Copy 类型的元组或固定大小的数组也是 Copy 类型
    // 3. Box<T> 不是 Copy 类型, 它拥有从堆中分配的缓冲区
    // 4. MutexGuard 也不是 Copy 类型, 它的复制毫无意义
    // 5. 自定义类型 struct 和 enum 默认不是 Copy 类型

    // 可以通过 `#[derive(Copy, Clone)]` 属性为自定义类型添加 Copy 和 Clone 特性
    #[derive(Copy, Clone)]
    struct Label {
        number: i32,
    }
    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }
    let l = Label { number: 1 };
    print(l);
    println!("My label number is {}", l.number);
}

#[test]
#[allow(non_snake_case)]
fn Rc_and_Arc() {
    // Rust 提供了引用计数型指针 Rc 和 Arc
    // 1. Arc 是原子引用计数的缩写 (Atomic Reference Counting)
    // 2. Rc 和 Arc 的唯一区别是 Arc 是线程安全的
    // 3. 如果不需要在线程之间共享指针, 那么没有理由为 Arc 的性能损失买单, 应该使用 Rc

    use std::rc::Rc;

    // 对于任意类型 T, Rc<T> 是一个附带引用计数的在堆上分配的 T 型指针
    // 克隆一个 Rc<T> 并不会复制 T, 只会创建另一个指向它的指针并递增引用计数
    let s: Rc<String> = Rc::new("cat".to_string());
    #[allow(unused_variables)]
    let t: Rc<String> = s.clone();
    #[allow(unused_variables)]
    let u: Rc<String> = s.clone();

    // Rc 指针拥有的值是不可变的, 所以你不能修改 Rc 指针指向的值
    // Rust 内存和线程安全保证的基石是: 确保不会有任何值是既共享又可变的
}

fn main() {
    println!("Hello, world!");
}

fn gcd(mut n : u64, mut m : u64) -> u64 {
    // 不满足时触发 panic
    // debug_assert! 宏只在 debug 模式下有效
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // 函数返回值不需要使用 return 关键字, 而且没有分号结尾
    n
}

#[test]
fn test_gld() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    println!("Hello, world!");
    println!("gcd(14, 49 is {}", gcd(14, 49));
}

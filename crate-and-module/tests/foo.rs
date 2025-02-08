use crate_and_module::*;

// cargo test --test foo
// 集成测试: 从外部视角看待 crate, 就像用户的做法一样, 可以测试公共 API
#[test]
fn test_foo() {
    assert_eq!(math::add(1, 2), 3);
}

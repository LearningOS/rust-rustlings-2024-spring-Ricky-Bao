// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 删除无用的 if 语句块，并删除 unwrap
    // 删除这一行：my_option.unwrap();

    // 在数组的初始化中添加逗号
    let my_arr = &[
        -1, -2, -3, // 添加逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 创建一个空的向量
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换两个值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

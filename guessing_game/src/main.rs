// 引入标准库中的io库
use std::io;

fn main() {
    println!("猜测一个数字!");
    println!("请输入你的猜测.");

    // 创建一个新的字符串变量. 使用 mut 关键字来创建一个可变变量
    // ::new 是一个关联函数，关联函数是针对类型实现的函数. 就是静态方法
    let mut guess = String::new();

    // &mut guess 传递一个可变引用给 read_line
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你的猜测: {}", guess);

}

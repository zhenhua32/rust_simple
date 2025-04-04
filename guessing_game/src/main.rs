// 引入标准库中的io库
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜测一个数字!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("秘密数字是: {}", secret_number);

    loop {
        println!("请输入你的猜测.");

        // 创建一个新的字符串变量. 使用 mut 关键字来创建一个可变变量
        // ::new 是一个关联函数，关联函数是针对类型实现的函数. 就是静态方法
        let mut guess = String::new();

        // &mut guess 传递一个可变引用给 read_line
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // trim 去除字符串两端的空白字符. parse 将字符串转换为数字
        let guess: u32 = match guess.trim().parse() {
            // 这些是枚举, parse 返回一个 Result 类型, Result 是有两个成员的枚举: Ok 和 Err
            Ok(num) => num,
            // _ 是一个通配符, 匹配所有 Err 值. continue 会跳过剩下的代码并进入下一次循环
            Err(_) => continue,
        };

        println!("你的猜测: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
}

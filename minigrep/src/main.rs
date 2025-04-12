use std::env;
use std::process;

use minigrep::Config;

// 启动命令: cargo run -- "duct tape" poem.txt
// $Env:IGNORE_CASE=1; cargo run -- "to" .\poem.txt
// Remove-Item Env:IGNORE_CASE
// 运行测试 cargo test
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }


}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("[猜数字小游戏V1.0]");

    //随机产生1-100之间的数字
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut times: i32 = 1;
    println!("我已经默默选好了1~100之间的一个数字.");
    loop {
        println!("请输入你猜的数字 (输入负数放弃猜数).");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number.");

        //转换为数字，忽略错误输入
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //输入负数直接退出
        if guess < 0 {
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" #第[{}]次 -- {}太小了！", times, guess),
            Ordering::Greater => println!(" #第[{}]次 -- {}太大了!", times, guess),
            Ordering::Equal => {
                println!("******** #[{}] 恭喜你，猜对了! ********", times);
                break;
            }
        }
        times += 1;
    }
    show_secret(secret_number);
    if guess_again() {
        main();
    }
}

fn show_secret(num: i32) {
    println!("...我刚才默默想好的数字是：{} ！", num);
}

fn guess_again() -> bool {
    println!("再玩一次好不好？(y/N)");
    let mut again = String::new();
    io::stdin().read_line(&mut again).expect("请输入Y/N");
    "Y" == again.trim().to_uppercase()
}

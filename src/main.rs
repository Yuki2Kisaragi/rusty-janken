use rand::Rng;
use std::io::stdin;

// #[derive(Debug)]でEnum型を文字列にする
#[derive(Debug)]
enum GameResult {
    Draw = 0, // 本来は不要だが,明示的に定義する
    Win,
    Lose,
    Null,
}

fn disp_hand(num: &i32) -> &str {
    match num {
        0 => "Lock",
        1 => "Sissor",
        2 => "Paper",
        _ => "None",
    }
}

fn comp_hands(user_hands: i32, cpu_hands: i32) -> GameResult {
    // USERとCPUの出した手(0~2)を使い、勝ち負けを判断する
    // USER - CPU = 0 -> Draw
    // USERが勝つパターン
    //  Lock   vs Sissor  : 0 - 1 = -1
    //  Sissor vs Paper   : 1 - 2 = -1
    //  Paper  vs Lock    : 2 - 0 =  2
    // USERが負けるパターン
    //  Lock   vs Paper   : 0 - 2 = -2
    //  Sissor vs Lock    : 1 - 0 =  1
    //  Paper  vs Sissor  : 2 - 1 =  1
    let comp: i32 = (user_hands - cpu_hands + 3) % 3;

    match comp {
        0 => GameResult::Draw,
        1 => GameResult::Lose,
        2 => GameResult::Win,
        _ => GameResult::Null,
    }
}

fn main() {
    // 0~2までの乱数を作る
    let cpu_choice: i32 = rand::thread_rng().gen_range(0..3);

    // CPUの手を見る
    // println!("CPU choices: {}", disp_hand(&cpu_choice));

    loop {
        println!("Please input your choice. [0:lock, 1:sissor, 2:paper]");
        let mut input_str = String::new();
        stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        // 入力文字から数字を解析する
        // 失敗したらループ最初へ戻る
        let input_number: i32 = match input_str.trim().parse() {
            Ok(parsed_number) => parsed_number,
            Err(_) => {
                println!("This is not number");
                continue;
            }
        };

        // 入力された数字が0~2の数字かどうか判別する
        let user_choice = match input_number {
            0 | 1 | 2 => input_number,
            _ => {
                println!("Please inseret number.(0~2)");
                continue;
            }
        };

        println!("USER: {}", disp_hand(&user_choice));
        println!("CPU : {}", disp_hand(&cpu_choice));

        let results = comp_hands(user_choice, cpu_choice);
        println!("Game Result : {:?}", results);
        break;
    }
}

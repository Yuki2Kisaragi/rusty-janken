use rand::Rng;
use std::io::stdin;

fn disp_hand(num: &i32) -> &'static str {
    match num {
        0 => "Lock", // 本来は不要だが,明示的に定義する
        1 => "Sissor",
        2 => "Paper",
        _ => "None",
    }
}

fn comp_hands(user_hands: i32, cpu_hands: i32) -> GameResult {
    let comp: i32 = (user_hands - cpu_hands + 3) % 3;

    match comp {
        0 => GameResult::Draw,
        1 => GameResult::Lose,
        2 => GameResult::Win,
        _ => GameResult::Null,
    }
}

#[derive(Debug)]
enum GameResult {
    Draw = 0, // 本来は不要だが,明示的に定義する
    Win,
    Lose,
    Null,
}

fn main() {
    // generate number :0~2
    let cpu_choice: i32 = rand::thread_rng().gen_range(0..3);
    //println!("CPU choices: {}", disp_hand(&cpu_choice));

    loop {
        println!("Please input your choice. [0:lock, 1:sissor, 2:paper]");
        let mut input_str = String::new();
        stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        let input_number: i32 = match input_str.trim().parse() {
            Ok(input_number) => input_number,
            Err(_) => {
                println!("This is not number");
                continue;
            }
        };
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

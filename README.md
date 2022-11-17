# CLIでじゃんけんを作ってみよう！


## 出力結果イメージ
完全に同じにする必要はありませんが、下記のようにCPUとじゃんけんするCLIを作ってみてください！

```sh
$ cargo run
Please input your choice. [0:lock, 1:sissor, 2:paper]
0
USER: Lock
CPU : Lock
Draw! Choice next your hand!
Please input your choice. [0:lock, 1:sissor, 2:paper]
1
USER: Sissor
CPU : Paper
Game Result : Win

$ cargo run
Please input your choice. [0:lock, 1:sissor, 2:paper]
1
USER: Sissor
CPU : Paper
Game Result : Win

$ cargo run
Please input your choice. [0:lock, 1:sissor, 2:paper]
2
USER: Paper
CPU : Sissor
Game Result : Lose
```

また、0~2の数字でないor文字の入力を受け付けず、繰り返すようにしてください。

```rust
Please input your choice. [0:lock, 1:sissor, 2:paper]
a
This is not number
Please input your choice. [0:lock, 1:sissor, 2:paper]
10000
Please inseret number.(0~2)
Please input your choice. [0:lock, 1:sissor, 2:paper]
```

## ヒント1：使用するRustの文法
* loop文
* 関数
* 参照
* match
* Enum

## ヒント2：使用クレート
このCLIでは、下記のクレートを扱います。

* rand
  * 乱数を扱う
* std::io
  * 標準入出力


### 乱数からCPUの手を生成する

```rust
// 0~2までの乱数を作る
let cpu_choice: i32 = rand::thread_rng().gen_range(0..3);
```

### 標準入力を受け取る
```rust
let mut input_str = String::new();
stdin()
    .read_line(&mut input_str)
    .expect("Failed to read line");
```

## ヒント3：じゃんけんの勝敗の判定方法

```rust
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
  let comp: i32 = (<ユーザの手> - <コンピュータの手> + 3) % 3;
```
## ヒント4: 標準入力から与えられた文字列を整数に変換する

* input_strがString型の文字列とする
* trim() : スペースを取り除く
* parse(): 整数に変換する
* 変換に成功した場合:整数が格納される
* 変換に失敗した場合:最初からやり直し(`continue`)
```rust
let input_number: i32 = match input_str.trim().parse() {
    Ok(parsed_number) => parsed_number,
    Err(_) => {
        println!("This is not number");
        continue;
    }
};
```

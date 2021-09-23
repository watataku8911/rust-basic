# Rust-basic

## 基本データ型

[参考。](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/fb866b)
Rust の標準にある基本的なデータ型は次のとおりです：

- 整数型: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
- 浮動小数点型: f32, f64
- ブーリアン型: bool
- 文字列型: char
- タプル（複合）型: (500, 6.4, true), () はユニット．
- 配列型: [1,2,3,4,5], [3;5] = [3,3,3,3,3]

数値型のリテラルには次のものが使えます：

- 98_222 (10 進数)
- 0xff (16 進数)
- 0o77 (8 進数)
- 0b1111_0000 (2 進数)
- b'A' (バイト)
- 0. (浮動小数点数)

## 変数・定数(let, mut, const)

変数を宣言するには let を使用しますが、Rust ではイミュータブルな(作成後に変更することができない)オブジェクトとして生成されます。

```
let n = 0;
```

**変更可能**な(ミュータブルな)変数を宣言するには **mut** を使用する必要があります。

```
let mut n = 0;
```

**定数**を定義するには **const** を用います。

```
const MAX_POINTS: u32 = 100_000;
```

## 型変換(as)

暗黙の型変換は行ってくれません。**as** を用いて明示的に型変換します。

```
let x: i32 = 123;
let y: i64 = x as i64;
```

## 関数

[参考。](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/259f24)

関数を定義するには **fn** を使い，本体は {} で囲みます．_引数の型は必ず明記しなければなりません。_
fn は文で， {} は式です。_返り値の型は -> で指定します．_
また， return で処理を中断して値を返すことが出来ます。

```
fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn main() {
    print!("{}", add(10,20));
}
```

関数の引数に対してパターンによる分解束縛をすることが出来ます．

```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("location: ({},{})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

## 制御構文

[参考。](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/80f0ee)

- if 式

if は条件によって処理を分岐するものです． if, else, else if が使えます．それぞれに続くものは式 {} です．

```
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 2");
    }
}
```

if は式なので，値を返せます．ただし，その場合は返す値が同じ型でなければなりません．

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; Error!
    println!("The value of number is: {}", number);
}
```

- loop 式

**無限ループ**するには loop を使います．ループから抜ける場合は break を使います． loop もまた式なので， break に返り値を指定することが出来ます．

```
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

- while 式

条件を満たしている間だけループさせる場合は while 式を使います． while 式は常に () を返します． break は使えますが，値を返すことは出来ません．

```
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    };

    println!("LIFTOFF!!!");
}
```

- for 式

イテレータを使って，各要素に対して処理を行いたい場合は for を使います． for 式は常に () を返します。
**イテレータとは，連続する一連のデータへのアクセスを提供するオブジェクトのことです**

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

## 構造体

[参考。](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/0e7a37)

**構造体** はデータ型の要素を集めたものです．
１つ１つの要素を **フィールド** と呼びます．
構造体の定義は `struct` を使い，_フィールドは名前と型を指定します．_

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

構造体のオブジェクトを作成する場合は，各フィールドを key:value という形で束縛します．

※束縛
_let 文を使うことでオブジェクトと変数を 束縛 します．変数はそのスコープから外れたときに束縛していた所有権を放棄します．また，最初に束縛したオブジェクトの所有権は基本的に原本となり，原本および仮の所有権がすべて放棄された時にオブジェクトは破棄されます．_

```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

- メソッド

**メソッド**は関数に似ていますが，構造体と関連していて， `self` を使うことで，そのメソッドを呼び出したオブジェクトを操作することが出来ます．
メソッドの第一引数は必ず self になります．また，基本的に不変参照 (&self) か可変参照 (&mut self) になります．もちろん，可変参照のメソッドは，呼び出し元が可変の所有権を使って呼び出さなければなりません．
メソッドは `impl` ブロックの中で，関数と同じく fn を使って定義します．

```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50, };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```

_※impl ブロックは複数定義することが出来ます．トレイトごとに実装を分けたりすることが出来ます_

##

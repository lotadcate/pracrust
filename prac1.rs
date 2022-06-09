
// ユーザ定義
enum Gender {
    Male,
    Female
}

enum Role {
    Player(u32, u64),
    Supporter(u32)
}

// インスタンス生成
Person {
    age: 20,
    gender: Gender::Female,
    role: Person::Supporter(70)
}

// ジェネリクス
enum Option<T> {
    Some(T), // 成功なら値を内包
    None // 失敗ならリターン
}

struct Pair<A> {
    first: A,
    second: A
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// let文 変数定義
fn let_example() -> u32 {
    let x = 100;
    let mut y = 20;
    let z: u32 = 5;
    let w;
    y *= x + z; // y が mut なのでOk
    w = 8;
    y + w //　セミコロンなしは戻り値
}

fn hello(v: u32) {
    println!("Hello World!: v = {}", v); // {}内にvが入る
}

fn add(a: u32, b: u32) -> u32{
    a + b
}

fn my_func1() {
    let n = add(4, 5);
    hello(n);
}

// if式 必ずbool 式なので値をreturn
fn is_even(v: u32) {
    if v%2 == 0 {
        true
    } else {
        false
    }
}

// match式
fn print_pred(v: u32) {
    match pred(v) {
        Some(w) => { // Some()につつまれた値をwに代入
            println!("pred({}) = {} ", v, w);
        }
        None => {
            println!("pred() is undefind", v);
        }
    }
}

// for文
fn even_odd() {
    for n in 0..10 {
        println!("{} is {}", n, if is_even(n) {"even"} else {"odd"});
    }
}

// loop文
fn even_odd() {
    let mut n = 0;
    loop {
        println!("{} is {}", n, if is_even(n) {"even"} else {"odd"});
        n += 1;
        if n >= 10 {
            break;
        }
    }
}

// 参照はずし
fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y
}

fn my_func2() {
    let mut n = 10;
    let m = 20;
    println!("n = {}, m = {}", n, m);

    mul(&mut n, &m);

    println!("n = {}, m = {}", n, m);

}

//関数ポインタ

/*
参照とポインタは別
参照... 所有権、ライフタイムによって安全性の確保
ポインタ.. その限りでないf
*/
fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64{
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

fn mul2(x: u64) -> u64 {
    x*2
}

fn my_func3() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}

// クロージャ 関数のこと
/*
|y| x*y　この部分がクロージャ
move は所有権の移動
なぜmove必要か ...  xはmul_xを抜けた時点で破棄されるから

y は束縛変数　（引数に現れているため
x は自動変数
xは所有権移動によりキャプチャされる　という

Box コンテナの一種, ヒープ上にデータを配置したいとtき
Box スマートポインタの一種でもある, スコープから抜けると確保されたデータは自動的に破棄される
dyn そのトレイトの振る舞いが同的に決まることを表している

Box::<dyn Fn(u64) -> u64>
ヒープ上に確保された関数とデータへのポインタ を持つクロージャへのスマートポインタ
*/

fn mul_x(x :u64) -> Box::<dyn Fn(u64) -> u64> {
    Box::now(move |y| x*y);
}

fn my_func4() {
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
}

// ライフたいmう
struct Foo {
    val: u32
}

fn add<'a>(x: &'a Foo, y:&'a Foo) -> u32 {
    x.val + y.val
}

fn main() {
    let x = Foo{val: 10}; // 10行目 ~ 16行目まで
    {
        let y = Foo{val: 20}; // 12行目 ~ 15行目まで
        let z = add(&x, &y);
        println!("z = {}", z); //ライフタイムの範囲の小さい方に合わせる
    }
}

// 借用 ... 参照の生成

/*
参照の生成 ...
はじめに生成されるのは mutable変数
そこから & や　&mutでimmutableと mutable参照を生成する

mutable 変数, 参照
immutable変数, 参照


*/

struct Foo {
    val: u32
}

fn my_func8() {
    let mut x = Foo{val: 10};
    {
        let a = &mut x; // &mut

        // ok
        println!("a.val : {}", a.val);
        // ng
        println!("x.val: {}", x.val);


        let b: &Foo = a; // immutable
        //ng
        a.val = 20;

        println!("b.val: {}", b.val);

        // ok なぜできる
        a.val = 30;

    }
}

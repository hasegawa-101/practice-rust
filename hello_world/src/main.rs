const POINT: u32 = 100_000;
// 100_000を100000としても10_0000としてもエラーは出ない（？）

fn main() {
    println!("Hello, world!");
    //    let x = 5;
    //    x = 10;
    //    変数はimmutableのため上記はエラーになる。
    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{},", x);

    println!("ポイントは {} です。", POINT);

    println!("{}", usize::BITS);
    println!("メモリアドレスの番地は {:p} です。", &POINT);

    let i1: u64 = 1;
    let i2: u64 = 2;

    println!("スタックアドレスは {:p} です。", &i1);
    println!("スタックアドレスは {:p} です。", &i2);
    //    結果
    //    スタックアドレスは 0x16bdaade0 です。
    //    スタックアドレスは 0x16bdaade8 です。
}

fn main() {
    println!("Hello, world!");
    //    let x = 5;
    //    x = 10;
    //    変数はimmutableのため上記はエラーになる。
    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{},", x);
}

fn main() {
//    Can't compile
//    cannot assign twice to immutable variable
//    let x = 5;

    let mut x = 5;    // mut keyword delcare mutable varialbe 
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);


//    const example
//    const MAX_POINTS: u32 = 100_000;    //가독성 향상을 위한 자릿수 구분     
}

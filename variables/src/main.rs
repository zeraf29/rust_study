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


    let y = 5;
    
    let y = y + 1;   //shadowed y

    let y = y * 2;

    println!("y 값: {}", y);


    let spaces = "    ";
    let spaces = spaces.len(); // spaces와 이름은 같지만, 숫자 타입의 새로운 변수
    println!("spaces: {}", spaces);

    
//    let mut spaces = "    ";
//    spaces = spaces.len();   // 컴파일 에러, mismatched types. let mut 로 선언한 변수는 let 처럼 다시 선언하여 shadwed 할수 없다.  


//    let guess: u32 = "42".parse().expect("숫자가 아닙니다!"); 
//    // 다른 데이터 타입으로 변환 시, 타입 애노테이션 annotation 을 이용해 타입을 명시해야 함. 
}

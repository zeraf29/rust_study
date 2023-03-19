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


    //부동소수점 타입
    // let var_x = 2.0;             //f64
       
    // let var_y: f32 = 3.0;        // f32


    //덧셈 
    let sum = 5 + 10;

    //뺄셈 
    let difference = 95.5 - 4.3;

    //곱셈
    let product = 4 * 30;

    //나눗셈
    let quotient = 56.7 / 32.2;

    //나머지
    let remainder = 43 % 5;



    //Boolean
    let t = true;

    let f: bool = false; //Type annotation 사용


    //Character 
    // 문자열 리터럴은 큰따옴표, char 리터럴은 작은따옴표
    // 형식 chartset 별 유니코드 값은 다를 수 있으므로, 실제 표시 값은 다를 수 있음

    let c = 'z';
    let z = 'z';
    //let heart_eyed_cat = '';



    // 컴파운드 타입(compound type): 하나의 타입으로 여러개의 값을 그룹화한 타입 
    // 기본적으로 튜플(tupes)과 배열(arrays)등 두 가지 컴파운드 타입을 지원 

    // 튜플 타입 
    // 서로 다른 타입의 여러 값을 하나의 컴파운드 타입으로 그룹화 하기에 적합한 타입
    // 고정된 길이를 가짐. 한번 정의하면 그 크기를 재정의 못함 
    // 튜플의 각 요소는 타입을 가지며, 반드시 같을 필요는 없다. 선택적으로 타입 애노테이션 적용 가능 
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y 값: {}", y);


    let tup3 = (500, 6.4, 1);
    let five_hundred = tup3.0;
    let six_point_four = tup3.1;
    let one = tup3.2;
    println!("x, y, z: {}, {}, {}", five_hundred, six_point_four, one);
   

    // 배열 타입
	// 반드시 각 요소는 같은 타입이어야 한다. 
	// 고정 길이
	
	let array_1 = [1, 2, 3, 4, 5];
    
	// 배열은 데이터를 heap 이 아닌 stack 메모리에 할당하거나, 항상 고정된 개수의 요소들을 다룰 때 유용 
	// 하지만 배열은vector 만큼 유연하지는 않다. (벡터는 배열과 유사하나, 크기 조절이 자유롭다.)
	// 배열과 벡터 사용 여부가 모호할땐, 벡터 사용이 더 낫다.

    // 배열 타입은 대괄호[]를 이용해 작성하며, 괄호 안에는 각 원소의 타입과 세미콜론, 그리고 배열 저장 원소 개수를 지정
	let array_2: [i32; 5] = [1, 2, 3, 4, 5];
	//i32: 타입, 5는 배열 사이즈 

	// 배열 초기화 시 초기값, 사이즈 선언은 아래와 같이 진행
	let a = [3; 5];

	//배열 = 스택에 할당된 한 덩어리 메모리. 
	let first = array_1[0];
	let second = array_1[1];

	let index = 10;
	let element = array_2[index];
	// error 발생. index out of bounds
	//println!("요소의 값: {}", element);

    
}

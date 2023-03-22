fn main() {
    println!("Hello, world!");

	another_function();

	another_function_1(10);

	another_function_2(15, 30);

    another_function_3();

    let five = five();

	println!("함수 five를 통한 x의 값: {}", five);

	let plus_one = plus_one(5);
	println!("함수 plus_one을 통한 x의 값: {}", plus_one);
}


//Rust는 함숭 명명에 snake_case 를 사용
fn another_function() {
    println!("또 다른 함수"); 
}

//함수 시그니처에는 각 매개변수의 타입을 명시해야 한다. 
fn another_function_1(x: i32){
	println!("x의 값: {}", x);
}

fn another_function_2(x: i32, y: i32){
	println!("x의 값: {}", x);
	println!("y의 값: {}", y);
}


// 함수 본문 = 여러개의 구문 + 선택적 표현식
fn another_function_3(){
	//에러 발생
	//let x = (let y = 6); //구문은 값을 리턴하지 않으므로, let y = 6 구문을 다른 변수에 대입 할 수 없다. 
	//C, Ruby 와는 다르다!! 
	

	//표현식은 어떤 값으로 평가 된다. 
	//표현식은 구문의 일부가 될 수 있다. 
	let x = 5;
	let y = {
		let x = 3; 
		x + 1
	};

	println!("x 의 값:{}, y의 값:{}", x, y);
}

// 리턴값에는 이름을 부여하지는 않지만, 리턴할 값의 타입은 화살표(->) 다음에 지정
fn five() -> i32 {
	//세미콜론 없이 리턴할 값만 작성
	5
}

// 함수에서 값을 전달(리턴) 하려면 세미콜론 없이 작성 종료 하거나 return ; 형태로 작성 
fn plus_one(x: i32) -> i32 {
	//return 없이 x+1; 로 하면 에러
	//x+1;

	//return ; 형태로 리턴값 지정하거나
	//return x + 1; 

	//return과 ; 없이 리턴할 값만 지정해서 반환값 명시
	x+1
}

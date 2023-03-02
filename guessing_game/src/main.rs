use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The number user has to answer: {}", secret_number);

    loop {

        println!("Please enter what you think correct answer.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
		.expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Entered number: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Entered number is less than secret_number"),
      	    Ordering::Greater => println!("Entered number is bigger than secret_number"),
      	    Ordering::Equal => {
                println!("정답!");
                break;
            }
        }    
    }      
}

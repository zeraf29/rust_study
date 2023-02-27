use std::io;
use rand::Rng;

fn main() {
   println!("숫자를 맞혀봅시다!");
   println!("Please enter the answer number what you think");

   let secret_number = rand::thread_rng().gen_range(1, 101);

   println!("The number user has to answer: {}", secret_number);

   println!("Please enter what you think correct answer.");

   let mut guess = String::new();
   io::stdin().read_line(&mut guess)
	.expect("입력한 값을 읽지 못했습니다. Cannot read a entered number");

   println!("Entered number: {}", guess);

      

}

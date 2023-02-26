use std::io;

fn main() {
   println!("숫자를 맞혀봅시다!");
   println!("Please enter the answer number what you think");

   let mut guess = String::new();
   io::stdin().read_line(&mut guess)
	.expect("입력한 값을 읽지 못했습니다. Cannot read a entered number");

   println!("Entered number: {}", guess);
}

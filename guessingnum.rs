use std::io; // input and output standard library 
use rand::Rng;
use std::cmp::Ordering;
fn main(){
  println!("guess the number");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  loop{

  println!("please input your guess");
  let mut guess = String::new();

  io::stdin().read_line(&mut guess).expect("failed to read the line");
  let guess : u32 = match guess.trim().parse(){
    Ok(num)=>num,
    Err(_) => {
      println!("please enter valid input ");
      continue;
    },
  };

  
  match guess.cmp(&secret_number) {
    Ordering::Equal => {
      println!("You win!");
      break;
    },
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    
  }
}
}


// output of the guessing the number 
/*$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win! 

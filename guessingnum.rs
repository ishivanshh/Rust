use std::io;
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

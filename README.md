 Guessing ther game of number with user and computer with the help of rust programming languages......

1.Setting up the new project 

>>cargo new guessing_game
>>cd guessing_game

The first command, cargo new, takes the name of the project (guessing_game) as the first argument. The second command changes to the new project’s directory.

.. cargo file is generated (Cargo.toml file)

2.Reopen the src/main.rs file. You’ll be writing all the code in this file.

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess. 

>>Filename: src/main.rs

use std::io;
fn main(){
    println!("guess the number!");
    println!("please input your guess.");
    
    let mut guess = String::new();
    io:::stdin().read_line(&mut guess).expect("failed to read this");

    println("you guessed: {}",guess);
}

// To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std: >> use std::io;

we’ll create a variable to store the user input,
>> let mut guess = String::new(); // mutable data type

::new line indicates that new is an associated function of the string type. This new function creates new and empty string...
The let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String.

3. Receiving the user input
we’ll call the stdin function from the io module, which will allow us to handle user input.
>> io::stdin().read_line(&mut guess)
 The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

4. Potential failure with the result.
>> io::stdin().read_line(&mut guess).expect("Failed to read line");
The purpose of these Result types is to encode error-handling information.

Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

5. Generating a secret numbers.
crate is a collection of Rust source code files.
Filename: Cargo.toml
[dependencies]
rand = "0.8.5"

Generating a Random Number.
Let’s start using rand to generate a number to guess.
>> use rand::Rng;

Filename: src/main.rs

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// the rand::thread_rng function that gives us the particular random number generator.
The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.

6. Comparing the guess to the secret number

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

first,
      use statement, bringing a type called std::cmp::Ordering into scope from the standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

// the cmp method compares two values and can be called on anything that can be compared. it’s comparing guess to secret_number. Then it returns a variant of the Ordering enum we brought into scope with the use statement. We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.

match expression example => user guessed = 50 | secreat guessed = 38.
When the code compares 50 to 38, the cmp method will return Ordering::Greater because 50 is greater than 38. The match expression gets the Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is Ordering::Greater, which does match Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The match expression ends after the first successful match, 

// The core of the error states that there are mismathces types. rust have a strong, static type system. we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type. The secret_number, on the other hand, is a number type.
Ultimately, we want to convert the String the program reads as input into a number type so we can compare it numerically to the secret number. We do so by adding this line to the main function body:

   // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
 Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables.
>> guess.trim().parse()

guess refers => original guess variable that contained the input as a string.
trim method on string => will eliminate any whitespace at the beginning and end, which we must do before we can convert the string to a u32, which can only contain numerical data.
The trim method eliminates \n or \r\n , resulting in just 5 or numerical value by eliminating the whitespace found during read_line method.
parse method on strings => converts string to another type.

8. Allowing multiple guesses with loopimg.

the loop keyword creates an infinite loop.
// // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

9. quitting after a correct guesss.
   // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
Adding the break line 

?????

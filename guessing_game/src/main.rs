use std::io; //brings io(input/output) library into scope so we can obtain and print user input 
             //the io library comes from the standard library(std)
use rand::prelude::*; //prelude module contains the most commonly used parts of the rand crate
                     //use makes the commonly used parts available in our program scope
use std::cmp::Ordering; //bring Ordering enum into scope from the standard library
                        //this enum has the variants Less,Greater,Equal when you compare two values

fn main(){
    println!("Guess the number!"); //use macro to print message

    let secret_number = rand::rng().random_range(1..=100); //call rand::rng function and then call random_range
                                                           //which is a method brought into scope with the use rand::prelud::* statement
                                                           //random_range takes range in the form of start..=end, both numbers being inclusive
                                                           //so we do 1..=100 to generate a number from 1-100 inclusive on both ends

loop { //create loop process to repeatedly ask for guess until correct answer
    println!("Please input your guess.");

    let mut guess = String::new(); //create mutable variable(variables are immutable by default)
                                   //String::new() bounds guess to new empty instance of a String
    io::stdin()
        .read_line(&mut guess) //reads one line from stdin and appends it to guess
        .expect("Failed to read line"); //read_line returns a Result (an enum: Ok or Err).
                                        //expect unwraps Ok(n) to get n, the bytes read,
                                        //which is discarded here since nothing uses it.
                                        //On Err, it panics, showing this message plus the error.
                                        //if we don't have .expect, the program will compile but we get a warning
                                        //since we haven't handled a possible error

//whole block above could be written as just one line:
// io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() { //this line reuses the same guess String variable defined previously and does multiple things
        Ok(num) => num,
        Err(_) => continue,
    }; 
    //trim on a String gets rid of the newline char and whitespace, which must happen before conversion to u32 type(unsigned 32 bit number)
    //parse on Strings converts a String to another type, here we use it to convert to a number of type u32
    //we use what's called a match expression(same thing is used below to compare numbers)
    //the match expression handles errors since .parse returns a Result enum and so we can take actions depending on what Result is
    //if the string to number conversion works(Ok), we just return num and guess is that number
    //if it doesn't work(Err, with the _ being a catchall for all possible Errs), continue makes it so that the loop restarts and asks the user for an input again until they input something valid

        println!("You guessed: {guess}"); //print the user input

        match guess.cmp(&secret_number){ //compare guess to secret_number and uses Ordering enum. 
            Ordering::Less => println!("Too small!"), //these 3 comparisons are seeing if guess is less/greater/equal to the secret number
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { //put the 2 lines of code inside brackets, since it's not just one line anymore
                println!("You win!");
                break; //break out of loop when correct answer is inputted
            }
        }
    }

}
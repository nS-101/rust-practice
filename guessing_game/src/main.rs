use std::io; //brings io(input/output) library into scope so we can obtain and print user input 
             //the io library comes from the standard library(std)


fn main(){
    println!("Guess the number!"); //use macro to print message

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


    println!("You guessed: {guess}")
}
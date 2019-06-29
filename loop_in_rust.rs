
use std::io;


fn main() { 

             loop 
                {
                println!("guess number");
                println!("enter your guess number");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess)
                .expect("failed");
                println!("guess    {} : ",guess);
               }	     
                  
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game");
    let secret_number: i32 = rand::thread_rng().gen_range(1,101);
    
    loop{
        let mut guess:String = String::new();

        io::stdin()
            .read_line( &mut guess)
            .expect("Failed to read line");
            let guess_num: i32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,

            };
        match guess_num.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win ");
                break;
            }
        }
    }
}

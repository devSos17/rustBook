use rand::random;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num: u8 = random::<u8>();
    println!("Guessing game=================================================");

    println!("Guess a number between 0-255:");

    let mut intentos: u8 = 0;
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Valio kk");
        println!("Guess: {buff}");
        let guess: u8 = match buff.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No mms, no's  un numero we");
                println!("Vuelve a intentar prro stupido");
                continue;
            }
        };
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("pp Smoll"),
            Ordering::Greater => println!("pp BIG"),
            Ordering::Equal => {
                println!("Jackpot, en {intentos}");
                break;
            }
        }
        intentos += 1;
        if intentos > 10 {
            println!("MMSTE stas pendejo, era: {rand_num}");
            return;
        }
    }
}

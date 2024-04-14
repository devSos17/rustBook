use rand::random;
use std::cmp::Ordering;
use std::io;

const DEBUG: bool = false;
const RANGE: u8 = 100; // from 0 -> RANGE (lower than 255)

pub fn game() {
    // GET A SECRET NUMBER
    let secret: u8 = (random::<f64>() * RANGE as f64).round() as u8;

    println!("Atinale pndjo game=================================================");

    println!("Entre 0 y {RANGE}:");

    let mut intentos: u8 = 0;

    loop {
        let guess = match guess() {
            Ok(num) => num,
            Err(e) => {
                println!("Error:{e}");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("pp Smoll"),
            Ordering::Greater => println!("pp BIG"),
            Ordering::Equal => {
                println!("Jackpot, en {intentos} intentos");
                break;
            }
        }
        intentos += 1;
        if intentos > 10 {
            println!("MMSTE stas pendejo, era: {secret}");
            return;
        }
    }
}

fn guess() -> Result<u8, String> {
    let mut buff = String::new();
    match io::stdin().read_line(&mut buff) {
        Ok(n) => {
            if DEBUG {
                println!("{n} bytes read");
                println!("{buff}");
            }
        }
        Err(error) => println!("error: {error}"),
    }

    if DEBUG {
        println!("Guess: {buff}")
    }

    match buff.trim().parse::<u8>() {
        Ok(num) => {
            if num > RANGE {
                return Err(format!("Entre 0 y {RANGE} PENDAJO"));
            }
            Ok(num)
        }
        Err(_) => Err(format!(
            "No mms, no's  un numero we, entre 0 y {RANGE}\n Vuelve a intentar prro stupido"
        )),
    }
}

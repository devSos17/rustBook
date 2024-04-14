mod fibonacci;
mod guess_game;
mod temp_converter;

fn main() {
    println!(" Guessing game =======================");
    guess_game::game();
    println!(" Fibonnaci ===========================");
    for i in 1..10 {
        fibonacci::fib(i);
    }
    println!(" Convert C to F ======================");
    temp_converter::converter();
}

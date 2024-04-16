mod fibonacci;
mod guess_game;
mod structs;
mod temp_converter;

fn guessing_game() {
    println!(" Guessing game =======================");
    guess_game::game();
}
fn fib() {
    println!(" Fibonnaci ===========================");
    let mut cuenta: usize = 0;
    for i in 1..=30 {
        let (f, fc) = fibonacci::fib(i);
        // let f = fibonacci::fib_ciclo(i, &mut cuenta);
        println!("Fib de {i} es {f}");
        cuenta += fc;
    }
    println!("{}", cuenta);
}

fn converter() {
    println!(" Convert C to F ======================");
    temp_converter::converter();
}

fn structs() {
    let mut s: structs::User = structs::User::create("devSos17".to_string(), "sos".to_string());
    println!("User id1: {:#?}", s);
    println!("User id2: {:#?}", s.recreate_id());
    println!(
        "Update email => {:#?}",
        s.update_mail("sos@email.com".to_string())
    );
}

// fn

fn main() {
    todo!() // TODO: IMPLEMENT A MATCH TO SELECT PROGRAM
}

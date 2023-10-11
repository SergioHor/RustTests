use rand::Rng;

fn main() {
    let num_random: u8 = rand::thread_rng().gen_range(1..=10);
    println!("Ingresa un Num del 1 al 10:");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num_guessed: u8 = buf.trim().parse().expect("LOCO PONE BIEN LAS COSAS");
    if num_guessed > 10 || num_guessed < 1 {
        panic!("LOCO PONE BIEN LAS COSAS")
    }
    println!("El Num es: {num_random}");
    if num_random == num_guessed {
        //num_random == num_guessed ó rand::random::<u8>() == num_guessed
        println!("Congratz! :D")
    } else {
        println!("BOOOOOO")
    }
}

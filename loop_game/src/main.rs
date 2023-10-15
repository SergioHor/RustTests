fn main() {
    let mut main_array = [0u128; 10];
    loop {
        println!("Pick between:");
        println!("1. Set array value");
        println!("2. Show array value");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let chosen_num: u8 = buf.trim().parse().unwrap();
        if chosen_num == 1 {
            println!("Position:");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let position: usize = buf.trim().parse().unwrap();
            println!("Set Number:");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let number = buf.trim().parse().unwrap();
            main_array[position] = number;
        }
        if chosen_num == 2 {
            println!("Position:");
            //different but same approach buf = String::new();
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let position: usize = buf.trim().parse().unwrap();
            println!("{}", main_array[position]);
        }
    }
}

/*
Definir array de 10 u128s
Leer en loop el input del usuario, preguntar si:
si elige 1
preguntar posicion
preguntar numero escribir
escribir en posicion numeroingresado

si elige 2
preguntar posicion
printear posicion ingresada
*/

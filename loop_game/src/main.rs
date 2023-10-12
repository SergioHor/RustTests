fn main() {
    let main_array: [u128; 10] = [0; 10];
    println!("Choose 1 or 2:");
    loop{
        let mut buf = String::new();
        let chosen_num: u8 = buf.trim().parse().unwrap();
        std::io::stdin().read_line(&mut buf).unwrap();
            if chosen_num == 1{
        println!("Which position do you want?:");
        println!("Which number do you want to show?:");
        println!("Number:{} & Position:{}");
    }
        if chosen_num == 2{
        println!("Which position do you want?:");
        println!("{}");
    }
}
}

/*
Definir array de 10 u128s
Leer en loop el input del usuario, preguntar si:
leer array
printear posicion array
si elige 1
preguntar posicion
preguntar numero escribir
escribir en posicion numeroingresado

si elige 2
preguntar posicion
printear posicion ingresada
*/
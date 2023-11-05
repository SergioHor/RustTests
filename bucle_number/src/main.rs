fn main() {
    loop {
        println!("Choose Number:");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let number: u128 = buf.trim().parse().unwrap();
        let mut x = 1;
        loop {
            let mut str = "".to_string();

            if x > number {
                break;
            }
            if x % 3 == 0 {
                str += "Fizz";
            }
            if x % 5 == 0 {
                str += "Buzz";
            }

            if str.is_empty() {
                println!("{}", x);
            } else {
                println!("{str}");
            }
            x += 1;
            // free
        }
    }
}

// empieza el programa y pide un numero para terminar u128
// empezando por 1
// terminando en el numero ingresado
// tiene que pasar por todos los numeros
//  si el num es divisible por 3 imprimir "fizz"
//  si el num es divisible por 5 imprimir "buzz"
//  si el num es divisible po ambos 3 y 5 imprimir "fizzbuzz"
//  si no se cumple ninguno de los anteriores imprimir el numero

// recorrer todos los num usando: Loop, While, For

// Cosas que podes hacer mal con el malloc (memory allocation) /free

// 1 no hacer el free despues de un malloc (ej: break antes de llamar a un malloc) - no es problema de seguridad, pero si un problema de uso de recursos
// 2 use after free - problema de seguridad (ub)
// 3 free after free - problema de seguridad (undefined behavior)

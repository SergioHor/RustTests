fn main() {
    loop {
        println!("Choose Number:");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let number: u128 = buf.trim().parse().unwrap();
        for x in 1..number {
            while x % 3 == 0 && x % 5 == 0 {
                println!("FizzBuzz");
                break;
            }
            if x % 3 == 0 {
                println!("Fizz")
            } else if x % 5 == 0 {
                println!("Buzz")
            } else {
                println!("{}", x)
            }
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

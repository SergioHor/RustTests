fn main() {
    let xfulano = (3, 'Y', 10);
    let resultado = xfulano.0 + xfulano.2;
    println!("{}, {}", resultado, xfulano.0);

    let (x, _y, _z) = xfulano;
    println!("{}, {x}", xfulano.0);

    let mut sarasa = ['🦆'; 6];
    sarasa[4] = 'Y';
    //println!("{} {}", sarasa[4], sarasa[10]);

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let buf: usize = buf.trim().parse().unwrap();
        println!("{}", unsafe { sarasa.get_unchecked(buf) })
    }
}

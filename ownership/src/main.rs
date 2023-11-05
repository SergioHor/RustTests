fn main() {
    // let mut x = 10;
    // let y = x;
    // x += 1;
    // println!("{x}");
    // println!("{y}");

    let mut s1 = String::new();

    s1.push_str("blah");

    let s1 = {
        let mut s2 = s1;
        s2.push_str("bvl");
        println!("{s2}");
        s2
    };
    let s1 = foo(s1);
    println!("{s1}");
}

fn foo(mut x: String) -> String {
    println!("{x}");
    x.push_str("sukudu");
    x
}

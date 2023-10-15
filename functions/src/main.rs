fn main() {
    /*
            println!("Hello, world!");

            second_function();

            new_function(2);

        println!("{}", return_function(6));
        println!("{}", return_function(5));


        let mut counter = 0;

        let result = loop {
            counter += 1;
            println!("{counter}");
            if counter == 15 {
                break counter + 5;
            }
        };
        println!("{result}");

        println!("");

        for counter in 0..=15 {
            for count in 0..40 {
                println!("{counter}, {count}");
            }
        }
    */
    let mut counter = 0;
    while counter < 15 {
        counter += 1;
        //Continue let you skip
        if counter == 13 {
            continue;
        }
        println!("{counter}");
    }
}

fn second_function() {
    println!("Hello");
}

fn new_function(x: u8) {
    println!("{}", x + 1);
}

fn return_function(x: u8) -> u64 {
    if x == 6 {
        //return interrupts the flow of the function & return expression
        return (x + 2).into();
    }
    (x + 10).into()
}

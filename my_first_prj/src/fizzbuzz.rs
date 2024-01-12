fn main() {
    let s1 = "Fizz";
    let s2 = "Buzz";
    let mut i = 1;
    loop {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}{}", s1, s2),
            (0, _) => println!("{}", s1),
            (_, 0) => println!("{}", s2),
            (_, _) => println!("{}", i),
        }
        i += 1;
        if i == 100 {
            break;
        }
    }
}

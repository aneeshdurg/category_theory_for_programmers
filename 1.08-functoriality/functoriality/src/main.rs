fn bimap_pair<A, B, C, D>(
    f: Box<dyn Fn(A) -> C>, g: Box<dyn Fn(B) -> D>, pair: (A, B)) -> (C, D)
{
    return (f(pair.0), g(pair.1));
}

fn main() {
    // bimap: (char -> u8) -> (u8 -> char) -> (char, u8) -> (u8, char)
    println!("{:?}", bimap_pair(
            Box::new(|x| x as u8), // f: char - > u8
            Box::new(|s| s as char), // g: u8 -> char
            ('A', 65 as u8))); // pair: (char, u8)
}

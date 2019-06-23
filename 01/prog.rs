pub fn id <T>(x: T) -> T
{
    return x;
}

#[derive(Debug)]
struct Example<'a> {
    x: i32,
    a: &'a str,
}

pub fn comp<A, B, C> (
    f: Box<dyn Fn(A) -> B>,
    g: Box<dyn Fn(B) -> C>) -> impl Fn(A) -> C
{
    move |a| g(f(a))
}

fn main() {
    println!("{}", id(1));
    println!("{}", id('a'));
    println!("{}", id("a"));
    println!("{}", id("a"));
    let e = Example {
        x: 1,
        a: "1234",
    };
    println!("{:?}", id(e));

    let f = |x| x as char;
    let g = |x: char| format!("Result is: {}.", x);
    println!("{}", (comp(Box::new(f), Box::new(g)))(65 as u8));
    println!("{} == \"Result is: b.\"", (comp(Box::new(id), Box::new(g)))('b'));
    println!("{} == 'A'", (comp(Box::new(f), Box::new(id)))(65 as u8));
}

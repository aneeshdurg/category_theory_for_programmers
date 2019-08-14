extern crate rand;

use rand::Rng;

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub fn memoize<T, U> (
    mut f: Box<dyn FnMut(T) -> U>) -> impl FnMut(T) -> U
    where T: Hash + Eq + Copy + Debug,
          U: Copy + Debug
{
    let mut map: HashMap<T, U> = HashMap::new();
    move |x| {
        match map.get(&x) {
            Some(val) => *val,
            None => {
                let val = f(x);
                map.insert(x, val);
                val
            }
        }
    }
}

pub fn factorial(x: u32) -> u32{
    eprintln!("Called factorial!");
    let mut res = 1;
    for i in 1..x {
       res *= i;
    }
    res
}

pub fn challenge_1() {
    let mut mem_factorial = memoize(Box::new(factorial));
    println!("10! = {}", factorial(10));
    println!("10! = {}", factorial(10));

    println!("10! = {}", mem_factorial(10));
    println!("10! = {}", mem_factorial(10));
}

pub fn challenge_2() {
    let mut rng_1 = rand::thread_rng();
    let mut rng_2 = rand::thread_rng();
    let mut random = move |_x| rng_1.gen::<i32>();
    let random_2 = move |_x| rng_2.gen::<i32>();
    let mut mem_random = memoize(Box::new(random_2));

    println!("random(0) = {}", random(0));
    println!("random(0) = {}", random(0));
    println!("mem_random(0) = {}", mem_random(0));
    println!("mem_random(0) = {}", mem_random(0));

    println!("memoized random generation does not work");
}

pub fn challenge_3() {
    let random = move |x: usize| {
        let seed: &[_] = &[x];
        let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);
        rng.gen::<i32>()
    };
    println!("random() = {}", random(0));
    println!("random() = {}", random(1));

    let mut mem_random = memoize(Box::new(random));
    println!("mem_random() = {}", mem_random(0));
    println!("mem_random() = {}", mem_random(1));

    println!("memoized seeded generation works!");
}

pub fn challenge_5() {
    println!("There are four functions:");
    fn bool_00(_: bool) -> bool {
        return false;
    }

    fn bool_01(x: bool) -> bool {
        // not
        return !x;
    }

    fn bool_10(x: bool) -> bool {
        // id
        return x;
    }

    fn bool_11(_: bool) -> bool {
        return true;
    }

    println!("t -> {}, f -> {}", bool_00(true), bool_00(false));
    println!("t -> {}, f -> {}", bool_01(true), bool_01(false));
    println!("t -> {}, f -> {}", bool_10(true), bool_10(false));
    println!("t -> {}, f -> {}", bool_11(true), bool_11(false));
}

pub fn main() {
    println!("--- Challenge1");
    challenge_1();
    println!("---");

    println!("--- Challenge2");
    challenge_2();
    println!("---");

    println!("--- Challenge3");
    challenge_3();
    println!("---");

    println!("--- Challenge5");
    challenge_5();
    println!("---");

}

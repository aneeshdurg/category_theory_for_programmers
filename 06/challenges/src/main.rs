use std::f32;

#[derive(Debug)]
enum Maybe<A> {
    Just(A),
    Nothing,
}

#[derive(Debug)]
enum Either<A, B> {
    Left(A),
    Right(B),
}

fn either_to_maybe<A> (e: Either<A, ()>) -> Maybe<A> {
    match e {
        Either::Left(a) => Maybe::Just(a),
        _ => Maybe::Nothing,
    }
}

fn maybe_to_either<A> (m: Maybe<A>) -> Either<A, ()> {
    match m {
        Maybe::Just(a) => Either::Left(a),
        _ => Either::Right(()),
    }
}

enum Shape {
    Circle(f32),
    Rect(f32, f32),
    Square(f32), // challenge 4
}

impl Shape {
    fn area(s: Shape) -> f32{
        match s {
            Shape::Circle(r) => std::f32::consts::PI * r * r,
            Shape::Rect(l, w) => l * w,
            Shape::Square(l) => l * l, // challenge 4
        }
    }

    // Challenge 3
    fn circ(s: Shape) -> f32 {
        match s {
            Shape::Circle(r) => 2.0 * std::f32::consts::PI * r,
            Shape::Rect(l, w) => 2.0 * (l + w),
            Shape::Square(l) => 4.0 * l, // challenge 4
        }
    }
}


fn main() {
    println!("{:?}", maybe_to_either(either_to_maybe(Either::Left(0))));
    let b:Either<i32, ()> = Either::Right(());
    println!("{:?}", maybe_to_either(either_to_maybe(b)));

    println!("{:?}", either_to_maybe(maybe_to_either(Maybe::Just(0))));
    let b:Maybe<i32> = Maybe::Nothing;
    println!("{:?}", either_to_maybe(maybe_to_either(b)));
}

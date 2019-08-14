enum Either<A, B> {
    Left(A),
    Right(B),
}

fn main() {
    let _x: Either<i32, bool> = Either::Left(0);
    let _y: Either<i32, bool> = Either::Right(false);
}

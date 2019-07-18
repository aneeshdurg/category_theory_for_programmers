type Reader<T, R> = Box<dyn FnMut(T) -> R>;

fn reader_fmap<A: 'static, B: 'static, T: 'static>(
    mut f: Box<dyn FnMut(A) -> B>, mut val: Reader<T, A>) -> Reader<T, B>
{
    Box::new(move |x| {
        return f(val(x))
    })
}

fn main() {
    // TODO test reader_fmap
}

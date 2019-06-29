pub trait Monoid {
    fn empty() -> Self;
    fn add(a: &Self, b: &Self) -> Self;
}

impl Monoid for i32 {
    fn empty() -> i32 { 0 }
    fn add(a: &i32, b: &i32) -> i32 { *a + *b }
}

impl Monoid for String {
    fn empty() -> String { "".to_string() }
    fn add(a: &String, b: &String) -> String { format!("{}{}", a, b) }
}


#[derive(
    Copy,
    Clone,
    Debug
)]
pub struct Writer<U: Copy, X: Monoid> {
    pub data: U,
    pub state: X,
}

impl<U: Copy, X: Monoid> Writer<U, X> {
    pub fn new (data: U) -> Self {
        Writer {
            data,
            state: X::empty(),
        }
    }

    pub fn from_both (data: U, state: X) -> Self {
        Writer {
            data,
            state,
        }
    }

    pub fn set_state(&mut self, state: X) {
        self.state = state;
    }

    pub fn append(&mut self, to_write: &X) {
        self.state = Monoid::add(&self.state, to_write);
    }

    pub fn set_data(&mut self, data: U) {
        self.data = data;
    }
}

pub fn compose_kleisli<T, U, V, X> (
    mut f: Box<dyn FnMut(T) -> Writer<U, X>>,
    mut g: Box<dyn FnMut(U) -> Writer<V, X>>) -> impl FnMut(T) -> Writer<V, X>
    where X: Monoid,
          U: Copy,
          V: Copy,
{
    move |x| {
        let w = f(x);
        let w_1 = g(w.data);
        let mut ret_w = Writer::from_both(w_1.data, w.state);
        ret_w.append(&w_1.state);
        return ret_w;
    }
}

pub fn kleisli_id<U, X> (u: U) -> Writer<U, X>
    where U: Copy,
          X: Monoid,
{
    Writer::from_both(u, X::empty())
}

pub fn safe_reciprocal(x: Option<f32>) -> Writer<Option<f32>, String> {
    match x {
        Some(num) => {
            if num == 0.0 {
                return Writer::from_both(
                    None, "safe_reciprocal: Argument was 0!\n".to_string())
            }

            kleisli_id(Some(1.0 / num))
        },
        None => {
            kleisli_id(None)
        }
    }
}

pub fn safe_root(x: Option<f32>) -> Writer<Option<f32>, String> {
    match x {
        Some(num) => {
            if num < 0.0 {
                return Writer::from_both(
                    None, "safe_root: Argument was negative!\n".to_string())
            }

            kleisli_id(Some((num as f32).sqrt()))
        },
        None => {
            kleisli_id(None)
        }
    }
}

pub fn f32_to_option(x: f32) -> Writer<Option<f32>, String> {
    kleisli_id(Some(x))
}

pub fn main() {
    let mut safe_root_reciprocal = compose_kleisli(
        Box::new(f32_to_option),
        Box::new(compose_kleisli(
            Box::new(safe_root), Box::new(safe_reciprocal))));
    println!("1/sqrt(5) = {:?}", safe_root_reciprocal(25.0).data.unwrap());
    println!("1/sqrt(-3) = {:?}", safe_root_reciprocal(-3.0));
    println!("1/sqrt(0) = {:?}", safe_root_reciprocal(0.0));
}

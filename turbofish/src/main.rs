use num_traits::cast::NumCast;
use num_traits::Num;

fn add<T, U>(a: T, b: U) -> Option<T>
where
    T: Num + NumCast,
    U: Num + NumCast,
{
    let b_as_t = T::from(b)?;
    Some(a + b_as_t)
}

fn main() {
    let x = 5;
    let y = 4.99999;

    match add::<i32, f64>(x, y) {
        Some(result) => println!("Approximate sum of {x} and {y}: {}", result),
        None => println!("Cannot perform the addition operation."),
    }
}

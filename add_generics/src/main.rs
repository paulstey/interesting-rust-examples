use num_traits::cast::NumCast;
use num_traits::Num;

fn add<T, U, R>(a: T, b: U) -> Option<R>
where
    T: Num + NumCast,
    U: Num + NumCast,
    R: Num + NumCast,
{
    let a_as_r = R::from(a)?;
    let b_as_r = R::from(b)?;
    Some(a_as_r + b_as_r)
}

fn main() {
    let x = 5;
    let y = 3.2;

    match add::<i32, f64, f64>(x, y) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot perform the operation."),
    }
}

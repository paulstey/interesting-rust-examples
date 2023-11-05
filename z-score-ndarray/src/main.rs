use ndarray::Array;

fn z_score<T>(data: &[T]) -> Vec<f64>
where
    T: Into<f64> + Clone,
{
    let data_f64: Vec<f64> = data.iter().map(|elem| (*elem).clone().into()).collect();

    let array = Array::from_vec(data_f64);

    let mean = array.mean().unwrap();

    let sd = array.std(1.0);

    let z_scores = array.iter().map(|elem| (*elem - mean) / sd).collect();

    z_scores
}

fn main() {
    let x = [1, 2, 3, 4, 5, 6, 7];

    let z = z_score(&x);

    println!("{:?}", z);
}

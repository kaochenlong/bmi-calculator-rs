pub fn calculator<T, U>(height: T, weight: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    let h = height.into() / 100.0;
    let bmi = weight.into() / (h * h);

    (bmi * 10.0).round() / 10.0
}

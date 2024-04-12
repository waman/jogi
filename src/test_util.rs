
pub fn f_eq(x: f64, y: f64) -> bool {
    return (x - y).abs() < 1e-10;
}
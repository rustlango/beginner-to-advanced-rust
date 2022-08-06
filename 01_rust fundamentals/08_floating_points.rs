// lecture notes Floating points
fn main() {
    let a_min: f32 = std::f32::MIN;
    let b_min: f64 = std::f64::MIN; 

    let a_max: f32 = std::f32::MAX;
    let b_max: f64 = std::f64::MAX; 
  
    println!("{}, {}", a_min, b_min);
    println!("{}, {}", a_max, b_max);
}
// You need 1.0, not 1 to create a float.

// 0.1 + 0.2 does not equal 0.3. See floating point arithmetic.0.30000000000000000000000000000004

fn main() {
    let a: f64 = 1.0; 
    let b: f64 = 0.1;
    let c: f64 = 0.2;
  
    println!("{}, {}", a, b + c);
}
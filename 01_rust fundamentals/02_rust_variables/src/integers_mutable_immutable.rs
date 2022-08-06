pub mod add {
    pub fn adding() {
        let x = 2;
        let y = 5;
        println!("{} + {} = {}",x,y, x+y);
    }
}
    // Primitive variable types are also called scalar types. Scalars: 

    // unsigned integers
    // signed integers 
    // floating points 
    // characters 
    // booleans

pub mod unsigned_integers {
    pub fn unsigned_integers() {
        let a: u8 = 255;
        let b: u16 = 65535;
        let c: u32 = 4294967295;
        let d: u64 = 18446744073709551615;
    } 
}
// Defining the type is optional. Rust auto-implies it.

// If you increase any of the numbers by 1, you get an error. There are no overflows. 

// Unsigned integer min and max values:
pub mod uint_min_max {
    pub fn unsigned_integers() {
        let a_min: u8 = std::u8::MIN;
        let b_min: u16 = std::u16::MIN;
        let c_min: u32 = std::u32::MIN;
        let d_min: u64 = std::u64::MIN;
        let e_min: u128 = std::u128::MIN;

        let a_max: u8 = std::u8::MAX;
        let b_max: u16 = std::u16::MAX;
        let c_max: u32 = std::u32::MAX;
        let d_max: u64 = std::u64::MAX;
        let e_max: u128 = std::u128::MAX;

        println!("u8 MIN: {}, u16 MIN: {} u32 MIN: {}, u64 MIN: {} u128 MIN: 
        {}", a_min, b_min, c_min, d_min, e_min);
        println!("u8 MAX: {}, u16 MAX: {} u32 MAX: {}, u64 MAX: {} u128 MAX: 
        {}", a_max, b_max, c_max, d_max, e_max);
    }
}

pub mod signed_integers_min_max {
    pub fn signed_integers_min_max() {
        let a_min: i8 = std::i8::MIN;
        let b_min: i16 = std::i16::MIN;
        let c_min: i32 = std::i32::MIN;
        let d_min: i64 = std::i64::MIN;
        let e_min: i128 = std::i128::MIN;

        let a_max: i8 = std::i8::MAX;
        let b_max: i16 = std::i16::MAX;
        let c_max: i32 = std::i32::MAX;
        let d_max: i64 = std::i64::MAX;
        let e_max: i128 = std::i128::MAX;

        println!("i8 MIN: {}, i16 MIN: {} i32 MIN: {}, i64 MIN: {} i128 MIN: 
        {}", a_min, b_min, c_min, d_min, e_min);
        println!("i8 MAX: {}, i16 MAX: {} i32 MAX: {}, i64 MAX: {} i128 MAX: 
        {}", a_max, b_max, c_max, d_max, e_max);
    }
}
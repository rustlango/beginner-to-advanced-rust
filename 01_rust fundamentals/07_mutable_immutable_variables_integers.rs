// for this lesson I generated a project called rust_variables
// using cargo to better manage my projects

// ----lecture---
// Primitive datatypes and variables

// Adding variables to println:

fn main() {
   let x = 2;
   let y = 5;
   println!("{} + {} = {}", x, y, x+y);
}
// Primitive variable types are also called scalar types. Scalars: 

// unsigned integers
// signed integers 
// floating points 
// characters 
// booleans
fn main() {
    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615; 
  
    println!("{}, {}, {}, {}", a, b, c, d);
 }
// Defining the type is optional. Rust auto-implies it.

// If you increase any of the numbers by 1, you get an error. There are no overflows. 

// Unsigned integer min and max values:

fn main() {
    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN; 
    let e_min: usize = std::usize::MIN;

    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX; 
    let e_max: usize = std::usize::MAX;
  
    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);
 }
Signed integer min and max values:

fn main() {
    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN; 
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX; 
    let e_max: isize = std::isize::MAX;
  
    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);
 }

//  // lecture commit notes and tips
//   (HEAD -> main)
// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:39:10 2022 +0200

//     git ignore added

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:37:38 2022 +0200

//     git ignor efile added to exclude all target directories

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:32:52 2022 +0200

//     moved target directory to .ignore

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:25:47 2022 +0200

//     brought all modules using the shortcut: use integers_mutable_immutable::*; works when there are a lot of modules in a library file


// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:23:17 2022 +0200

//     updated lecture notes

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 14:21:40 2022 +0200

//     turned all functions into separte modules in an external rust file and imported all of them using the use keyowrd

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 13:51:49 2022 +0200

//     seprated modules as external files to house all the seprate pieces of code - it is fine for now becuas ethe modules are not big

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 13:50:59 2022 +0200

//     created external module dot rs files and then imported them using the use keyword


// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 13:42:58 2022 +0200

//     created external module dot rs files and then imported them using the use keyword

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 11:52:24 2022 +0200

//     cargo and npm distinctions made


// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 11:45:04 2022 +0200

//     ran rust project using cargo

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 11:40:43 2022 +0200

//     rust compiler hello_world progrm with newline characters. Use rusc to compile

// Author: Alvo Rithm <alvorithmemail@gmail.com>
// Date:   Sat Aug 6 11:34:37 2022 +0200

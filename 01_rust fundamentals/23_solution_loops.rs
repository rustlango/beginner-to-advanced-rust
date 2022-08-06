// solution to exercises on rust fundamentals
fn main() {
    // 1. Convert temperatures between Fahrenheit and Celsius.
    let celsius1 = 37.0;
    let celsius2 = 0.0;
    let farenheit1 = 32.0;
    let farenheit2 = 104.0;
 
    println!("{} C = {:.1} F", celsius1, 32.0 + 9.0 / 5.0 * celsius1);
    println!("{} C = {} F", celsius2, 32.0 + 9.0 / 5.0 * celsius2);
    println!("{} F = {} C", farenheit1, (farenheit1 - 32.0) * 5.0 / 9.0);
    println!("{} F = {} C", farenheit2, (farenheit2 - 32.0) * 5.0 / 9.0);
   
    // 2. Generate the nth Fibonacci number.
    // fib_0 = 0
    // fib_1 = 1
    // fib_2 = 1
    // fib_3 = 2
    // fib_4 = 3
    // fib_5 = 5
 
    let n = 15;
    let mut previous = 0;
    let mut current = 1;
 
    for _ in 2..=n {
        let temp = previous + current;
        previous = current;
        current = temp;
    }
 
    println!("Fib({}) = {}\n\n", n, current);
 
 
    // 3. Print the lyrics to the Christmas carol
    // “The Twelve Days of Christmas,”
    // taking advantage of the repetition in the song.
    let first_line_prefix = "On the ";
    let first_line_suffix = " day of Christmas";
    let days = ["first", "second", "third", "fourth", "fifth", "sixth"];
    let things = [
        "And a partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a laying"
    ];
 
    for i in 0..days.len() {
        println!(
            "{}{}{}\nMy true love sent to me",
            first_line_prefix,
            days[i],
            first_line_suffix);
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            let mut j = i;
            loop {
                println!("{}", things[j]);
                if j == 0 {
                    break;
                }
                j = j - 1;
            }
        }
 
        println!();
    }
}
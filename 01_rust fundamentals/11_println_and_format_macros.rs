// lecture notes on println and format macros:
fn main() {
    let btc = "Bitcoin";
    let eth = "Ethereum";
    let sol = "Solana";
 
    println!("Normal order: {} {} {}", btc, eth, sol);
    println!("Flippening: {1} {0} {2} {2}", btc, eth, sol);
    println!(
        "Kwargs: {solana} {ethereum} {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol
    );
    // format macro function is just to return a string
    let message = format!(        
        "Kwargs: {solana} {ethereum} {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol
    );
    println!("Look I've made this special {message}!", message=message);
 
}
use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();

    // do something with the input
    // writing to the journal makes it public
    env::commit(&input);
}
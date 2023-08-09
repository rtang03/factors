use factors_methods::MULTIPLY_ID;
use host::multiply_factors;

fn main() {
    // Pick two numbers
    let (receipt, _) = multiply_factors(17, 23);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}

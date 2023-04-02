use methods::{PREDICTION_ELF, PREDICTION_ID};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use std::io;


fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(PREDICTION_ELF).expect("Prover should be constructed from valid ELF binary");

    // TODO: Implement communication with the guest here
    println!("Please input the sepal length, sepal width, petal length, petal width.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();


    let sepal_length: u32 = s.next().unwrap().parse().unwrap(); 
    let sepal_width: u32 = s.next().unwrap().parse().unwrap();
    let petal_length: u32 = s.next().unwrap().parse().unwrap();
    let petal_width :u32 = s.next().unwrap().parse().unwrap();

    prover.add_input_u32_slice(&to_vec(&sepal_length).unwrap());
    prover.add_input_u32_slice(&to_vec(&sepal_width).unwrap());
    prover.add_input_u32_slice(&to_vec(&petal_length).unwrap());
    prover.add_input_u32_slice(&to_vec(&petal_width).unwrap());


    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&PREDICTION_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

        // Extract journal of receipt
        let c: u32 = from_slice(&receipt.journal).unwrap();

        let dic = ["setosa", "versicolor", "virginica"];


        // Print an assertion
        println!("This is the {} flower, and I can prove it!", dic[c as usize]);
}

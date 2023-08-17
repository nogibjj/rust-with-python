// integration_test.rs in the "tests" directory

use polars_cli::calculate;  // Importing the library crate
use std::process::Command;

#[test]
fn test_cli_output() {
    // Prepare and run the command
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");
    
    // Ensure the command exited without errors
    assert!(output.status.success(), "Command exited with error status");
    
    // Convert the output to a string and check if it contains the expected value
    let actual = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    let expected = "shape: (3, 5)";
    assert!(actual.contains(expected), "Output does not contain expected string");
}

#[test]
fn test_calculate_output_shape() {
    // Call the `calculate()` function and ensure it executes successfully
    let df = calculate().expect("Calculate function failed");
    
    // Check the shape of the resulting DataFrame against the expected value
    assert_eq!(df.shape(), (3, 5), "DataFrame shape is not as expected");
}

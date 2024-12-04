/** Command line:
1. cargo fmt - formats all the source code
2. rustfmt {path of the file} - formats only a speciofic file
e.g.  rustfmt src/advanced_code_editing/Format
 */

/**This method solves the cartesian product/lists all possible pairs or characters from the two input strings*/
pub fn cartesian_product(s1: &str, s2: &str) {
    let mut result = Vec::new();
    for c1 in s1.chars() {
        // First loop iterates over the first string
        {
            for c2 in s2.chars() {
                // Second loop iterates over the second string
                {
                    result.push(format!("{}{}", c1, c2)); // Nesting to create pairs
                }
            }
        }
    }

    // Additional loop to demonstrate multiple levels of loops and nesting
    for i in 0..result.len() {
        {
            for _ in 0..1 {
                {
                    result[i] = format!("[{}]", result[i]); // Wrapping each pair in brackets
                }
            }
        }
    }

    println!("{:?}", result);
}

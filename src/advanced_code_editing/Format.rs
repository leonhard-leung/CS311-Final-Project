pub fn cartesian_product(s1: &str, s2: &str)
    {
        let mut result = Vec::new();
        for c1 in s1.chars()
            // First loop iterates over the first string
            {
                for c2 in s2.chars()
                    // Second loop iterates over the second string
                    {
                        // Nesting to create pairs
                        result.push(format!("{}{}", c1, c2));
                    }

            }
    
        // Additional loop to demonstrate multiple levels of loops and nesting
        for i in 0..result.len()
            {
                for _ in 0..1
                    {
                        // Wrapping each pair in brackets
                        result[i] = format!("[{}]", result[i]);
                    }

            }

        println!("{:?}", result);
    }

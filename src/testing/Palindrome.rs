fn palindrome_checker(word: String) -> bool
    {
        let chars: Vec<char> = word.chars().collect();
        let max_length = chars.len();
        let half_length = max_length / 2;
    
        for step in 0..half_length
            {
                if chars[step] != chars[max_length - step - 1]
                    {
                        return false;
                    }
            }
    
        true
    }

#[test]
fn test_palindrome_checker()
    {
        assert!(palindrome_checker("abbba".to_string()));
        assert_eq!(palindrome_checker("hello".to_string()), true);
        assert_ne!(palindrome_checker("madam".to_string()), false);
    }

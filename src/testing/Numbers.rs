fn add(n1: i32, n2: i32) -> i32
    {
        n1 + n2
    }

fn subtract(n1: i32, n2: i32) -> i32
    {
        n1 - n2
    }

fn multiply(n1: i32, n2: i32) -> i32
    {
        n1 * n2
    }

fn divide(n1: f64, n2: f64) -> Result<f64, &'static str>
    {
        if n2 == 0.0 {
            panic!()
        }
    
        Ok(n1 / n2)
    }

fn odd_number(num: u32) -> bool
    {
        num % 2 != 0
    }

fn factorial(num: u32) -> u32
    {
        if num == 0 || num == 1 {
            return 1
        } else {
            num * factorial(num - 1)
        }
    }

fn square_root(num: f64) -> Result<f64, &'static str>
    {
        if num < 0.0 {
            return Err("Cannot find the square root of a negative number");
        }
        Ok(num.sqrt())
    }

fn fibonacci(n: u32) -> u32
    {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }


/** ================================================================================================*/
/** ================================================================================================*/

/**
1. tmod - test module */

#[cfg(test)]
mod tests {
    use super::*; //allows test module to have access to parent module

    //add test method here

    #[test]
    fn test_subtract()
        {
            assert_eq!(subtract(5, 3), 2);
            assert_eq!(subtract(3, 5), -2);
        }

    #[test]
    fn test_multiply()
        {
            assert_eq!(multiply(2, 3), 6);
            assert_eq!(multiply(-2, 3), -6);
        }

    #[test]
    fn test_divide()
        {
            assert_eq!(divide(6.0, 3.0), Ok(2.0));
            assert_eq!(divide(10.0, 2.0), Ok(5.0));
        }

    #[test]
    fn test_odd_number()
        {
            assert!(odd_number(3));
            assert!(!odd_number(4));
        }

    #[test]
    fn test_factorial()
        {
            assert_eq!(factorial(0), 1);
            assert_eq!(factorial(1), 1);
            assert_eq!(factorial(5), 120);
            assert_eq!(factorial(6), 720);
        }

    #[test]
    fn test_square_root()
        {
            assert_eq!(square_root(9.0), Ok(3.0));
            assert_eq!(square_root(-1.0), Err("Cannot find the square root of a negative number"));
        }

    #[test]
    fn test_fibonacci()
        {
            assert_eq!(fibonacci(0), 0);
            assert_eq!(fibonacci(1), 1);
            assert_eq!(fibonacci(5), 5);
            assert_eq!(fibonacci(6), 8);
        }

    // should panic testing
    #[test]
    #[should_panic]
    fn test_division_by_zero()
        {
            let _ = divide(10.0,0.0);
        }
}
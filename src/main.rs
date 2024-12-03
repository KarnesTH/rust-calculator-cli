/// Parses user input into two numbers and an operator.
///
/// # Arguments
/// * `input` - A string slice containing the input to be parsed
///
/// # Returns
/// * `Result<(f64, f64, &str), Box<dyn std::error::Error>>` - A tuple containing:
///   - first number (f64)
///   - second number (f64)
///   - operator (&str)
///
/// # Errors
/// Returns an error if:
/// * The input doesn't contain exactly 3 parts (two numbers and an operator)
/// * The numbers cannot be converted to f64
/// * The operator is not one of the allowed operators (+, -, *, /)
///
/// # Examples
/// ```
/// let input = "5.5 + 3.2";
/// let result = parse_input(input);
/// assert!(result.is_ok());
/// let (num1, num2, op) = result.unwrap();
/// assert_eq!(num1, 5.5);
/// assert_eq!(num2, 3.2);
/// assert_eq!(op, "+");
/// ```
fn parse_input(input: &str) -> Result<(f64, f64, &str), Box<dyn std::error::Error>> {
    let values: Vec<&str> = input.split_whitespace().collect();

    if values.len() != 3 {
        return Err("Invalid input".into());
    }

    let num1: f64 = values[0].parse()?;
    let num2: f64 = values[2].parse()?;
    let operator = values[1];

    if !["+", "-", "*", "/"].contains(&operator) {
        return Err("Invalid operator. Use +, -, *, /".into());
    }

    Ok((num1, num2, operator))
}

/// Performs a mathematical calculation with two numbers and an operator.
///
/// # Arguments
/// * `num1` - First number (f64)
/// * `num2` - Second number (f64)
/// * `operator` - Mathematical operator as string slice
///
/// # Returns
/// * `Result<f64, Box<dyn std::error::Error>>` - The result of the calculation
///
/// # Supported Operators
/// * `+` - Addition
/// * `-` - Subtraction
/// * `*` - Multiplication
/// * `/` - Division
///
/// # Errors
/// Returns an error if:
/// * Division by zero is attempted
/// * An unsupported operator is used
///
/// # Examples
/// ```
/// let result = calculate(10.0, 5.0, "+");
/// assert_eq!(result.unwrap(), 15.0);
///
/// let divide_by_zero = calculate(5.0, 0.0, "/");
/// assert!(divide_by_zero.is_err());
/// ```
fn calculate(num1: f64, num2: f64, operator: &str) -> Result<f64, Box<dyn std::error::Error>> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero".into())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator".into()),
    }
}

/// Entry point of the calculator application.
///
/// This function runs an interactive command-line calculator that:
/// - Continuously prompts for user input
/// - Processes mathematical expressions
/// - Handles errors gracefully
/// - Allows clean program termination
///
/// # Usage
/// The program accepts input in the format: "number operator number"
/// - Valid operators: +, -, *, /
/// - Numbers can be integers or floating-point
/// - Enter 'q' to quit the program
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) on successful execution
///
/// # Examples
/// ```text
/// Please enter your calculation (e.g. 5 + 5) or 'q' to quit:
/// 5 + 5
/// 5 + 5 = 10
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Please enter your calculation (e.g. 5 + 5) or 'q' to quit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "q" {
            println!("Thanks for using.");
            break;
        }

        match parse_input(&input).and_then(|(num1, num2, operator)| {
            let result = calculate(num1, num2, operator)?;
            println!("{} {} {} = {}", num1, operator, num2, result);
            Ok(())
        }) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_valid() {
        let input = "5 + 5";
        let result = parse_input(input);
        assert!(result.is_ok());
        let (num1, num2, operator) = result.unwrap();
        assert_eq!(num1, 5.0);
        assert_eq!(num2, 5.0);
        assert_eq!(operator, "+");
    }

    #[test]
    fn test_parse_input_invalid_format() {
        let input = "5 + ";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_parse_input_invalid_number() {
        let input = "abc + 5";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_parse_input_invalid_operator() {
        let input = "5 % 5";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_calculate_with_decimals() {
        let result = calculate(5.5, 2.2, "+");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 7.7);
    }

    #[test]
    fn test_calculate_negative_numbers() {
        let result = calculate(-5.0, 3.0, "+");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -2.0);
    }

    #[test]
    fn test_calculate_addition() {
        let result = calculate(5.0, 5.0, "+");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.0);
    }

    #[test]
    fn test_calculate_subtraction() {
        let result = calculate(5.0, 5.0, "-");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_calculate_multiplication() {
        let result = calculate(5.0, 5.0, "*");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 25.0);
    }

    #[test]
    fn test_calculate_division() {
        let result = calculate(5.0, 5.0, "/");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);
    }

    #[test]
    fn test_calculate_division_by_zero() {
        let result = calculate(5.0, 0.0, "/");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
    }
}

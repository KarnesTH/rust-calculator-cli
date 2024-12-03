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

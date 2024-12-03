# Rust Calculator CLI

A simple command-line calculator implemented in Rust that supports basic arithmetic operations.

## Features

- Basic arithmetic operations (+, -, *, /)
- Input validation
- Error handling
- Unit tests
- Interactive command-line interface

## Usage

The calculator accepts input in the format: `number operator number`

Example:
```bash
Please enter your calculation (e.g. 5 + 5) or 'q' to quit:
5 + 5
5 + 5 = 10
```

### Supported Operations

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

### Error Handling

The calculator handles various error cases:
- Invalid input format
- Division by zero
- Invalid operators
- Non-numeric inputs

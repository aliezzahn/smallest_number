# Smallest Number Library

A Rust library to find the smallest number that can be formed by rearranging the digits of a given number such that the product of its digits is divisible by a specified target.

![CI](https://github.com/aliezzahn/smallest_number/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/smallest_number/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Efficient Algorithm**: Optimized for performance and memory usage.
- **Robust Testing**: Includes over 100 test cases to ensure correctness.
- **Easy to Use**: Simple API for integration into your projects.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
smallest_number = "0.1.0"
```

Or run the following command:

```bash
cargo add smallest_number
```

## Usage

### Example

```rust
use smallest_number::smallest_number;

fn main() {
    let num = String::from("100");
    let t = 2;
    let result = smallest_number(num, t);
    println!("The smallest number is: {}", result); // Output: "112"
}
```

### API

#### `smallest_number(num: String, t: i64) -> String`

- **Parameters**:
  - `num`: A string representing the input number.
  - `t`: The target value that the product of the digits must be divisible by.
- **Returns**: A string representing the smallest number that satisfies the condition.

## Testing

The library includes over 100 test cases to ensure correctness. To run the tests, use the following command:

```bash
cargo test
```

### Example Test Output

```
running 100 tests
test tests::test_case_1 ... ok
test tests::test_case_2 ... ok
...
test tests::test_case_100 ... ok

test result: ok. 100 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Write your code and add tests.
4. Ensure all tests pass (`cargo test`).
5. Submit a pull request.

### Guidelines

- Follow Rust's coding conventions.
- Write clear and concise commit messages.
- Include tests for new features or bugfixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Built with ❤️ using Rust.

---

## Support

If you encounter any issues or have questions, please open an issue on [GitHub](https://github.com/yourusername/smallest_number/issues).

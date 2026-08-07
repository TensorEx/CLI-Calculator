# 🧮 CLI Calculator

A simple command-line calculator written in Rust that supports basic arithmetic operations. Built with Rust 🦀

## ⚙️ Features

-  Addition (+)
-  Subtraction (-)
-  Multiplication (*)
-  Division (/)
-  Repeat calculations with optional loop
-  Input validation and error handling

## 🔨 Building

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
cargo build --release
```

## ▶️ Running

```bash
cargo run
```

## 📖 Usage

1.  Start the program
2.  Enter your first number
3.  Enter an operator (+, -, *, /)
4.  Enter your second number
5.  The result will be displayed
6.  Choose to calculate again or exit

### 💡 Example

```
Welcome to the calculator in CLI!
Please enter your first desired number: 
10
Enter operator (+, -, *, /): 
+
Good, now the second number: 
5
Result: 15
Do you wanna calculate again? 
y
```

## 📚 Learning Points

This project demonstrates:
-  Rust's `match` expression for pattern matching
-  String input/output with `io::stdin()`
-  Type parsing with `.parse()`
-  String manipulation with `.trim()`
-  Control flow with loops and conditionals

## 👨‍💻 Author

Created with ❤️ while learning Rust 🦀

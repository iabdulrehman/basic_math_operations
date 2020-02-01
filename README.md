# Basic Math Operations
You can integrate this library and use Basic Math Operations within your rust program by following the method below:

To use this library you have to add following line in dependency section of cargo.toml

`basic_math_operation = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Abdul Rehman <rehman.abdul5666@gmail.com>"]
edition = "2018"

[dependencies]
basic_math_operation = "0.1.0"
```

In `src/main.rs` you can call this library by using "use" keyword like this:

```
use basic_math_operation::calculate;
fn main() {
    let first_no:u8 = 10; 
    let second_no:u8 = 20;
    calculate::addition(first_no,second_no); //Calling Addition function here. It takes two u8 type arguments
						//Remember to check parameter types before passing them.
}
```
following method will also work. By following method you can define the path once and call the functions directly like below:
```
use basic_math_operation::calculate::addition;
use basic_math_operation::calculate::subraction;

fn main() {
    let first_no:u8 = 10; 
    let second_no:u8 = 6;
    addition(first_no,second_no);
    }
```
#NOTE: by following above approach you should not create your own functions with same name or else you will face name conflicting errors within your program.

now type this command in terminal `cargo run` for results

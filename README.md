# list_math

This crate provides functions for performing basic mathematical operations on lists in
Rust.

## Installation

Add the following line to your `Cargo.toml` file:

```
[dependencies]
list_math = "0.1.0"
```
Then run `cargo build` to install the crate.

## Usage

### largest function

The largest function finds the largest element in a list of `isize` values.
```
use list_math::largest;

fn main() {
let numbers = vec![ 3 , 10 , - 1 , 5 ];
let biggest = largest(&numbers);
println!("The largest number is: {}", biggest);
}
```

This code will print:
```
The largest number is: 10
```
Note: The largest function works with any type that implements the `PartialOrd` trait,
which allows for ordering elements.
### AverageList struct

The AverageList struct helps you keep track of a running average of `i32` values.


```
use list_math::AverageList;

fn main() {
let mut avg_list = AverageList::new();
avg_list.add( 2 );
avg_list.add( 8 );
avg_list.add( 4 );
println!("Current average: {}", avg_list.average()); // Output: 4.
avg_list.remove(); // Removes last element (4)
println!("Average after removal: {}", avg_list.average()); // Output: 5.
}
```
This code will print:
```
Current average: 4.
Average after removal: 5.
```
Methods:
● new: Creates a new AverageList with an initial average of 0.
● add(value): Adds a new value to the list and updates the average.
● remove:Removes the last added value from the list and updates the average (returns None if the list is empty).
● average: Returns the current average as a f64.

## Contributing

We welcome contributions to this crate! Please measage me at piscitello284@gmail.com for
details on how to get involved.
## License
This crate is licensed under the MIT License.


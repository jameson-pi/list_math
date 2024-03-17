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
### smallest function

The smallest function finds the smallest element in a list of `isize` values.
```
use list_math::smallest;

fn main() {
let numbers = vec![ 3 , 10 , - 1 , 5 ];
let biggest = smallest(&numbers);
println!("The smallest number is: {}", biggest);
}
```

This code will print:
```
The smallest number is: -1
```
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
# Function: standard_deviation
## Description:

This function calculates the standard deviation of a list of floating-point numbers. The standard deviation is a measure of how spread out the data is from its mean (average).

### Parameters:

data: A slice of type `&[f64]`. This represents a reference to a slice of floating-point numbers.

#### Return Type:

`Option<f64>`: The function returns an Option of type f64. The Option type can hold either Some(value) where value is the calculated standard deviation or None if there's an error (e.g., empty data list).
Error Handling:

The function handles the case of an empty data list by returning None.

### Implementation Details:

The function first checks if the input data is empty.
If not empty, it calculates the mean of the data.
It then iterates through the data and computes the squared deviations from the mean for each element.
Bessel's correction is applied by dividing the sum of squared deviations by data.len() as f64 - 1.0. This adjusts for the bias introduced when estimating the population mean from a sample.
Finally, the function returns the square root of the variance (sum of squared deviations divided by the adjusted number of elements), which is the standard deviation.
### Example Usage:

```
fn main() {
  let numbers = vec![1.0, 3.0, 5.0, 7.0, 9.0];
  let std_dev = standard_deviation(&numbers);

  if let Some(std_dev) = std_dev {
    println!("Standard deviation: {}", std_dev);
  } else {
    println!("Empty data list");
  }
}
```
Note: This function assumes the data type is f64 for floating-point numbers.
## Contributing

We welcome contributions to this crate! Please measage me at piscitello284@gmail.com for
details on how to get involved.
## License
This crate is licensed under the MIT License.


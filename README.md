# list_math
list_math is a rust package for doing math on a list.
## functions
```
use list_math::largest_isize;

fn main() {
  let list = vec!(1,2,3,4,5);
  let largest = largest_isize(&list);
  assert_eq!(largest, 5);
``` 
would work.
  

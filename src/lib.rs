//! a crate for doing math on a list.
/// a thing for finding largest isize
pub fn largest<X: PartialOrd>(list: &[X]) -> X {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest{
            largest = item;
        }
    }
    largest
}
/// find average on list
pub struct AverageList {
    list: Vec<i32>,
    average: f64,
}
impl AverageList {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    pub fn average(&mut self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[test]
fn largest_isize_test(){
    let list = vec!(1,6,2);
    let largest = largest(&list);
    assert_eq!(largest, 6);
    println!("{}",largest);
}
#[test]
fn average_list_test(){
  let mut list = AverageList{
    list: vec!(3),
    average: 3 as f64
  };
  for x in 1..7 {
    list.add(x);
  }
  list.remove();
  let num = 3;
  assert_eq!(num as f64,list.average);
}

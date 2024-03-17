//! a crate for doing math on a list.
/// a thing for finding largest isize
fn largest<T: PartialOrd + std::cmp::Ord>(list: &[T]) -> Option<&T> {
    list.iter().max()
  }
fn smallest<T: PartialOrd + std::cmp::Ord>(list: &[T]) -> Option<&T> {
    list.iter().min()
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

//! a crate for doing math on a list.

use std::cmp::Ord;
use std::convert::TryInto;

/// a function for finding largest isize
#[inline]
pub fn largest<T: PartialOrd + Ord>(list: &[T]) -> Option<&T> {
    list.iter().max()
}

#[inline]
pub fn smallest<T: PartialOrd + Ord>(list: &[T]) -> Option<&T> {
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
            }
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

/// This function calculates the standard deviation of a list of floating-point numbers. The
/// standard deviation is a measure of how spread out the data is from its mean (average).
pub fn standard_deviation<T: Clone + TryInto<f64>>(data: &[T]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    let data: Vec<f64> = data
        .iter()
        .map(|x| x.clone().try_into().ok())
        .collect::<Option<Vec<_>>>()?;

    let mean = data.iter().sum::<f64>() / data.len() as f64;

    let variances = data
        .iter()
        .map(|x| (x - mean).powi(2)) // Square the difference from the mean
        .sum::<f64>()
        / (data.len() as f64 - 1.0); // Bessel's correction

    Some(variances.sqrt()) // Return the square root of the variance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_standard_deviation() {
        let numbers = vec![1, 3, 5, 7, 9];
        let std_dev = standard_deviation(&numbers);
        assert_eq!(std_dev, Some(3.1622776601683795));
    }
}

//! a crate for doing math on a list.

use std::cmp::Ord;
use std::convert::TryInto;

/// a function for finding largest isize
#[inline]
pub fn largest<T: Clone + PartialOrd + Ord>(list: &[T]) -> Option<T> {
    list.iter().max().cloned()
}

#[inline]
pub fn smallest<T: Clone + PartialOrd + Ord>(list: &[T]) -> Option<T> {
    list.iter().min().cloned()
}

/// find average on list
#[derive(Debug, Clone, Default)]
pub struct AverageList<T: Into<i128> + Clone> {
    list: Vec<T>,
    average: f64,
}

impl<T: Into<i128> + Clone> AverageList<T> {
    pub fn add(&mut self, value: T) {
        self.list.push(value);
        self.update_average();
    }

    /// Remove the last element from the list and update the average.
    pub fn remove(&mut self) -> Option<T> {
        let value = self.list.pop()?;
        self.update_average();
        Some(value)
    }

    pub fn average(&mut self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i128 = self.list.iter().map(|x| x.clone().into()).sum();
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

    #[test]
    fn avg_list() {
        let mut avg_list = AverageList::default();
        avg_list.add(2);
        avg_list.add(8);
        avg_list.add(4);
        assert_eq!(Some(4), avg_list.remove());
    }

    #[test]
    fn smallest_test() {
        let numbers = vec![1, 3, 5, 7, 9];
        let smallest = smallest(&numbers);
        assert_eq!(smallest, Some(1));
    }

    #[test]
    fn largest_test() {
        let numbers = vec![1, 3, 5, 7, 9];
        let largest = largest(&numbers);
        assert_eq!(largest, Some(9));
    }
}

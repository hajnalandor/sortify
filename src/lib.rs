pub trait Sortifier<T> {
    fn sort(&self,slice: &mut [T])
    where
        T: Ord;
}

mod buble;
mod insertion;
mod quick;
mod selection;

pub use buble::BubbleSort;
pub use insertion::InsertionSort;
pub use quick::QuickSort;
pub use selection::SelectionSort;

pub struct StdSorter;
impl<T> Sortifier<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sortifier<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut v = vec![5, 4, 3, 2, 1];
        StdSorter.sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }
}

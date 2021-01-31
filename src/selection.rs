use super::Sortifier;

pub struct SelectionSort;

impl<T> Sortifier<T> for SelectionSort {
    fn sort(&self,slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest] {
                    smallest = i
                }
            }
            slice.swap(unsorted, smallest);
        }
    }
}

#[test]
fn selection_odd() {
    let mut v = vec![5, 4, 3, 2, 1];
    SelectionSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

#[test]
fn selection_even() {
    let mut v = vec![5, 4, 3, 2];
    SelectionSort.sort(&mut v);
    assert_eq!(v, &[2, 3, 4, 5]);
}

use super::Sortifier;

pub struct BubbleSort;

impl<T> Sortifier<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;

        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubble_odd() {
    let mut v = vec![5, 4, 3, 2, 1];
    BubbleSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

#[test]
fn bubble_even() {
    let mut v = vec![5, 4, 3, 2, 136, 8];
    BubbleSort.sort(&mut v);
    assert_eq!(v, &[2, 3, 4, 5, 8, 136]);
}

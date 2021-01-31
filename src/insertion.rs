use super::Sortifier;

pub struct InsertionSort;

impl<T> Sortifier<T> for InsertionSort {
    fn sort(&self,slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                Ok(i) => i,
                Err(i) => i,
            };
            slice[i..=unsorted].rotate_right(1);
        }
    }
}

#[test]
fn insertion_odd() {
    let mut v = vec![5, 4, 3, 2, 1];
    InsertionSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

#[test]
fn insertion_even() {
    let mut v = vec![5, 4, 3, 2];
    InsertionSort.sort(&mut v);
    assert_eq!(v, &[2, 3, 4, 5]);
}

use super::Sortifier;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    let left = left + 1;
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl<T> Sortifier<T> for QuickSort {
    fn sort(&self,slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
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
fn quick_odd() {
    let mut v = vec![5, 4, 3, 2, 1];
    QuickSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

#[test]
fn quick_even() {
    let mut v = vec![5, 4, 3, 2];
    QuickSort.sort(&mut v);
    assert_eq!(v, &[2, 3, 4, 5]);
}

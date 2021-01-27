use crate::Exit;

pub trait Orster {
    fn orst<T, C>(&self, slice: &mut [T], callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize) -> Exit;
}

pub struct BubbleOrst;

impl Orster for BubbleOrst {
    fn orst<T, C>(&self, slice: &mut [T], mut callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize) -> Exit,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);

                    if matches!(callback(i, i + 1), Exit::Yes) {
                        return;
                    }

                    swapped = true;
                }
            }
        }
    }
}

pub struct InsertionOrst;

impl Orster for InsertionOrst {
    fn orst<T, C>(&self, slice: &mut [T], mut callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize) -> Exit,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);

                if matches!(callback(i - 1, i), Exit::Yes) {
                    return;
                }

                i -= 1;
            }
        }
    }
}

pub struct QuickOrst;

impl Orster for QuickOrst {
    fn orst<T, C>(&self, slice: &mut [T], mut callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize) -> Exit,
    {
        quicksort(slice, 0, &mut callback);
    }
}

fn quicksort<T, C>(slice: &mut [T], offset: usize, callback: &mut C) -> Exit
where
    T: Ord,
    C: FnMut(usize, usize) -> Exit,
{
    match slice.len() {
        0 | 1 => return Exit::No,
        2 => {
            if slice[0] >= slice[1] {
                slice.swap(0, 1);
                if matches!(callback(0 + offset, 1 + offset), Exit::Yes) {
                    return Exit::Yes;
                }
            }
            return Exit::No;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");

    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            // move element to the right side
            rest.swap(left, right);
            if matches!(callback(left + 1 + offset, right + 1 + offset), Exit::Yes) {
                return Exit::Yes;
            }
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // place pivot at its final position
    slice.swap(0, left);
    if matches!(callback(0 + offset, left + offset), Exit::Yes) {
        return Exit::Yes;
    }

    let (left, right) = slice.split_at_mut(left);
    if matches!(quicksort(left, offset, callback), Exit::Yes) {
        return Exit::Yes;
    }
    if matches!(quicksort(&mut right[1..], offset + left.len() + 1, callback), Exit::Yes) {
        return Exit::Yes;
    }

    Exit::No
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn bubble_orst_works() {
        let mut things = vec![4, 2, 3, 1];
        let mut other_things = things.clone();
        BubbleOrst.orst(&mut things, |i, j| {
            other_things.swap(i, j);
            Exit::No
        });
        assert_eq!(things, vec![1, 2, 3, 4]);
        assert_eq!(things, other_things);
    }

    #[test]
    fn insertion_orst_works() {
        let mut things = vec![4, 2, 3, 1];
        let mut other_things = things.clone();
        InsertionOrst.orst(&mut things, |i, j| {
            other_things.swap(i, j);
            Exit::No
        });
        assert_eq!(things, vec![1, 2, 3, 4])
    }

    #[test]
    fn quick_orst_works() {
        let mut rng = rand::thread_rng();
        let mut things: Vec<u64> = (1..=42).collect();
        things.shuffle(&mut rng);

        let mut expected = things.clone();
        expected.sort();

        let mut other_things = things.clone();
        QuickOrst.orst(&mut things, |i, j| {
            other_things.swap(i, j);
            Exit::No
        });

        assert_eq!(things, expected);
        assert_eq!(things, other_things);
    }
}

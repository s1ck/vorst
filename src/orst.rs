pub trait Orster {
    fn orst<T, C>(&self, slice: &mut [T], callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize);
}

pub struct BubbleOrst;

impl Orster for BubbleOrst {
    fn orst<T, C>(&self, slice: &mut [T], mut callback: C)
    where
        T: Ord,
        C: FnMut(usize, usize),
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    callback(i, i + 1);
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
        C: FnMut(usize, usize),
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                callback(i - 1, i);
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
        C: FnMut(usize, usize),
    {
        quicksort(slice, 0, &mut callback);
    }
}

fn quicksort<T, C>(slice: &mut [T], offset: usize, callback: &mut C)
where
    T: Ord,
    C: FnMut(usize, usize),
{
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] >= slice[1] {
                slice.swap(0, 1);
                callback(0 + offset, 1 + offset);
            }
            return
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
            callback(left + 1 + offset, right + 1 + offset);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // place pivot at its final position
    slice.swap(0, left);
    callback(0 + offset, left + offset);

    let (left, right) = slice.split_at_mut(left);
    quicksort(left, offset, callback);
    quicksort(&mut right[1..], offset + left.len() + 1, callback);
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn bubble_orst_works() {
        let mut things = vec![4, 2, 3, 1];
        let mut other_things = things.clone();
        BubbleOrst.orst(&mut things, |i, j| other_things.swap(i, j));
        assert_eq!(things, vec![1, 2, 3, 4]);
        assert_eq!(things, other_things);
    }

    #[test]
    fn insertion_orst_works() {
        let mut things = vec![4, 2, 3, 1];
        let mut other_things = things.clone();
        InsertionOrst.orst(&mut things, |i, j| other_things.swap(i, j));
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
        QuickOrst.orst(&mut things, |i, j| other_things.swap(i, j));

        assert_eq!(things, expected);
        assert_eq!(things, other_things);
    }
}

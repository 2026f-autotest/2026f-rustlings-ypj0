/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    for start in (0..len / 2).rev() {
        sift_down(array, start, len);
    }

    for end in (1..len).rev() {
        array.swap(0, end);
        sift_down(array, 0, end);
    }
}

fn sift_down<T: Ord>(array: &mut [T], mut root: usize, end: usize) {
    loop {
        let left = 2 * root + 1;
        if left >= end {
            break;
        }

        let right = left + 1;

        let mut child = left;
        if right < end && array[right] > array[left] {
            child = right;
        }

        if array[child] <= array[root] {
            break;
        }

        array.swap(root, child);
        root = child;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}

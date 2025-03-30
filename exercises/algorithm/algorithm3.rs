/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::{
    array,
    fmt::{Debug, Display},
    vec,
};

fn merge<T: PartialOrd + Clone + Debug>(lo: usize, hi: usize, arr: &mut [T]) {
    let mut temp = arr[lo..hi + 1].to_vec();
    println!("{:?}", temp);
    let mid = (temp.len() + 1) / 2;
    let mut i = 0;
    let mut j = (temp.len() + 1) / 2;
    for k in lo..hi + 1 {
        if i >= mid {
            arr[k] = temp[j].clone();
            j += 1;
        } else if j >= temp.len() {
            arr[k] = temp[i].clone();
            i += 1;
        } else if temp[i] <= temp[j] {
            arr[k] = temp[i].clone();
            i += 1;
        } else {
            arr[k] = temp[j].clone();
            j += 1;
        }
    }
    println!("{:?}", arr);
}

fn sort_helper<T: PartialOrd + Clone + Debug>(lo: usize, hi: usize, arr: &mut [T]) {
    if lo >= hi {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    sort_helper(lo, mid, arr);
    sort_helper(mid + 1, hi, arr);
    merge(lo, hi, arr);
    println!("merge:{} to {}", lo, hi);
}
fn sort<T: Clone + PartialOrd + Debug>(array: &mut [T]) {
    //TODO
    sort_helper(0, array.len() - 1, array);
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

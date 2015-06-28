use std::cmp::Ord;
use std::cmp::Ordering::{Less, Equal, Greater};

fn chop<T: Ord>(item : T, slice : &[T]) -> i32 {
    let length = slice.len();
    // Catch empty slices
    if length < 1 {
        return -1;
    }

    let mut width = length;
    let mut low = 0;

    while width > 0 {
        let mid_index = low + (width / 2);
        let comparison = item.cmp(&slice[mid_index]);
        match comparison {
            Less => (),
            Greater => {
                low = mid_index + 1;
                width -= 1;
            }
            Equal => return mid_index as i32
        }
        width /= 2;
    }
    return -1;
}

fn main() {
    println!("{}", chop(3, &[1,3,5]));
}

#[test]
fn test() {
    assert_eq!(-1, chop(3, &[]));
    assert_eq!(-1, chop(3, &[1]));
    assert_eq!(0,  chop(1, &[1]));

    assert_eq!(0,  chop(1, &[1, 3, 5]));
    assert_eq!(1,  chop(3, &[1, 3, 5]));
    assert_eq!(2,  chop(5, &[1, 3, 5]));
    assert_eq!(-1, chop(0, &[1, 3, 5]));
    assert_eq!(-1, chop(2, &[1, 3, 5]));
    assert_eq!(-1, chop(4, &[1, 3, 5]));
    assert_eq!(-1, chop(6, &[1, 3, 5]));

    assert_eq!(0,  chop(1, &[1, 3, 5, 7]));
    assert_eq!(1,  chop(3, &[1, 3, 5, 7]));
    assert_eq!(2,  chop(5, &[1, 3, 5, 7]));
    assert_eq!(3,  chop(7, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(0, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(2, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(4, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(6, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(8, &[1, 3, 5, 7]));
}

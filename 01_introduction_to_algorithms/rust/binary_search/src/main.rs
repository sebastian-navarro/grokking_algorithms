use std::cmp;

fn binary_search<T: cmp::PartialOrd>(lst: &[T], item: T) -> Option<usize> {
    let mid = ((lst.len() / 2) as f32).ceil() as usize;

    match lst.get(mid) {
        None => None,
        Some(val) => {
            if *val == item {
                Some(mid)
            } else if *val > item {
                let sublist = &lst[..mid];
                binary_search(sublist, item)
            } else {
                let sublist = &lst[(mid + 1)..];
                // mapping is necessary when the item is
                // to the right of the middle since indices on the
                // sublist are erased and being at 0, 1, 2, 3, ... etc
                binary_search(sublist, item).map(|pos| pos + mid + 1)
            }
        },
    }
}


fn main() {
    let num_slice = &[2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100];
    let result = binary_search(num_slice, 100);
    println!("{:?}", result);
    
}

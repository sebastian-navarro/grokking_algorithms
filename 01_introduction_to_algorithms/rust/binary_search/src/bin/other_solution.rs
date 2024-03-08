

fn binary_search(list: Vec<i32>, item:i32 ) -> Option<usize> {
    let middle = list.len() / 2;
    
    match list.get(middle) {
        None => None,
        Some(value) => {
            if *value == item {
                Some(middle)
            } else if *value > item { 
                let sublist = Vec::from(&list[..middle]);
                binary_search(sublist, item)
            } else {
                let sublist = Vec::from(&list[(middle +1)..]);
                binary_search(sublist, item).map(|pos| pos + middle + 1)
            }
           
        }
    }
}


fn main() {
    let num_slice = vec![2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100];
    let result = binary_search(num_slice, 4);
    println!("{:?}", result);
    
}

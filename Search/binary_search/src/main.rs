use std::cmp::Ordering;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let search = binary_search(5, &data);
    match search {
        Some(index) => println!("Found at index: {index}"),
        None => println!("Not found")
    }
}

fn binary_search<T: Ord>(k: T, items: &[T]) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = items.len() - 1;

    while low <= high {
        let middle = (high + low) / 2;
        match items[middle].cmp(&k) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => high = middle,
            Ordering::Less => low = middle + 1
        }
    }

    None
}
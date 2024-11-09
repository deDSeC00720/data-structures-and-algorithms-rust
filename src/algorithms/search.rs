pub fn linear_search<T: PartialEq>(vec: &[T], target: T) -> Option<usize> {
    for (i, element) in vec.iter().enumerate() {
        if *element == target {
            return Some(i);
        }
    }
    None
}

pub fn binary_search<T: PartialOrd>(vec: &[T], target: T) -> Option<usize> {
    let mut start = 0;
    let mut end = vec.len() - 1;

    while start <= end {
        let mid = start + (end - start) / 2;

        if vec[mid] == target {
            return Some(mid);
        }

        if vec[mid] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    None
}

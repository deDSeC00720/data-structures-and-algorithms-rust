pub fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    for i in 0..vec.len() - 1 {
        let mut min_index = i;
        for j in i + 1..vec.len() {
            if vec[j] < vec[min_index] {
                min_index = j;
            }
        }
        vec.swap(min_index, i);
    }
}

pub fn bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    for _ in 0..vec.len() - 1 {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    for i in 1..vec.len() {
        let temp = vec[i];
        let mut j = i;
        while j > 0 && temp < vec[j - 1] {
            vec[j] = vec[j - 1];
            j -= 1;
        }
        vec[j] = temp;
    }
}

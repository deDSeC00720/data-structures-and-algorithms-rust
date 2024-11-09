pub fn selection_sort<T: PartialOrd>(vec: &mut [T]) {
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

pub fn bubble_sort<T: PartialOrd>(vec: &mut [T]) {
    for _ in 0..vec.len() - 1 {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(vec: &mut [T]) {
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

fn merge<T: PartialOrd + Copy>(vec: &mut [T]) {
    let mid = vec.len() / 2;
    let left = vec[..mid].to_vec();
    let right = vec[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < mid && j < right.len() {
        if left[i] <= right[j] {
            vec[k] = left[i];
            i += 1;
        } else {
            vec[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        vec[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        vec[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    let mid = vec.len() / 2;
    merge_sort(&mut vec[..mid]);
    merge_sort(&mut vec[mid..]);

    merge(vec);
}

fn partition<T: PartialOrd>(vec: &mut [T]) -> usize {
    let mut pivot = 0;

    for i in 1..vec.len() {
        if vec[i] < vec[0] {
            pivot += 1
        }
    }

    vec.swap(0, pivot);

    let mut i = 0;
    let mut j = vec.len() - 1;

    while i < pivot && j > pivot {
        if vec[i] < vec[pivot] {
            i += 1;
            continue;
        }
        if vec[j] >= vec[pivot] {
            j -= 1;
            continue;
        }
        vec.swap(i, j);
        i += 1;
        j -= 1;
    }

    pivot
}

pub fn quick_sort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    let pivot = partition(vec);

    quick_sort(&mut vec[..pivot]);
    quick_sort(&mut vec[(pivot + 1)..]);
}

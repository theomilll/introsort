use std::cmp::Ordering;

pub fn introsort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let max_depth = 2 * (arr.len() as f64).log2() as usize;
    
    introsort_recursive(arr, max_depth);
}

fn introsort_recursive<T: Ord + Copy>(arr: &mut [T], max_depth: usize) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    let len = arr.len();
    
    if len <= INSERTION_SORT_THRESHOLD {
        insertion_sort(arr);
        return;
    }
    
    if max_depth == 0 {
        heap_sort(arr);
        return;
    }
    
    let pivot_index = partition(arr);
    
    introsort_recursive(&mut arr[0..pivot_index], max_depth - 1);
    introsort_recursive(&mut arr[pivot_index + 1..], max_depth - 1);
}

fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn heap_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }
    
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord + Copy>(arr: &mut [T], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    
    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != root {
        arr.swap(root, largest);
        heapify(arr, heap_size, largest);
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    
    let pivot_index = median_of_three(arr);
    
    arr.swap(pivot_index, len - 1);
    
    let pivot = arr[len - 1];
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

fn median_of_three<T: Ord + Copy>(arr: &[T]) -> usize {
    let len = arr.len();
    let mid = len / 2;
    
    let mut indices = [0, mid, len - 1];
    
    for i in 0..2 {
        for j in 0..2 - i {
            if arr[indices[j]] > arr[indices[j + 1]] {
                indices.swap(j, j + 1);
            }
        }
    }
    
    indices[1]
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    introsort(arr);
}

pub fn sort_integers(arr: &mut [i32]) {
    introsort(arr);
}

pub fn sort_strings(arr: &mut [String]) {
    if arr.len() <= 1 {
        return;
    }
    
    let max_depth = 2 * (arr.len() as f64).log2() as usize;
    string_introsort_recursive(arr, max_depth);
}

fn string_introsort_recursive(arr: &mut [String], max_depth: usize) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    let len = arr.len();
    
    if len <= INSERTION_SORT_THRESHOLD {
        string_insertion_sort(arr);
        return;
    }
    
    if max_depth == 0 {
        string_heap_sort(arr);
        return;
    }
    
    let pivot_index = string_partition(arr);
    
    string_introsort_recursive(&mut arr[0..pivot_index], max_depth - 1);
    string_introsort_recursive(&mut arr[pivot_index + 1..], max_depth - 1);
}

fn string_insertion_sort(arr: &mut [String]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn string_heap_sort(arr: &mut [String]) {
    let len = arr.len();
    
    for i in (0..len / 2).rev() {
        string_heapify(arr, len, i);
    }
    
    for i in (1..len).rev() {
        arr.swap(0, i);
        string_heapify(arr, i, 0);
    }
}

fn string_heapify(arr: &mut [String], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    
    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != root {
        arr.swap(root, largest);
        string_heapify(arr, heap_size, largest);
    }
}

fn string_partition(arr: &mut [String]) -> usize {
    let len = arr.len();
    let pivot_index = string_median_of_three(arr);
    
    arr.swap(pivot_index, len - 1);
    
    let pivot = arr[len - 1].clone();
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

fn string_median_of_three(arr: &[String]) -> usize {
    let len = arr.len();
    let mid = len / 2;
    
    let mut indices = [0, mid, len - 1];
    
    for i in 0..2 {
        for j in 0..2 - i {
            if arr[indices[j]] > arr[indices[j + 1]] {
                indices.swap(j, j + 1);
            }
        }
    }
    
    indices[1]
}

fn main() {
    let mut numbers = [5, 2, 9, 1, 5, 6, 3, 8, 4, 7, 0];
    println!("Números antes da ordenação: {:?}", numbers);
    sort_integers(&mut numbers);
    println!("Números depois da ordenação: {:?}", numbers);
    
    let mut words = vec![
        String::from("banana"),
        String::from("maçã"),
        String::from("pera"),
        String::from("uva"),
        String::from("abacaxi"),
        String::from("laranja"),
        String::from("morango"),
    ];
    println!("Palavras antes da ordenação: {:?}", words);
    sort_strings(&mut words);
    println!("Palavras depois da ordenação: {:?}", words);
    
    let mut large_array = [0; 10000];
    for i in 0..large_array.len() {
        large_array[i] = (large_array.len() - i) as i32;
    }
    
    use std::time::Instant;
    let start = Instant::now();
    sort_integers(&mut large_array);
    let duration = start.elapsed();
    
    println!("Tempo para ordenar um array de 10.000 elementos: {:?}", duration);
    println!("Os primeiros 10 elementos ordenados: {:?}", &large_array[0..10]);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        introsort(&mut arr);
        assert_eq!(arr, []);
    }
    
    #[test]
    fn test_single_element() {
        let mut arr = [42];
        introsort(&mut arr);
        assert_eq!(arr, [42]);
    }
    
    #[test]
    fn test_sorted_array() {
        let mut arr = [1, 2, 3, 4, 5];
        introsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = [5, 4, 3, 2, 1];
        introsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_random_array() {
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut expected = arr.clone();
        
        introsort(&mut arr);
        expected.sort();
        
        assert_eq!(arr, expected);
    }
}
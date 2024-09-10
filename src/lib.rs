use std::cmp::{max, min};

pub fn quicksort<T: Ord + Clone>(data: &[T]) -> Vec<T> {
    let mut result = Vec::from(data);
    _quicksort(&mut result, 0, data.len());
    result
}

pub fn quicksort_inplace<T: Ord + Clone>(data: &mut [T]) {
    _quicksort(data, 0, data.len());
}

fn _quicksort<T: Ord + Clone>(data: &mut [T], start: usize, end: usize) {
    let amount = end - start;
    if amount < 2 {
        return;
    }

    let mid = start + amount / 2;
    // take median of (start, mid, end) as pivot as it has low chance of O(n^2)
    let pivot = max(
        min(data[start].clone(), data[end - 1].clone()),
        min(
            max(data[start].clone(), data[end - 1].clone()),
            data[mid].clone(),
        ),
    );

    let mut pivot_idx = if pivot == data[start] {
        start
    } else if pivot == data[end - 1] {
        end - 1
    } else {
        mid
    };

    // bring elements in order
    let mut idx = start;
    while idx < pivot_idx {
        if data[idx] > pivot {
            // swap element right of pivot if it is greater and left of pivot
            data.swap(idx, pivot_idx - 1);
            data.swap(pivot_idx, pivot_idx - 1);
            pivot_idx -= 1;
            // if an element is moved from front to back, index must stay the same
            continue;
        }
        idx += 1;
    }

    idx = pivot_idx + 1;
    while idx < end {
        // swap elements if they are not in right position
        if data[idx] < pivot {
            // swap element left of pivot if it is smaller and right of pivot
            data.swap(idx, pivot_idx + 1);
            data.swap(pivot_idx, pivot_idx + 1);
            pivot_idx += 1;
        }
        idx += 1;
    }

    // sort smaller elements
    _quicksort(data, start, pivot_idx);
    // sort bigger elements
    _quicksort(data, pivot_idx + 1, end);
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::time::Duration;

    use super::*;

    #[test]
    fn out_of_place() {
        let mut rng = rand::thread_rng();
        let data: Vec<u32> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();

        let before = std::time::Instant::now();
        let sorted = quicksort(&data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("Out of place sorting took {duration:?}");

        let mut idx = 0;
        while idx < sorted.len() - 1 {
            let curr = sorted[idx];
            let next = sorted[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }
    }

    #[test]
    fn in_place() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<u32> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();

        let before = std::time::Instant::now();
        quicksort_inplace(&mut data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("In place sorting took {duration:?}");

        let mut idx = 0;
        while idx < data.len() - 1 {
            let curr = data[idx];
            let next = data[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }
    }

    #[test]
    fn performance() {
        const ELEMENTS: u32 = 1_000;
        const RUNS: u32 = 100;
        let mut rng = rand::thread_rng();

        let mut duration_sum = Duration::ZERO;
        for _ in 0..RUNS {
            let mut data: Vec<u32> = (0..ELEMENTS).map(|_| rng.gen_range(0..ELEMENTS)).collect();
            let before = std::time::Instant::now();
            quicksort_inplace(&mut data);
            let after = std::time::Instant::now();
            let duration = after - before;
            duration_sum += duration;
        }

        let duration = duration_sum / RUNS;
        println!("Average duration for {ELEMENTS} randomly sorted elements repeated {RUNS} times: {duration:?}");
    }

    #[test]
    fn worst_case() {
        let mut data: Vec<u32> = (0..1000).collect();

        let before = std::time::Instant::now();
        let result = quicksort(&mut data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("Out of place sorting best case took {duration:?}");

        let mut idx = 0;
        while idx < result.len() - 1 {
            let curr = result[idx];
            let next = result[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }

        let before = std::time::Instant::now();
        quicksort_inplace(&mut data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("In place sorting best case took {duration:?}");

        let mut idx = 0;
        while idx < data.len() - 1 {
            let curr = data[idx];
            let next = data[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }
    }

    #[test]
    fn best_case() {
        let mut data: Vec<u32> = (0..1000).rev().collect();

        let before = std::time::Instant::now();
        let result = quicksort(&mut data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("Out of place sorting worst case took {duration:?}");

        let mut idx = 0;
        while idx < result.len() - 1 {
            let curr = result[idx];
            let next = result[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }

        let before = std::time::Instant::now();
        quicksort_inplace(&mut data);
        let after = std::time::Instant::now();
        let duration = after - before;
        println!("In place sorting worst case took {duration:?}");

        let mut idx = 0;
        while idx < data.len() - 1 {
            let curr = data[idx];
            let next = data[idx + 1];
            idx += 1;
            if curr > next {
                panic!("curr: {curr} > next: {next}");
            }
        }
    }
}

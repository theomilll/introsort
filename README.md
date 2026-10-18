# introsort

An introsort implementation in Rust, written from scratch. It starts with quicksort (median-of-three pivot), switches to heapsort when recursion gets too deep to keep the worst case at O(n log n), and uses insertion sort for slices of 16 elements or fewer.
It sorts any `Ord + Copy` type, with a separate path for `String` slices, and includes unit tests for empty, single-element, sorted, reversed, and random inputs.

## Run

```bash
rustc -O introsort-rust.rs && ./introsort-rust
rustc --test introsort-rust.rs -o tests && ./tests
```

## Benchmark

Sorting 1,000,000 `i32` values, best of 7 runs, release build on an Apple M4, compared with Rust's built-in `sort_unstable`.

| Input | introsort | `sort_unstable` |
| --- | --- | --- |
| Random | 42.8 ms | 9.0 ms |
| Already sorted | 5.7 ms | 0.2 ms |
| Reverse sorted | 20.8 ms | 0.3 ms |
| Many duplicates (100 distinct values) | 27.8 ms | 2.8 ms |

The standard library's pattern-defeating sort is faster on every input, especially on presorted data that it detects and skips. This project focuses on the classic algorithm, not on beating it.

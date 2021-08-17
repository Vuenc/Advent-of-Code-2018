My Rust solutions for the first 11 days of [Advent of Code 2018](https://adventofcode.com/2018).

To run a particular day, the `run_day` function in `main.rs` can be used:

```rust
// Run star 1 of day 3
run_day(3, Some(1));
// Run star 2 of day 5
run_day(5, Some(2));
// Run star 3 of day 11
run_day(3, None);
```

By default, the main function runs all days.
Usage:
```bash
cargo run --release
```

Output:
```
Running Day 1:
Star 2: 66932 (29.05 ms)
Running Day 2:
Star 1: 7192 (410.62 μs)
Star 2: mbruvapghxlzycbhmfqjonsie (2.87 ms)
Running Day 3:
Star 1: 103482 (8.17 ms)
Star 2: 686 (5.34 ms)
Running Day 4:
Min minute: 43, max guard: 283
Star 1: 12169 (4.32 ms)
Star 2: 16164 (4.54 ms)
Running Day 5:
Star 1: 9288 (10.17 ms)
Star 2: 5844 (48.47 ms)
Running Day 6:
Star 1: 3890 (22.61 ms)
Star 2: 40284 (8.64 ms)
Running Day 7:
Star 1: GJFMDHNBCIVTUWEQYALSPXZORK (351.52 μs)
Star 2: 1050 (330.24 μs)
Running Day 8:
Star 1: 41849 (885.78 μs)
Star 2: 32487 (843.64 μs)
Running Day 9:
Star 1: 375465 (4.96 ms)
Star 2: 3037741441 (599.44 ms)
Running Day 10:
Star 1: 
#.......#....#..#####...#....#..######..#....#..#....#.....###
#.......#...#...#....#..#....#.......#..#....#..#....#......#.
#.......#..#....#....#..#....#.......#..#....#..#....#......#.
#.......#.#.....#....#..#....#......#...#....#..#....#......#.
#.......##......#####...######.....#....######..######......#.
#.......##......#.......#....#....#.....#....#..#....#......#.
#.......#.#.....#.......#....#...#......#....#..#....#......#.
#.......#..#....#.......#....#..#.......#....#..#....#..#...#.
#.......#...#...#.......#....#..#.......#....#..#....#..#...#.
######..#....#..#.......#....#..######..#....#..#....#...###.. (30.27 ms)
Star 2: 10159 (50.88 ms)
Running Day 11:
Star 1: 235,48 (4.12 ms)
Star 2: 285,113,11 (161.86 ms)
Total time: 1.09 s
```
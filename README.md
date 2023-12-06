# Advent of Code 2023 in Rust
AoC is an annual coding competition.  Learn more here: https://adventofcode.com/

## Languages
This year, my primary language is Rust and this is my core repository.  For those on my leader boards, please consider that when you are doing `count = int(j)`, I'm doing `let count = num.parse::<i64>().unwrap();`.   :)

I also have repositories for:
* Java (for FRC)
* Python (for work)
* C++ (for kids in Computer Engineering)

They are the same algorithms, but you can see a difference in syntax, language features, and performance.  It is unlikely that I will keep these up for that past a few days.

## How to run
Download the repository and run with:

`cargo run --release`.

Run the test cases (they use test input files) with 

`cargo test`

## Development Process
I generally have three passes to my code:
1. Get it working
2. Optimize for runtime performance
3. Ensure the code is easy to read

I prefer the fewest lines as possible without over obfuscating the logic.  There is no production-level error handling.

## Performance Results
These times are from:
* ASUS Zephyrus Duo
* AMD Ryz
en 9 6900 HX 3.3 GHz
* 32 GB Ram
* Windows 11 Pro (an the associate bloat running in the background)
```
01: load/parse      in    266.2µs
 1:           53194 in     62.3µs
 2:           54249 in    166.3µs
02: load/parse      in    126.5µs
 1:            2449 in      900ns
 2:           63981 in      300ns
03: load/parse      in    523.7µs
 1:          560570 in     65.3µs
 2:        91601924 in     38.6µs
04: load/parse      in    411.1µs
 1:           25231 in      1.4µs
 2:         9721255 in     21.9µs
05: load/parse      in    120.3µs
 1:       379811651 in      2.2µs
 2:        27992443 in     23.6µs
Total elapsed time:      3.2121ms
```
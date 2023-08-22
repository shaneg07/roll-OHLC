# Tensorfox Homework Assignment

## Task completed - 


- &#x2611; Create a rust library to compute rolling OHLC.
- &#x2611; Use created crate as a dependency to read JSON data and produce output in given format.
- &#x2611; Provide tests.
- &#x2611; Provide benchmarks.

## Benchmarks 

The time taken for the functions to process each ticker was measured using the Struct std::time::Instant. <br />
#### Note :- In both the cases, I have excluded the time taken to write the data to file.
The following code describes the measurement parameters - 

```rust

  for ticker in data {

    let now: Instant = Instant::now();
        {
          //ticker processing code here
        }
     
    let elapsed: std::time::Duration = now.elapsed();
    let output: f32 = elapsed.as_nanos() as f32;
    bencher.push(output);
    mode.push(output as i32);

  }
    let mean = statistical::mean(&bencher);
    let median = statistical::median(&bencher);
    let mode = statistical::mode(&mode).unwrap();
    let sd = statistical::standard_deviation(&bencher, Some(mean));

```
<br />

____________________________

1. The algorithm which has O(n) time complexity, completes processing a ticker with the following stats : <br />
- Mean : 112105.62 ns
- Median : 104000 ns
- Mode : 223000 ns
- SD : 67722.04 ns <br />
![alt text](https://github.com/shaneg07/roll-OHLC/blob/main/on.jpg?raw=true)

<br />

### Due to O(n) time complexity, the time taken to processing goes on increasing linearly as the amount of tickers processed increases (around 476200 ns was the highest for this dataset).
____________________________

<br />

2. On the other hand the update algorithm that uses Deque(s), completes processing a ticker completes processing a ticker with the following stats :
- Mean : 856.36 ns
- Median : 600 ns
- Mode : 600 ns
- SD : 2643 ns <br />
![alt text](https://github.com/shaneg07/roll-OHLC/blob/main/deque.jpg?raw=true)

<br />

### Due to O(1) time complexity, the processing time for each ticker is approximately the same.<br />

____________________________

## About the tests and modified dataset

I have provided two tests in the library that will take the "output files" as their input and compare it with the expected "correct" provided to us.

- The first test compares the output provided 'ohlc-5m-a.txt', with the output produced by this code.
- The second test compares an expected output where some tickers occur with a gap of more than 300_000 ms, and makes sure the open-high-low-close should change accordingly.

- Both these tests pass successfully.


---

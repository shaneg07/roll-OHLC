use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, Write};
use thiserror::Error;
extern crate statistical;
// use std::time::Instant;

// the window is fixed and provided at startup, same for all symbols
// for task, time window = 300,000 ms.

#[derive(Error, Debug)]
enum FileError {
    #[error("Failed to open file: {0}")]
    OpenError(#[source] Error),

    #[error("Failed to read file: {0}")]
    ReadError(#[source] Error),

    #[error("Failed to write file: {0}")]
    WriteError(#[source] Error),
}

fn main() -> Result<(), FileError> {
    //let input = File::open("rolling_ohlc/src/data/dataset-b.txt");

    let input: File = match File::open("rolling_ohlc/src/data/dataset-b.txt") {
        Ok(file) => file,
        Err(e) => return Err(FileError::OpenError(e)),
    };

    let data = ohlc_lib::parse_json(input);

    let mut file: File = match File::create("rolling_ohlc/src/output/ohlc-5m-b_shane.txt") {
        Ok(file) => file,
        Err(e) => return Err(FileError::ReadError(e)),
    };

    // let mut bencher: Vec<f32> = vec![];
    // let mut mode: Vec<i32> = vec![];

    //We know all the symbols at startup.
    let mut turbo_open: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();
    let mut turbo_high: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();
    let mut turbo_low: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();

    let mut fish_open: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();
    let mut fish_high: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();
    let mut fish_low: VecDeque<ohlc_lib::TickerVal> = VecDeque::new();

    for ticker in data {
        // let now: Instant = Instant::now();
        let a: &Decimal = &ticker.a;
        let b: &Decimal = &ticker.b;
        let price: Decimal = (a + b) / dec!(2);
        let maxmize: bool = true;

        // We know the symbols at startup.
        if &ticker.s == "TURBOUSDT" {
            turbo_open = ohlc_lib::calc_open(&ticker, turbo_open, price);
            let open_price: Decimal = turbo_open[0].price;

            turbo_high = ohlc_lib::calc_highlow(&ticker, turbo_high, price, maxmize);
            let high_price: Decimal = turbo_high[0].price;

            turbo_low = ohlc_lib::calc_highlow(&ticker, turbo_low, price, !maxmize);
            let low_price: Decimal = turbo_low[0].price;

            match file.write_all(
                ohlc_lib::format_output(
                    &ticker.s, ticker.t, open_price, high_price, low_price, price,
                )
                .as_bytes(),
            ) {
                Ok(file) => file,
                Err(e) => return Err(FileError::WriteError(e)),
            };
        }

        if &ticker.s == "FISHUSDT" {
            fish_open = ohlc_lib::calc_open(&ticker, fish_open, price);
            let open_price: Decimal = fish_open[0].price;

            fish_high = ohlc_lib::calc_highlow(&ticker, fish_high, price, true);
            let high_price: Decimal = fish_high[0].price;

            fish_low = ohlc_lib::calc_highlow(&ticker, fish_low, price, false);
            let low_price: Decimal = fish_low[0].price;

            match file.write_all(
                ohlc_lib::format_output(
                    &ticker.s, ticker.t, open_price, high_price, low_price, price,
                )
                .as_bytes(),
            ) {
                Ok(file) => file,
                Err(e) => return Err(FileError::WriteError(e)),
            };
        }

        // let elapsed: std::time::Duration = now.elapsed();
        // let output: f32 = elapsed.as_nanos() as f32;
        // bencher.push(output);
        // mode.push(output as i32);
    }

    // let mean = statistical::mean(&bencher);
    // println!("Mean: {}, Median: {}, Mode: {}, \n Standard Deviation: {}", mean, statistical::median(&bencher), statistical::mode(&mode).unwrap() , statistical::standard_deviation(&bencher, Some(mean)));
    Ok(())
}

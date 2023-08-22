use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;

const WINDOW_TIMEFRAME: u128 = 300_000; //in milliseconds

// json_serder parsing struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Ticker {
    #[serde(with = "rust_decimal::serde::float")]
    pub a: Decimal,
    pub s: String,
    pub b: Decimal,
    #[serde(rename = "T")]
    pub t: u128,
}

// struct for output of the parsed data
pub struct TickerVal {
    pub symbol: String,
    pub timestamp: u128,
    pub price: Decimal,
}

// struct to write the output to a json/txt file
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParseOutput {
    pub symbol: String,
    pub timestamp: u128,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}

/*
open = price @ index '0' OR price at timestamp T - WINDOW_TIMEFRAME
close = price currently
high = max price in range T - WINDOW_TIMEFRAME to current
low = min price in range T - WINDOW_TIMEFRAME to current
*/

//{"symbol":{},"timestamp":{},"open":"{}","high":"{}","low":"{}","close":"{}"}

//function to parse the input data
pub fn parse_json(input: File) -> impl Iterator<Item = Ticker> {
    let data = serde_json::Deserializer::from_reader(input)
        .into_iter::<Ticker>()
        .filter_map(|it: Result<Ticker, serde_json::Error>| it.ok());

    data
}

pub fn calc_open(
    ticker: &Ticker,
    mut open_vec: VecDeque<TickerVal>,
    price: Decimal,
) -> VecDeque<TickerVal> {
    if open_vec.is_empty() {
        open_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
    }

    if ticker.t - open_vec[0].timestamp > WINDOW_TIMEFRAME {
        open_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
        while ticker.t - open_vec[0].timestamp > WINDOW_TIMEFRAME {
            open_vec.pop_front();
            if open_vec.is_empty() {
                break;
            }
        }
    }

    if ticker.t - open_vec[0].timestamp <= WINDOW_TIMEFRAME {
        open_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
    }
    open_vec
}

pub fn calc_highlow(
    ticker: &Ticker,
    mut highlow_vec: VecDeque<TickerVal>,
    price: Decimal,
    highlow: bool,
) -> VecDeque<TickerVal> {
    if highlow_vec.is_empty() {
        highlow_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
    }

    if ticker.t - highlow_vec[0].timestamp <= WINDOW_TIMEFRAME {
        while (price > highlow_vec[highlow_vec.len() - 1].price && highlow)
            || (price < highlow_vec[highlow_vec.len() - 1].price && !highlow)
        {
            highlow_vec.pop_back();
            if highlow_vec.is_empty() {
                break;
            }
        }
        highlow_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
    }

    if ticker.t - highlow_vec[0].timestamp > WINDOW_TIMEFRAME {
        while ticker.t - highlow_vec[0].timestamp > WINDOW_TIMEFRAME
            || (price > highlow_vec[highlow_vec.len() - 1].price && highlow)
            || (price < highlow_vec[highlow_vec.len() - 1].price && !highlow)
        {
            highlow_vec.pop_front();
            if highlow_vec.is_empty() {
                break;
            }
        }
        highlow_vec.push_back(TickerVal {
            symbol: ticker.s.clone(),
            timestamp: ticker.t,
            price,
        });
    }

    highlow_vec
}

pub fn format_output(
    name: &String,
    time: u128,
    o: Decimal,
    h: Decimal,
    l: Decimal,
    c: Decimal,
) -> String {
    let mut output = format!(
        r#"{{"symbol":"{}","timestamp":{},"open":"{}","high":"{}","low":"{}","close":"{}"}}"#,
        name, time, o, h, l, c
    );
    output.push('\n');

    output
}

#[cfg(test)]
mod tests {

    use crate::ParseOutput;
    use std::fs::File;

    #[test]
    fn check_against_sample() -> Result<(), std::io::Error> {
        /*
        Test to check if the float values being produced have correct precision.
        Also check if each Ticker has the correct symbol and timestamp, corresponding to their respective tickers in the given correct output.
        ohlc-5m-a_shane is the output produced by this program.
        ohlc-5m-a is the expected correct output.
        */

        let input1: File = match File::open("src/output/ohlc-5m-a_shane.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let input2: File = match File::open("src/output/ohlc-5m-a.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let data1 = serde_json::Deserializer::from_reader(input1)
            .into_iter::<ParseOutput>()
            .filter_map(|it| it.ok());
        let data2 = serde_json::Deserializer::from_reader(input2)
            .into_iter::<ParseOutput>()
            .filter_map(|it| it.ok());

        let mut result1: Vec<ParseOutput> = vec![];
        let mut result2: Vec<ParseOutput> = vec![];

        for ticker in data1 {
            result1.push(ticker);
        }

        for ticker in data2 {
            result2.push(ticker);
        }

        //Check both files processed an equal number of tickers.
        assert_eq!(result1.len(), result2.len());

        for i in 0..result1.len() {
            assert_eq!(result1[i], result2[i]);
        }
        Ok(())
    }

    #[test]
    fn check_against_testdata() -> Result<(), std::io::Error> {
        /*
        Here we use modified data to test if the function produces correct rolling outputs.
        Some tickers in this dataset occur with a gap of more than 300_000 ms, so the open-high-low-close should change accordingly.
        mod-ohlc-5m-a_shane is the output produced by this program.
        mod-ohlc-5m-a is the expected correct output.
        */

        let input1: File = match File::open("src/output/mod-ohlc-5m-a_shane.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let input2: File = match File::open("src/output/mod-ohlc-5m-a.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let data1 = serde_json::Deserializer::from_reader(input1)
            .into_iter::<ParseOutput>()
            .filter_map(|it| it.ok());
        let data2 = serde_json::Deserializer::from_reader(input2)
            .into_iter::<ParseOutput>()
            .filter_map(|it| it.ok());

        let mut result1: Vec<ParseOutput> = vec![];
        let mut result2: Vec<ParseOutput> = vec![];

        for ticker in data1 {
            result1.push(ticker);
        }

        for ticker in data2 {
            result2.push(ticker);
        }

        //Check both files processed an equal number of tickers.
        assert_eq!(result1.len(), result2.len());

        for i in 0..result1.len() {
            assert_eq!(result1[i], result2[i]);
        }
        Ok(())
    }
}

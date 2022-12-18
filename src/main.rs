//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::error::Error;

//use tokio::runtime::Builder;
use log::LevelFilter;

mod coffeeshop;
mod company;
mod customernames;
mod machinery;
mod staff;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        //.filter_module("channels", LevelFilter::Info)
        .filter_module("statistics", LevelFilter::Info)
        .filter_module("operations", LevelFilter::Info)
        .init();

    let rt = tokio::runtime::Runtime::new().unwrap();
    // let rt = Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .enable_time()
    //     .build()
    //     .unwrap();
    //let rt = Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(company::open_coffee_shop(coffeeshop::adrians_place));
    Ok(())
}

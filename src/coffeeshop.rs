//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::time::Instant;

use tokio::task;
use tokio::try_join;

use crate::company::{coffeeshop_entry, coffeeshop_exit, Coffee, Customer, Order};
use crate::machinery::{coffee_machine, steam_milk_machine};
use crate::staff::barista;

pub async fn open_coffee_shop() {
    println!("Open coffee shop");
    let opentime = Instant::now();

    let (customer_entry_tx, customer_entry_rx) = async_channel::unbounded::<Customer>();
    let (customer_exit_tx, customer_exit_rx) = async_channel::unbounded::<(Customer, Coffee)>();

    let (coffee_start_tx, coffee_start_rx) = async_channel::unbounded::<Order>();
    let (coffee_ready_tx, coffee_ready_rx) = async_channel::unbounded::<Order>();
    let (milk_start_tx, milk_start_rx) = async_channel::unbounded::<Order>();
    let (milk_ready_tx, milk_ready_rx) = async_channel::unbounded::<Order>();

    let joinclients = task::spawn(coffeeshop_entry(customer_entry_tx, 10.0, 0..100));
    let joincoffestore = task::spawn(coffeeshop_exit(customer_exit_rx));

    let brew_handler = task::spawn(coffee_machine(coffee_start_rx, coffee_ready_tx));
    let milk_handler = task::spawn(steam_milk_machine(milk_start_rx, milk_ready_tx));

    let barista1_handler = task::spawn(barista(
        "Jose",
        customer_entry_rx,
        customer_exit_tx,
        coffee_start_tx,
        coffee_ready_rx,
        milk_start_tx,
        milk_ready_rx,
    ));

    match try_join!(
        joinclients,
        joincoffestore,
        brew_handler,
        milk_handler,
        barista1_handler,
    ) {
        Ok(_) => println!("Finish OK"),
        Err(err) => println!("Finish Error {}", err),
    }
    println!(
        "Finish journey in {} milliseconds",
        opentime.elapsed().as_millis()
    );
}

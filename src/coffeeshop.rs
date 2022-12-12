//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use async_channel::{Receiver, Sender};
use tokio::task::JoinSet;

use crate::company::{Coffee, Customer, Order};
use crate::machinery::{coffee_machine, steam_milk_machine};
use crate::staff::barista;

pub fn adrians_place(
    joinset: &mut JoinSet<()>,
    customer_exit_tx: Sender<(Customer, Coffee)>,
    customer_entry_rx: Receiver<Customer>,
) {
    let (coffee_start_tx, coffee_start_rx) = async_channel::unbounded::<Order>();
    let (coffee_ready_tx, coffee_ready_rx) = async_channel::unbounded::<Order>();
    let (milk_start_tx, milk_start_rx) = async_channel::unbounded::<Order>();
    let (milk_ready_tx, milk_ready_rx) = async_channel::unbounded::<Order>();

    joinset.spawn(coffee_machine(coffee_start_rx, coffee_ready_tx));
    joinset.spawn(steam_milk_machine(milk_start_rx, milk_ready_tx));

    joinset.spawn(barista(
        "Jose",
        customer_entry_rx,
        customer_exit_tx,
        coffee_start_tx,
        coffee_ready_rx,
        milk_start_tx,
        milk_ready_rx,
    ));
}

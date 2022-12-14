//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::ops::Range;
use std::time::Duration;
use std::time::Instant;

use async_channel::{Receiver, Sender};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Exp};
use tokio::task::JoinSet;
use tokio::time::sleep;

use crate::customernames::generate_name;

#[derive(Debug)]
pub struct Customer {
    pub name: String,
    pub time: Instant,
}

#[derive(Debug)]
pub struct Coffee {
    pub brew: bool,
    pub milk: bool,
    pub time: Instant,
}
impl Coffee {
    pub fn ready(&self) -> bool {
        self.brew && self.milk
    }
}

#[derive(Debug)]
pub struct Order {
    pub customer: Customer,
    pub coffee: Coffee,
}

pub async fn open_coffee_shop<F>(shop: F)
where
    F: Fn(&mut JoinSet<()>, Sender<(Customer, Coffee)>, Receiver<Customer>) -> (),
{
    log::info!(target: "operations", "Abrimos la cafetería");

    let opentime = Instant::now();

    let (customer_entry_tx, customer_entry_rx) = async_channel::unbounded::<Customer>();
    let (customer_exit_tx, customer_exit_rx) = async_channel::unbounded::<(Customer, Coffee)>();

    let mut joinset = JoinSet::new();

    joinset.spawn(coffeeshop_queues(customer_entry_rx.clone()));

    joinset.spawn(coffeeshop_entry(customer_entry_tx, 10.0, 0..100));
    joinset.spawn(coffeeshop_exit(customer_exit_rx));

    // logging cada x tiempo del estado de las colas

    shop(&mut joinset, customer_exit_tx, customer_entry_rx);

    while let Some(res) = joinset.join_next().await {
        res.unwrap();
    }

    log::info!(target: "operations", "Cerramos la cafetería");
    log::info!(target: "statistics", "Tiempo de cafetería abierta: {}", opentime.elapsed().as_millis());
}

async fn coffeeshop_entry(
    customer_entry_tx: Sender<Customer>,
    entries_per_second: f64,
    customers: Range<i32>,
) {
    let exp = Exp::new(entries_per_second / 1000.0).unwrap();
    let mut rnd = StdRng::from_entropy();
    for _ in customers {
        sleep(Duration::from_millis(exp.sample(&mut rnd) as u64)).await;
        let new_customer = Customer {
            name: String::from(generate_name(&mut rnd)),
            time: Instant::now(),
        };
        log::info!(
            target: "operations",
            "{} entra por un café.",
            new_customer.name
        );

        customer_entry_tx.send(new_customer).await.unwrap();
    }
}

async fn coffeeshop_exit(customer_exit_rx: Receiver<(Customer, Coffee)>) {
    while let Ok((customer, coffee)) = customer_exit_rx.recv().await {
        assert!(coffee.ready());
        log::info!(
            target: "statistics", 
            "Tiempo en atender al cliente: {}", 
            customer.time.elapsed().as_millis());
        log::info!(
            target: "operations",
            "{} sale satisfecho con su café en la mano.",
            customer.name
        );
    }
}

async fn coffeeshop_queues(customer_entry_rx: Receiver<Customer>) {
    loop {
        if customer_entry_rx.is_closed() && customer_entry_rx.is_empty() {
            return;
        }
        sleep(Duration::from_millis(500)).await; // tiempo de elaboración
        log::info!(
            target: "channels",
            "Longitud de la cola de clientes: {}.",
            customer_entry_rx.len()
        );
    }
}

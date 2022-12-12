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
    enter: Instant,
}
#[derive(Debug)]
pub struct Coffee {
    pub brew: bool,
    pub milk: bool,
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
    println!("Opening coffee shop");
    let opentime = Instant::now();

    let (customer_entry_tx, customer_entry_rx) = async_channel::unbounded::<Customer>();
    let (customer_exit_tx, customer_exit_rx) = async_channel::unbounded::<(Customer, Coffee)>();

    let mut joinset = JoinSet::new();

    joinset.spawn(coffeeshop_entry(customer_entry_tx, 10.0, 0..100));
    joinset.spawn(coffeeshop_exit(customer_exit_rx));

    shop(&mut joinset, customer_exit_tx, customer_entry_rx);

    while let Some(res) = joinset.join_next().await {
        res.unwrap();
    }

    println!(
        "Closing coffee shop in {} milliseconds",
        opentime.elapsed().as_millis()
    );
}

async fn coffeeshop_entry(
    customer_entry_tx: Sender<Customer>,
    entries_per_second: f64,
    size: Range<i32>,
) {
    let exp = Exp::new(entries_per_second / 1000.0).unwrap();
    let mut rnd = StdRng::from_entropy();
    for _ in size {
        sleep(Duration::from_millis(exp.sample(&mut rnd) as u64)).await;
        let new_customer = Customer {
            name: String::from(generate_name(&mut rnd)),
            enter: Instant::now(),
        };
        println!("   --> Cliente entra por un café: {:?}", new_customer);
        customer_entry_tx.send(new_customer).await.unwrap();
    }
}

async fn coffeeshop_exit(customer_exit_rx: Receiver<(Customer, Coffee)>) {
    while let Ok(customercoffee) = customer_exit_rx.recv().await {
        println!(
            "   --> Cliente satisfecho en {} milisegundos: {:?} {:?}",
            customercoffee.0.enter.elapsed().as_millis(),
            customercoffee.0,
            customercoffee.1
        );
    }
}

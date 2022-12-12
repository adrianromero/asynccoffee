//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::ops::Range;
use std::time::Duration;
use std::time::Instant;

use async_channel::{Receiver, Sender};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Exp};
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

pub async fn coffeeshop_entry(
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

pub async fn coffeeshop_exit(customer_exit_rx: Receiver<(Customer, Coffee)>) {
    while let Ok(customercoffee) = customer_exit_rx.recv().await {
        println!(
            "   --> Cliente satisfecho en {} milisegundos: {:?} {:?}",
            customercoffee.0.enter.elapsed().as_millis(),
            customercoffee.0,
            customercoffee.1
        );
    }
}
//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use crate::company::Order;
use async_channel::{Receiver, Sender};
use std::time::Duration;
use tokio::time::sleep;

pub async fn coffee_machine(order_rx: Receiver<Order>, order_tx: Sender<Order>) {
    while let Ok(mut order) = order_rx.recv().await {
        log::info!(target: "statistics", "Preparando el café de {}", order.customer.name);

        sleep(Duration::from_millis(200)).await; // tiempo de moltura
        order.coffee.brew = true;

        log::info!(target: "statistics", "Preparado el café de {}", order.customer.name);
        order_tx.send(order).await.unwrap();
    }
}

pub async fn steam_milk_machine(order_rx: Receiver<Order>, order_tx: Sender<Order>) {
    while let Ok(mut order) = order_rx.recv().await {
        log::info!(target: "statistics", "Calentando la leche de {}", order.customer.name);

        sleep(Duration::from_millis(200)).await; // tiempo de elaboración
        order.coffee.milk = true;

        log::info!(target: "statistics", "Calentada la leche de {}", order.customer.name);
        order_tx.send(order).await.unwrap();
    }
}

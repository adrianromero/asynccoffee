//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

use async_channel::{Receiver, Sender};

use crate::company::{Coffee, Customer, Order};

pub async fn create_order(customer: Customer) -> Order {
    sleep(Duration::from_millis(100)).await; // tiempo en tomar la orden y cobrar
    Order {
        customer: customer,
        coffee: Coffee {
            brew: false,
            milk: false,
            time: Instant::now(),
        },
    }
}

pub async fn barista(
    name: &str,
    customer_entry_rx: Receiver<Customer>,
    customer_exit_tx: Sender<(Customer, Coffee)>,
    coffee_start_tx: Sender<Order>,
    coffee_ready_rx: Receiver<Order>,
    milk_start_tx: Sender<Order>,
    milk_ready_rx: Receiver<Order>,
) {
    while let Ok(customer) = customer_entry_rx.recv().await {
        log::info!(target: "operations", "Bienvenido {} le atiende {}", customer.name, name);

        let new_order = create_order(customer).await;

        coffee_start_tx.send(new_order).await.unwrap();

        let coffee_order = coffee_ready_rx.recv().await.unwrap();

        milk_start_tx.send(coffee_order).await.unwrap();

        let coffee_milk_order = milk_ready_rx.recv().await.unwrap();

        log::info!(
            target: "statistics", 
            "Tiempo en preparar el café: {}", 
            coffee_milk_order.coffee.time.elapsed().as_millis());
        log::info!(
            target: "operations",
            "Gracias por su visita {}, vuelva pronto!",
            coffee_milk_order.customer.name
        );
        customer_exit_tx
            .send((coffee_milk_order.customer, coffee_milk_order.coffee))
            .await
            .unwrap();
    }
}

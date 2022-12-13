//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use crate::company::{Coffee, Customer, Order};
use async_channel::{Receiver, Sender};

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
        log::info!(target: "statistics", "Bienvenido {} le atiende {}", customer.name, name);

        let new_order = Order {
            customer: customer,
            coffee: Coffee {
                brew: false,
                milk: false,
            },
        };

        coffee_start_tx.send(new_order).await.unwrap();

        let coffee_order = coffee_ready_rx.recv().await.unwrap();

        milk_start_tx.send(coffee_order).await.unwrap();

        let coffee_milk_order = milk_ready_rx.recv().await.unwrap();

        log::info!(
            target: "statistics",
            "Gracias por su visita {}, vuelva pronto!",
            coffee_milk_order.customer.name
        );
        customer_exit_tx
            .send((coffee_milk_order.customer, coffee_milk_order.coffee))
            .await
            .unwrap();
    }
}

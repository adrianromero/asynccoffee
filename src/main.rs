//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::error::Error;
use std::option::Option;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::SeedableRng;
// use rand_distr::{Distribution, Exp, Uniform};

use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task;
use tokio::time::sleep;
use tokio::try_join;

mod customernames;
use customernames::generate_name;

#[derive(Debug)]
pub struct Customer {
    name: Arc<String>,
    enter: Instant,
    coffee: Option<Coffee>,
}

#[derive(Debug)]
pub struct Coffee {
    label: Arc<String>,
    grinded: bool,
    brewed: bool,
    // steamed: bool,
    // ready: bool,
}

async fn coffee_shop() {
    let ten_millis = Duration::from_millis(100);

    // sleep(ten_millis).await;
    // let mut rnd = StdRng::from_entropy();
    // let exp = Exp::new(1.0).unwrap();
    // println!("{} is from a Exp(1) distribution", exp.sample(&mut rnd));
    // let uni: Uniform<i32> = Uniform::new(0, 10);
    // println!("{} is from a Uniform distribution", uni.sample(&mut rnd));

    println!("Open coffee shop");
    let opentime = Instant::now();

    let (client_tx, client_rx) = mpsc::unbounded_channel::<Customer>();
    let (client_served_tx, mut client_served_rx) = mpsc::unbounded_channel::<Customer>();
    let (start_tx, grinder_rx) = mpsc::unbounded_channel::<Coffee>();
    let (grinder_tx, brewer_rx) = mpsc::unbounded_channel::<Coffee>();
    let (brewer_tx, finish_rx) = mpsc::unbounded_channel::<Coffee>();

    let joinclients = task::spawn(async move {
        let mut rnd = StdRng::from_entropy();
        for _ in 0..10 {
            sleep(ten_millis).await;
            let new_customer = Customer {
                name: String::from(generate_name(&mut rnd)).into(),
                enter: Instant::now(),
                coffee: None,
            };
            println!("   --> Cliente entra por un café: {:?}", new_customer);
            client_tx.send(new_customer)?;
        }
        Ok::<(), SendError<Customer>>(())
    });
    let joincoffestore = task::spawn(async move {
        while let Some(customer) = client_served_rx.recv().await {
            println!(
                "   --> Cliente satisfecho en {} milisegundos: {:?}",
                customer.enter.elapsed().as_millis(),
                customer
            );
        }
        Ok::<(), SendError<Customer>>(())
    });

    let grinder_handler = task::spawn(grinder_machine(grinder_rx, grinder_tx));
    let brewer_handler = task::spawn(brewer_machine(brewer_rx, brewer_tx));

    let barista_handler = task::spawn(barista(client_rx, client_served_tx, start_tx, finish_rx));

    match try_join!(
        joinclients,
        joincoffestore,
        grinder_handler,
        brewer_handler,
        barista_handler
    ) {
        Ok(_) => println!("Finish OK"),
        Err(err) => println!("Finish Error {}", err),
    }
    println!(
        "Finish journey in {} milliseconds",
        opentime.elapsed().as_millis()
    );
}

async fn grinder_machine(
    mut grinder_rx: UnboundedReceiver<Coffee>,
    grinder_tx: UnboundedSender<Coffee>,
) {
    while let Some(mut coffee) = grinder_rx.recv().await {
        let name = coffee.label.clone();
        println!("moliendo el café de  {:?}", name);

        sleep(Duration::from_millis(200)).await; // tiempo de moltura
        coffee.grinded = true;

        grinder_tx.send(coffee).unwrap();
        println!("molido el café de   {:?}", name);
    }
}

async fn brewer_machine(
    mut brewer_rx: UnboundedReceiver<Coffee>,
    brewer_tx: UnboundedSender<Coffee>,
) {
    while let Some(mut coffee) = brewer_rx.recv().await {
        let label = coffee.label.clone();
        println!("Haciendo el café de  {:?}", label);

        sleep(Duration::from_millis(200)).await; // tiempo de elaboración
        coffee.brewed = true;

        brewer_tx.send(coffee).unwrap();
        println!("Hecho el café de   {:?}", label);
    }
}

async fn barista(
    mut client_rx: UnboundedReceiver<Customer>,
    client_served_tx: UnboundedSender<Customer>,
    start_tx: UnboundedSender<Coffee>,
    mut brewer_rx: UnboundedReceiver<Coffee>,
) {
    while let Some(mut customer) = client_rx.recv().await {
        println!("Bienvenido  {:?}", customer.name);

        let new_coffee = Coffee {
            label: customer.name.clone(),
            grinded: false,
            brewed: false,
            // steamed: false,
            // ready: false,
        };
        start_tx.send(new_coffee).unwrap();

        let coffee = brewer_rx.recv().await.unwrap();

        customer.coffee = Some(coffee);

        println!("Gracias por su visita {:?}, vuelva pronto!", customer.name);
        client_served_tx.send(customer).unwrap();
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     coffee_shop().await;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    //let rt = tokio::runtime::Runtime::new().unwrap();
    // let rt = Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .enable_time()
    //     .build()
    //     .unwrap();
    let rt = Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(coffee_shop());
    Ok(())
}

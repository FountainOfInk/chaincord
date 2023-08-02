// #![allow(dead_code)]
// #![allow(unused_variables)]

mod utils;
mod amongus;
mod config;
mod message;


use amongus::AmongUs;
use eframe::{run_native, NativeOptions};
use tokio::{self, runtime::Runtime};
use std::time::Duration;


fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    run_native("amongus!!", NativeOptions::default(), Box::new(|cc| Box::new(AmongUs::new(cc)))).expect("its over");
}
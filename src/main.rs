mod example;
use std::thread;

fn main() {
    example::a_mod::says();
    // ----------------------------------------------------
    // let result = example::publischer_confirm::start();
    // match result1 {
    //     Ok(v) => println!("working with result"),
    //     Err(e) => println!("error parsing result"),
    // }
    // ----------------------------------------------------
    // rpc example
    let receiver = thread::spawn(move || {
        let result = example::rpc_function::start();
        match result {
            Ok(v) => println!("reciver: working with result"),
            Err(e) => println!("reciver: error parsing result"),
        }
    });
    // rpc call
    let result1 = example::rpc_call::start();
    match result1 {
        Ok(v) => println!("sender: working with result"),
        Err(e) => println!("sender: error parsing result"),
    }
    receiver.join().unwrap();
}

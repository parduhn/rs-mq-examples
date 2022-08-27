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
        example::rpc_function::start();
    });
    // rpc call
    example::rpc_call::start(10);
    example::rpc_call::start(20);
}

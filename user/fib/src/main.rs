#![feature(asm)]
#![no_std]
#![no_main]

mod cr0;

use kernel_api::println;
use kernel_api::syscall::{getpid, time};

fn fib(n: u64) -> u64 {
	//println!("fib {:?}", n);
    match n {
        0 => 1,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    println!("Started...");

    let rtn = fib(40);




    let pid = getpid();
    println!("return pi from user: {:?}", pid);

    let current_time = time();
    println!("time return from user {}", current_time.as_secs());

    println!("Ended: Result = {}", rtn);







}

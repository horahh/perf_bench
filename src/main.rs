
use core::time::Duration;
use std::thread;
fn main() {
    f1();
    f2();
    f3();
    println!("Hello, world!");
}

fn f1() {
    thread::sleep(Duration::from_millis(100));
}
fn f2() {
    thread::sleep(Duration::from_millis(200));
}
fn f3() {
    thread::sleep(Duration::from_millis(300));
    f4()
}

fn f4() {
    thread::sleep(Duration::from_millis(400));
}

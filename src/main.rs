
use core::time::Duration;
use std::thread;

fn main() {
    perf_bench();
}

pub fn perf_bench(){
    for _ in 0..10 {
        f1();
        f2();
        f3();
    }
}

fn f1() {
    thread::sleep(Duration::from_millis(10));
}
fn f2() {
    thread::sleep(Duration::from_millis(20));
}
fn f3() {
    thread::sleep(Duration::from_millis(20));
    f4()
}

fn f4() {
    thread::sleep(Duration::from_millis(20));
}

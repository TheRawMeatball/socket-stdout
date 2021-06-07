mod lib;

fn main() {
    lib::debug_init("127.0.0.1:1923".parse().unwrap());
    loop {
        println!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

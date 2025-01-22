mod lib;

use lib::{lsd, test_android};
use log::*;







fn main() {
    println!("Hello, world!");
    test_android();
    lsd::print_lsd();



    trace!("trace message: is not logged");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message")
}
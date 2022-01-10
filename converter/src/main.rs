use converter::args_parse;
use converter::{execute};

fn main() {
    // parse arguments
    execute(&args_parse::get());
}

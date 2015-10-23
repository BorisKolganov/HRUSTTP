extern crate argparse;
mod HRUSTTP;
use argparse::{ArgumentParser, Store};


fn main() {
    let mut rootdir = "".to_string();
    let mut n_cpu = 0;
    let ip = "127.0.0.1:80".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("From HRUSTTP with love.");
        ap.refer(&mut rootdir)
            .add_option(&["-r", "--rootdir"], Store,
            "Root directory");
        ap.refer(&mut n_cpu)
            .add_option(&["-n", "--ncpu"], Store,
            "Number of cpu");
        ap.parse_args_or_exit();
    }

    HRUSTTP::HRUSTTP::new(rootdir, n_cpu, ip).go();
}

extern crate wifiscanner;

fn main() {
    use wifiscanner;
    println!("{:?}", wifiscanner::scan("wlp2s0"));
}

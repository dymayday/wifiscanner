extern crate wifiscanner;

fn main() {
    use wifiscanner;
    // The 'lo' interface usually refers to the 'localhost' on linux system
    println!("{:?}", wifiscanner::scan("lo"));
}

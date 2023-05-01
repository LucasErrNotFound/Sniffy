use spinners::{Spinner, Spinners};
use pnet::*;
use std::thread::sleep;
use std::time::Duration;

pub fn scan_packets() {
    let mut loading = Spinner::new(Spinners::Dots, "Sniffy is now sniffing packets".into());
    sleep(Duration::from_secs(3));
    loading.stop_with_newline();
}

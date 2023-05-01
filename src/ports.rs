use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

use pnet::*;

pub fn scan_ports() {
    let mut loading = Spinner::new(Spinners::BouncingBall, "Sniffy is now sniffing ports".into());
    sleep(Duration::from_secs(3));
    loading.stop_with_newline();
}

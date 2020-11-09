#[macro_use]
extern crate honggfuzz;

use sequoia_openpgp::parse::{Parse, PacketParser};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() > 0 {
                let _ = PacketParser::from_bytes(data);
            }
        });
    }
}

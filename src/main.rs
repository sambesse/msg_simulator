/**
* This program is meant to simulate a large number of messages
* sent over a network with variable periodicity and dynamically
* modifiable data
*
* e.g. message A has three data fields and is transmitted every 10ms
*   message B has two data fields and is transmitted every second
*   This program should send those packest at their specified rates
*   and allow a user to modify the fields via a terminal interface
*
* this is currently designed for simulating CAN messages, but would be
* nice to give the flexibility to allow any type of packet over any
* interface
*/
use nix; //for now just do the whole crate
use nix::fcntl;
use nix::fcntl::{OFlag, open};
use std::os::fd::RawFd;

enum PacketType {
    Periodic, // sent every period
    OnChange, // only sent when a data signal changes
}

struct DataSignalDesc {
    start_bit: u16, //soft limits packet length to 8KB
    length: u16,
}

struct MessageDesc {
    data: Vec<u8>,
    signals: Vec<DataSignalDesc>,
    packet_type: PacketType,
    period: u32,
    interface: RawFd,
}

fn main() {
    println!("Hello, world!");

    match init_can_interface("can0") {}
}

/* takes the can interface name e.g. can0 and returns a successfully opened
* interface fd, else the errno returned by the attempt */
fn init_can_interface(interface_name: &str) -> Result<RawFd, i32> {}

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
use embedded_can::{Frame, StandardId};
use socketcan::{CanFdSocket, CanFrame, Result, Socket};
use std::env;

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
    interface: CanFdSocket,
}

fn main() {
    println!("Hello, world!");

    let socket_tx = match CanFdSocket::open("can0".into()) {
        Ok() =>
    }
}

/* takes the can interface name e.g. can0 and returns a successfully opened
* interface fd, else the errno returned by the attempt */

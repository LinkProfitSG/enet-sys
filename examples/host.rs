extern crate enet_sys;

use enet_sys::ENetAddress;
use enet_sys::{enet_deinitialize, enet_initialize};
use enet_sys::{enet_host_create, enet_host_destroy};

use std::net::Ipv4Addr;

fn main() {
    println!("Starting test of host creation...");
    if unsafe { enet_initialize() } < 0 {
        panic!("Error on enet initialization.");
    }

    println!("Enet initialized.");

    println!("creating ENet server host...");

    // default localhost on port 12345
    let address = ENetAddress {
        type_: 0,
        host: enet_sys::_ENetAddress__bindgen_ty_1 { v4: Ipv4Addr::new(127, 0, 0, 1).octets() },
        port: 8080,
    };

    println!("Creating host for IP {:?}", Ipv4Addr::from(Ipv4Addr::LOCALHOST).octets());

    unsafe {
        let server = enet_host_create(
            0,
            &address, // address to bind the server host to
            32,       // allow up to 32 clients and/or outgoing connections
            2,        // allow up to 2 channels to be used, 0 and 1
            0,        // assume any amount of incoming bandwidth
            0,
        ); // assume any amount of outgoing bandwidth

        if server.is_null() {
            panic!("Unable to create server host")
        }

        println!("...ENet server host created");

        println!("server: {:p}", server);
        println!("server peers: {:p}", server.as_ref().unwrap().peers);
        println!("server &packetData: {:p}", &(*server).packetData);
        println!("server &serviceTime: {:p}", &(*server).serviceTime);
        println!("server channelLimit: {}", (*server).channelLimit);
        println!("server peerCount: {}", server.as_ref().unwrap().peerCount);
        println!("server connectedPeers: {}", (*server).connectedPeers);
        println!("server randomSeed: {:?}", (*server).randomSeed);
        println!("server socket: {:?}", (*server).socket);

        enet_host_destroy(server);
    }

    unsafe {
        enet_deinitialize();
    }
    println!("Enet deinitialized.");
}

use std::io;
use std::thread;
use std::net::UdpSocket;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::str::from_utf8;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;

use serde;
use serde_json;
use net2::UdpBuilder;

const INTERVAL_NS: u32 = 300_000_000; // 300 ms
const TIMEOUT_NS: u32 = 1_000_000_000; // 1000 ms
const N_REDUNDANCY: u32 = 4;

#[derive(Debug)]
pub struct PeerUpdate<T> {
    pub peers: Vec<T>,
    pub new: Option<T>,
    pub lost: Vec<T>,
}

impl<T> PeerUpdate<T>
    where T: Ord
{
    pub fn new() -> Self {
        PeerUpdate {
            peers: Vec::new(),
            new: None,
            lost: Vec::new(),
        }
    }

    pub fn add_peers(&mut self, id: T) {
        self.peers.push(id);
    }

    pub fn set_new(&mut self, id: T) {
        self.new = Some(id);
    }

    pub fn add_lost(&mut self, id: T) {
        self.lost.push(id);
    }

    fn sort(&mut self) {
        self.peers.sort();
        self.lost.sort();
    }
}

impl<T: fmt::Display> fmt::Display for PeerUpdate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Peer update:\n"));
        match self.peers.len() {
            0 => try!(write!(f, "\tpeers: []\n")),
            1 => try!(write!(f, "\tpeers: [{}]\n", self.peers[0])),
            n @ _ => {
                try!(write!(f, "\tpeers: [{},\n", self.peers[0]));
                for i in 1..n - 1 {
                    try!(write!(f, "\t        {},\n", self.peers[i]));
                }
                try!(write!(f, "\t        {}]\n", self.peers[n - 1]));
            }
        }
        match self.new {
            Some(ref new) => try!(write!(f, "\tnew:   [{}]\n", new)),
            None => try!(write!(f, "\tnew:   [None]\n")),
        }
        match self.lost.len() {
            0 => try!(write!(f, "\tlost:  []\n")),
            1 => try!(write!(f, "\tlost:  [{}]\n", self.lost[0])),
            n @ _ => {
                try!(write!(f, "\tlost:  [{},\n", self.lost[0]));
                for i in 1..n - 1 {
                    try!(write!(f, "\t        {},\n", self.lost[i]));
                }
                try!(write!(f, "\t        {}]\n", self.lost[n - 1]));
            }
        }
        Ok(())
    }
}

pub struct PeerTransmitter {
    conn: UdpSocket,
    enabled: Mutex<bool>,
}

impl PeerTransmitter {
    pub fn new(port: u16) -> io::Result<Self> {
        let conn = {
            let udp = try!(UdpBuilder::new_v4());
            try!(udp.reuse_address(true));
            let socket = try!(udp.bind("0.0.0.0:0"));
            try!(socket.set_broadcast(true));
            try!(socket.connect(("255.255.255.255", port)));
            socket
        };
        Ok(PeerTransmitter {
               conn: conn,
               enabled: Mutex::new(true),
           })
    }

    pub fn enable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = true;
    }

    pub fn disable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = false;
    }

    pub fn transmit<'a, T>(&self, data: &'a T) -> io::Result<()>
        where T: serde::ser::Serialize
    {
        let serialized = serde_json::to_string(&data).unwrap();
        for _ in 0..N_REDUNDANCY {
            try!(self.conn.send(serialized.as_bytes()));
        }

        Ok(())
    }

    pub fn run<'a, T>(self, i_am_stuck_rx: mpsc::Receiver<()>, data: &'a T) -> !
        where T: serde::ser::Serialize
    {
        loop {
            thread::sleep(Duration::new(0, INTERVAL_NS));

            if let Some(_) = i_am_stuck_rx.try_iter().last() {
                thread::sleep(Duration::from_millis(1000));
            }

            let enabled = self.enabled.lock().unwrap();
            if !*enabled {
                continue;
            }

            drop(enabled);
            self.transmit(data).unwrap_or_else(|_| {});
        }
    }
}


pub struct PeerReceiver {
    conn: UdpSocket,
}

impl PeerReceiver {
    pub fn new(port: u16) -> io::Result<Self> {
        let conn = {
            let udp = try!(UdpBuilder::new_v4());
            try!(udp.reuse_address(true));
            let socket = try!(udp.bind(("255.255.255.255", port)));
            try!(socket.set_broadcast(true));
            socket
        };
        Ok(PeerReceiver { conn: conn })
    }

    pub fn receive<T>(&self) -> io::Result<T>
        where T: serde::de::Deserialize
    {
        let mut buf = [0u8; 256];
        let (amt, _) = try!(self.conn.recv_from(&mut buf));
        let msg = from_utf8(&buf[..amt]).unwrap();
        Ok(serde_json::from_str(&msg).expect(format!("deser failed: {}", msg).as_str()))
    }

    pub fn run<T>(self, update_tx: mpsc::Sender<PeerUpdate<T>>) -> !
        where T: serde::de::Deserialize + Hash + Eq + Clone + Ord
    {
        let mut last_seen = HashMap::new();
        loop {
            let mut peer_update = PeerUpdate::new();
            let mut updated = false;

            self.conn.set_read_timeout(Some(Duration::new(0, TIMEOUT_NS))).unwrap();

            let new_id: T = match self.receive() {
                Ok(id) => id,
                Err(_) => {
                    continue;
                }
            };

            // Adding new connection
            if !last_seen.contains_key(&new_id) {
                peer_update.set_new(new_id.clone());
                updated = true;
            }
            last_seen.insert(new_id, Instant::now());

            // Removing dead connection
            for (id, time) in &last_seen {
                if Instant::now().duration_since(*time) > Duration::new(0, TIMEOUT_NS) {
                    peer_update.add_lost(id.clone());
                    updated = true;
                }
            }
            for id in &peer_update.lost {
                last_seen.remove(id);
            }

            // Sending update
            if updated {
                for (id, _) in &last_seen {
                    peer_update.add_peers(id.clone());
                }
                peer_update.sort();
                update_tx.send(peer_update).expect("Could not send peer update");
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::mpsc::channel;
    use localip::get_localip;

    #[test]
    fn it_works() {
        let port = 9887;
        thread::spawn(move || {
                          let id = format!("{}:{}", get_localip().unwrap(), "unique");
                          let transmitter = PeerTransmitter::new(port).unwrap();
                          transmitter.run(&id);
                      });

        let (tx, rx) = channel::<PeerUpdate<String>>();
        thread::spawn(move || {
                          let receiver = PeerReceiver::new(port).unwrap();
                          receiver.run(tx);
                      });

        // Send multiple times for redundancy
        for _ in 0..10 {
            let update = rx.recv().unwrap();
            println!("Peer update");
            println!("\tPeers:\t{:?}", update.peers);
            println!("\tNew:\t{:?}", update.new);
            println!("\tLost:\t{:?}", update.lost);
        }
    }
}

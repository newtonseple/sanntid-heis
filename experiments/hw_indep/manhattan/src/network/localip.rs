use std::env::args;
use std::net::{TcpStream, IpAddr};
use std::io::Result;
use std::sync::Mutex;

lazy_static! {
    static ref LOCAL_IP: Mutex<Option<IpAddr>> = Mutex::new(None);
}

pub fn get_localip() -> Result<String> {
    let mut local_ip = LOCAL_IP.lock().unwrap();
    let old_ip = local_ip.clone();
    
    
    let id_suffix = if let Some(index) = args().position(|s| s == "--id") {
        args().nth(index+1).expect("id expected after --id")
    } else {
        "0".to_string()
    };

    match old_ip {
        None => {
            let socket = try!(TcpStream::connect("8.8.8.8:53"));
            let ip = try!(socket.local_addr()).ip();
            *local_ip = Some(ip);
            Ok((ip.to_string()+":"+&id_suffix))
        }
        Some(ip) => Ok((ip.to_string()+":"+&id_suffix)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn check_connection() {
        assert_eq!(TcpStream::connect("8.8.8.8:53").is_ok(), true);
    }

    #[test]
    fn it_works() {
        //assert_eq!(LOCAL_IP.lock().unwrap().is_none(), true);
        // this will sometimes fail, as I cannot figure out how to control the test order
        let ip1 = get_localip();
        assert_eq!(ip1.is_ok(), true);
        assert_eq!(LOCAL_IP.lock().unwrap().is_some(), true);
        let ip2 = get_localip();
        assert_eq!(ip2.is_ok(), true);
        assert_eq!(ip1.unwrap(), ip2.unwrap());
    }
}

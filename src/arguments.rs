use clap::Parser;
use std::any::Any;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 200)]
    max_thread: u32,

    #[arg(short, long)]
    include_ip_file: Option<PathBuf>,

    #[arg(short, long)]
    exclude_ip_file: Option<PathBuf>,

    #[arg(short, long)]
    include_ip_range: Option<String>,

    #[arg(short, long)]
    exclude_ip_range: Option<String>,

    #[arg(short, long)]
    include_ip: Option<Vec<SocketAddr>>,

    #[arg(short, long)]
    exclude_ip: Option<Vec<SocketAddr>>,
}

#[derive(Clone, Debug)]
pub struct IpRange {
    start: IpAddr,
    stop: IpAddr,
    port: u16,
}

impl IpRange {
    fn check_same_ip_type(&self) -> Result<(), String> {
        let start_is_ipv4 = self.start.is_ipv4();
        let stop_is_ipv4 = self.stop.is_ipv4();
        if start_is_ipv4 == stop_is_ipv4 {
            return Ok(());
        }
        let error_message = format!(
            "Addresses type don't match !\n
            start address type is {}.\n
            stop address type is {}.\n",
            if start_is_ipv4 { "IPv4" } else { "IPv6" },
            if stop_is_ipv4 { "IPv4" } else { "IPv6" },
        );
        Err(error_message)
    }
    fn check_is_global(&self) -> Result<(), String> {
        let start_is_unspecified = self.start.is_unspecified();
        let stop_is_unspecified = self.stop.is_unspecified();
        let start_is_multicat = self.start.is_multicast();
        let stop_is_multicast = self.stop.is_multicast();

        if start_is_unspecified {
            return Err(String::from("start address is an unspecified address!"));
        }

        if stop_is_unspecified {
            return Err(String::from("stop address is an unspecified address!"));
        }

        if start_is_multicat {
            return Err(String::from("start address is a multicast address!"));
        }

        if stop_is_multicast {
            return Err(String::from("stop address is a multicast address!"));
        }
        Ok(())
    }

    fn cmp_ip(&self) -> Result<(), String> {
        let start_socket = SocketAddr::new(self.start, 0);
        let stop_socket = SocketAddr::new(self.stop, 0);
        match start_socket.ip().cmp(&stop_socket.ip()) {
            std::cmp::Ordering::Greater => Err(String::from(format!(
                "Address {} has a greater value than {}",
                start_socket, stop_socket,
            ))),
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => return Ok(()),
        }
    }

    pub fn check_range(&self) -> Result<(), String> {
        self.check_same_ip_type()?;
        self.check_is_global()?;
        self.cmp_ip()?;
        Ok(())
    }
}

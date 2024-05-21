use serialport::available_ports;
use std::io;

#[derive(serde::Serialize)]
pub struct SerialPortInfo {
    pub port_name: String,
    // Include other fields as needed
}

pub fn get_available_ports() -> io::Result<Vec<SerialPortInfo>> {
    let ports = available_ports()?;
    Ok(ports.into_iter().map(|port| SerialPortInfo {
        port_name: port.port_name,
        // Map other fields as needed
    }).collect())
}

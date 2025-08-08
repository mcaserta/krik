pub fn get_network_interfaces() -> Vec<String> {
    let mut interfaces = vec!["127.0.0.1".to_string()];
    if let Ok(local_ip) = local_ip_address::local_ip() {
        if local_ip.to_string() != "127.0.0.1" {
            interfaces.push(local_ip.to_string());
        }
    }
    if let Ok(network_interfaces) = local_ip_address::list_afinet_netifas() {
        for (_name, ip) in network_interfaces {
            let ip_str = ip.to_string();
            if !ip_str.starts_with("127.") && !ip_str.starts_with("169.254.") && !interfaces.contains(&ip_str) {
                interfaces.push(ip_str);
            }
        }
    }
    interfaces
}



#[derive(Clone)]
pub struct Host {
    protocol: String,
    host_addr: String,
    port: String,
}

impl Host {
    pub fn new(protocol: String, host_addr: String, port: String) -> Host {
        Host {
            protocol,
            host_addr,
            port,
        }
    }

    fn port_suffix(&self) -> String {
        if !self.port.is_empty() {
            format!(":{}", self.port)
        } else {
            String::new()
        }
    }

    pub fn endpoint(&self) -> String {
        format!(
            "{}//{}{}",
            self.protocol,
            self.host_addr,
            self.port_suffix()
        )
    }
    pub fn tendermint_endpoint(&self) -> String {
        format!(
            "{}//tendermint.{}{}",
            self.protocol,
            self.host_addr,
            self.port_suffix()
        )
    }
    pub fn grpc_endpoint(&self) -> String {
        format!(
            "{}//grpc.{}{}",
            self.protocol,
            self.host_addr,
            self.port_suffix()
        )
    }
    pub fn faucet_endpoint(&self) -> String {
        format!(
            "{}//faucet.{}{}",
            self.protocol,
            self.host_addr,
            self.port_suffix()
        )
    }
}

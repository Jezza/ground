//! Implements shared service configuration clap struct
//! This can be flattened within clap commands, or used directly.
//! It provides a simple standard that can be adhered to throughout the different crates.

/// Service network configuration
#[derive(Debug)]
#[cfg_attr(feature = "env", derive(ground_env::FromEnv))]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", clap(next_help_heading = "SERVICE", term_width = 200))]
pub struct ServiceArgs {
    /// The port the service is listening on
    #[cfg_attr(feature = "env", env(rename = "PORT"))]
    #[cfg_attr(feature = "clap", clap(long, env = "SERVICE_PORT"))]
    pub port: u16,

    /// The listen address of this service
    #[cfg_attr(feature = "env", env(rename = "HOST_ADDR", default = "::"))]
    #[cfg_attr(feature = "clap", clap(long, env = "SERVICE_HOST_ADDR", default_value_t = std::net::IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED)))]
    pub host_addr: std::net::IpAddr,
}

impl ServiceArgs {
    pub fn socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.host_addr, self.port)
    }
}

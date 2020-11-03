use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    num::NonZeroU16,
    str::FromStr,
};
use thiserror::Error;
use url::{ParseError as UrlParseError, Url};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DomainWithPort {
    domain: Box<str>,
    port: Option<NonZeroU16>,
}

impl DomainWithPort {
    #[inline]
    pub fn new(domain: impl Into<Box<str>>) -> Self {
        DomainWithPort {
            domain: domain.into(),
            port: None,
        }
    }

    #[inline]
    pub fn new_with_port(domain: impl Into<Box<str>>, port: u16) -> Self {
        DomainWithPort {
            domain: domain.into(),
            port: NonZeroU16::new(port),
        }
    }

    #[inline]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    #[inline]
    pub fn port(&self) -> Option<NonZeroU16> {
        self.port
    }

    #[inline]
    pub fn into_domain_and_port(self) -> (String, Option<NonZeroU16>) {
        (self.domain.into(), self.port)
    }
}

impl From<Box<str>> for DomainWithPort {
    #[inline]
    fn from(domain: Box<str>) -> Self {
        Self::new(domain)
    }
}

impl From<(Box<str>, u16)> for DomainWithPort {
    #[inline]
    fn from(domain_with_port: (Box<str>, u16)) -> Self {
        Self::new_with_port(domain_with_port.0, domain_with_port.1)
    }
}

impl From<String> for DomainWithPort {
    #[inline]
    fn from(domain: String) -> Self {
        Self::new(domain)
    }
}

impl From<(String, u16)> for DomainWithPort {
    #[inline]
    fn from(domain_with_port: (String, u16)) -> Self {
        Self::new_with_port(domain_with_port.0, domain_with_port.1)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DomainWithPortParseError {
    #[error("invalid port number")]
    InvalidPort,
    #[error("empty host")]
    EmptyHost,
    #[error("invalid domain character")]
    InvalidDomainCharacter,
}

impl FromStr for DomainWithPort {
    type Err = DomainWithPortParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(&format!("https://{}/", s)).map_err(|err| match err {
            UrlParseError::InvalidPort => DomainWithPortParseError::InvalidPort,
            UrlParseError::EmptyHost => DomainWithPortParseError::EmptyHost,
            _ => DomainWithPortParseError::InvalidDomainCharacter,
        })?;
        match (url.domain(), url.port()) {
            (Some(domain), None) => {
                if domain == s {
                    return Ok(DomainWithPort::new(domain));
                }
            }
            (Some(domain), Some(port)) => {
                if &format!("{}:{}", domain, port) == s {
                    return Ok(DomainWithPort::new_with_port(domain, port));
                }
            }
            _ => {}
        }
        Err(DomainWithPortParseError::InvalidDomainCharacter)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IpAddrWithPort {
    ip_addr: IpAddr,
    port: Option<NonZeroU16>,
}

impl IpAddrWithPort {
    #[inline]
    pub fn new(ip_addr: IpAddr) -> Self {
        IpAddrWithPort {
            ip_addr,
            port: None,
        }
    }

    #[inline]
    pub fn new_with_port(ip_addr: IpAddr, port: u16) -> Self {
        IpAddrWithPort {
            ip_addr,
            port: NonZeroU16::new(port),
        }
    }

    #[inline]
    pub fn ip_addr(&self) -> IpAddr {
        self.ip_addr
    }

    #[inline]
    pub fn port(&self) -> Option<NonZeroU16> {
        self.port
    }
}

impl From<IpAddr> for IpAddrWithPort {
    #[inline]
    fn from(ip_addr: IpAddr) -> Self {
        Self::new(ip_addr)
    }
}

impl From<IpAddrWithPort> for IpAddr {
    #[inline]
    fn from(ip_addr_with_port: IpAddrWithPort) -> Self {
        ip_addr_with_port.ip_addr()
    }
}

impl From<SocketAddr> for IpAddrWithPort {
    #[inline]
    fn from(socket_addr: SocketAddr) -> Self {
        Self::new_with_port(socket_addr.ip(), socket_addr.port())
    }
}

impl From<IpAddrWithPort> for SocketAddr {
    #[inline]
    fn from(ip_addr_with_port: IpAddrWithPort) -> Self {
        Self::new(
            ip_addr_with_port.ip_addr(),
            ip_addr_with_port.port().map_or(0, |port| port.get()),
        )
    }
}

impl From<(IpAddr, u16)> for IpAddrWithPort {
    #[inline]
    fn from(ip_addr_with_port: (IpAddr, u16)) -> Self {
        Self::new_with_port(ip_addr_with_port.0, ip_addr_with_port.1)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IpAddrWithPortParseError {
    #[error("invalid ip address: {0}")]
    ParseError(#[from] AddrParseError),
}

impl FromStr for IpAddrWithPort {
    type Err = IpAddrWithPortParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<SocketAddr, AddrParseError> = s.parse();
        if let Ok(socket_addr) = parse_result {
            return Ok(socket_addr.into());
        }
        let ip_addr: IpAddr = s.parse()?;
        Ok(ip_addr.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Endpoint {
    DomainWithPort(DomainWithPort),
    IpAddrWithPort(IpAddrWithPort),
}

impl Endpoint {
    #[inline]
    pub fn new_from_domain(domain: impl Into<Box<str>>) -> Self {
        Self::DomainWithPort(DomainWithPort {
            domain: domain.into(),
            port: None,
        })
    }

    #[inline]
    pub fn new_from_domain_with_port(domain: impl Into<Box<str>>, port: u16) -> Self {
        Self::DomainWithPort(DomainWithPort {
            domain: domain.into(),
            port: NonZeroU16::new(port),
        })
    }

    #[inline]
    pub fn new_from_ip_addr(ip_addr: IpAddr) -> Self {
        Self::IpAddrWithPort(IpAddrWithPort {
            ip_addr: ip_addr.into(),
            port: None,
        })
    }

    #[inline]
    pub fn new_from_socket_addr(addr: SocketAddr) -> Self {
        Self::IpAddrWithPort(IpAddrWithPort {
            ip_addr: addr.ip(),
            port: NonZeroU16::new(addr.port()),
        })
    }

    #[inline]
    pub fn domain(&self) -> Option<&str> {
        match self {
            Self::DomainWithPort(domain_with_port) => Some(domain_with_port.domain()),
            _ => None,
        }
    }

    #[inline]
    pub fn ip_addr(&self) -> Option<IpAddr> {
        match self {
            Self::IpAddrWithPort(ip_addr_with_port) => Some(ip_addr_with_port.ip_addr()),
            _ => None,
        }
    }

    #[inline]
    pub fn port(&self) -> Option<NonZeroU16> {
        match self {
            Self::DomainWithPort(domain_with_port) => domain_with_port.port(),
            Self::IpAddrWithPort(ip_addr_with_port) => ip_addr_with_port.port(),
        }
    }
}

impl From<DomainWithPort> for Endpoint {
    #[inline]
    fn from(domain_with_port: DomainWithPort) -> Self {
        Self::DomainWithPort(domain_with_port)
    }
}

impl From<IpAddrWithPort> for Endpoint {
    #[inline]
    fn from(ip_addr_with_port: IpAddrWithPort) -> Self {
        Self::IpAddrWithPort(ip_addr_with_port)
    }
}

impl From<Box<str>> for Endpoint {
    #[inline]
    fn from(domain: Box<str>) -> Self {
        DomainWithPort::new(domain).into()
    }
}

impl From<(Box<str>, u16)> for Endpoint {
    #[inline]
    fn from(domain_with_port: (Box<str>, u16)) -> Self {
        DomainWithPort::new_with_port(domain_with_port.0, domain_with_port.1).into()
    }
}

impl From<String> for Endpoint {
    #[inline]
    fn from(domain: String) -> Self {
        DomainWithPort::new(domain).into()
    }
}

impl From<(String, u16)> for Endpoint {
    #[inline]
    fn from(domain_with_port: (String, u16)) -> Self {
        DomainWithPort::new_with_port(domain_with_port.0, domain_with_port.1).into()
    }
}

impl From<IpAddr> for Endpoint {
    #[inline]
    fn from(ip_addr: IpAddr) -> Self {
        IpAddrWithPort::new(ip_addr).into()
    }
}

impl From<SocketAddr> for Endpoint {
    #[inline]
    fn from(socket_addr: SocketAddr) -> Self {
        IpAddrWithPort::new_with_port(socket_addr.ip(), socket_addr.port()).into()
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EndpointParseError {
    #[error("invalid port number")]
    InvalidPort,
    #[error("empty host")]
    EmptyHost,
    #[error("invalid domain character")]
    InvalidDomainCharacter,
}

impl From<DomainWithPortParseError> for EndpointParseError {
    fn from(err: DomainWithPortParseError) -> Self {
        match err {
            DomainWithPortParseError::InvalidPort => Self::InvalidPort,
            DomainWithPortParseError::EmptyHost => Self::EmptyHost,
            DomainWithPortParseError::InvalidDomainCharacter => Self::InvalidDomainCharacter,
        }
    }
}

impl FromStr for Endpoint {
    type Err = EndpointParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<IpAddrWithPort, IpAddrWithPortParseError> = s.parse();
        if let Ok(ip_addr_with_port) = parse_result {
            return Ok(ip_addr_with_port.into());
        }
        let domain_with_port: DomainWithPort = s.parse()?;
        Ok(domain_with_port.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        error::Error,
        net::{Ipv4Addr, Ipv6Addr},
        result::Result,
    };

    #[test]
    fn test_from_str_to_domain_with_port() -> Result<(), Box<dyn Error>> {
        let mut result: Result<DomainWithPort, DomainWithPortParseError>;
        result = "".parse();
        assert_eq!(result.unwrap_err(), DomainWithPortParseError::EmptyHost);

        result = "/".parse();
        assert_eq!(result.unwrap_err(), DomainWithPortParseError::EmptyHost);

        result = ":".parse();
        assert_eq!(result.unwrap_err(), DomainWithPortParseError::EmptyHost);

        result = ":8080".parse();
        assert_eq!(result.unwrap_err(), DomainWithPortParseError::EmptyHost);

        result = "127.0.0.1:8080".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "[127.0.0.1]:8080".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "8080:8080".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "8080".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "8080:".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "domain:".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "domain:8080".parse();
        assert_eq!(
            result.unwrap(),
            DomainWithPort::new_with_port("domain", 8080)
        );

        result = "domain:8080/".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        result = "domain:65536".parse();
        assert_eq!(result.unwrap_err(), DomainWithPortParseError::InvalidPort);

        result = "七牛云:65535".parse();
        assert_eq!(
            result.unwrap_err(),
            DomainWithPortParseError::InvalidDomainCharacter
        );

        Ok(())
    }

    #[test]
    fn test_from_str_to_ip_addr_with_port() -> Result<(), Box<dyn Error>> {
        let mut result: Result<IpAddrWithPort, IpAddrWithPortParseError>;
        result = "".parse();
        assert!(result.is_err());

        result = "/".parse();
        assert!(result.is_err());

        result = ":".parse();
        assert!(result.is_err());

        result = ":8080".parse();
        assert!(result.is_err());

        result = "127.0.0.1:8080".parse();
        assert_eq!(
            result.unwrap(),
            IpAddrWithPort::new_with_port(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        );

        result = "127.0.0.1:65536".parse();
        assert!(result.is_err());

        result = "fe80::e31c:b4e6:5919:728f".parse();
        assert_eq!(
            result.unwrap(),
            IpAddrWithPort::new(IpAddr::V6(Ipv6Addr::new(
                0xfe80, 0, 0, 0, 0xe31c, 0xb4e6, 0x5919, 0x728f,
            ))),
        );

        result = "[fe80::e31c:b4e6:5919:728f]:8080".parse();
        assert_eq!(
            result.unwrap(),
            IpAddrWithPort::new_with_port(
                IpAddr::V6(Ipv6Addr::new(
                    0xfe80, 0, 0, 0, 0xe31c, 0xb4e6, 0x5919, 0x728f,
                )),
                8080
            ),
        );

        result = "[127.0.0.1]:8080".parse();
        assert!(result.is_err());

        result = "8080:8080".parse();
        assert!(result.is_err());

        result = "8080".parse();
        assert!(result.is_err());

        result = "8080:".parse();
        assert!(result.is_err());

        result = "domain:".parse();
        assert!(result.is_err());

        result = "domain:8080".parse();
        assert!(result.is_err());

        Ok(())
    }
    #[test]
    fn test_from_str_to_endpoint() -> Result<(), Box<dyn Error>> {
        let mut result: Result<Endpoint, EndpointParseError>;

        result = "".parse();
        assert_eq!(result.unwrap_err(), EndpointParseError::EmptyHost);

        result = "/".parse();
        assert_eq!(result.unwrap_err(), EndpointParseError::EmptyHost);

        result = ":".parse();
        assert_eq!(result.unwrap_err(), EndpointParseError::EmptyHost);

        result = ":8080".parse();
        assert_eq!(result.unwrap_err(), EndpointParseError::EmptyHost);

        result = "127.0.0.1:8080".parse();
        assert_eq!(
            result.unwrap(),
            Endpoint::IpAddrWithPort(IpAddrWithPort::new_with_port(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                8080
            ))
        );

        result = "[127.0.0.1]:8080".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "8080:8080".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "8080".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "8080:".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "domain:".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "domain:8080".parse();
        assert_eq!(
            result.unwrap(),
            Endpoint::DomainWithPort(DomainWithPort::new_with_port("domain", 8080))
        );

        result = "domain:8080/".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        result = "domain:65536".parse();
        assert_eq!(result.unwrap_err(), EndpointParseError::InvalidPort);

        result = "七牛云:65535".parse();
        assert_eq!(
            result.unwrap_err(),
            EndpointParseError::InvalidDomainCharacter
        );

        Ok(())
    }
}
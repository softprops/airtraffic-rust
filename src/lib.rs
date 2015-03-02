#![feature(old_io)]
#![feature(old_path)]

use std::old_io::IoError;
use std::old_io::net::pipe::UnixStream;

pub type Result<T> = std::result::Result<T, IoError>;

pub struct Control {
  transport: UnixStream
}

pub enum Statable {
  Frontends = 1,
  Backends  = 2,
  Servers   = 4,
  Any       = -1
}

pub enum Proxy {
  Id(String),
  Any
}

pub enum Server {
  Id(String),
  Any
}

pub enum Fallible {
  Id(String),
  Any
}

pub struct Weight {
  value: String
}

impl Weight {
  pub fn abs(value: u16) -> Weight {
    let normalized = match value {
      over if over > 256 => 256,
      ok => ok
    };
    Weight { value: normalized.to_string() }
  }

  pub fn rel(value: u8) -> Weight {
    let normalized = match value {
      over if over > 100 => 100,
      ok => ok
    };
    Weight { value: format!("{}%", normalized) }
  }
}

impl Control {

  pub fn new(path: Path) -> Control {
    let transport = match UnixStream::connect(&path) {
      Err(e) => panic!("failed to connect to socket: {:?}", e),
      Ok(s)  => s
    };
    Control { transport: transport }
  }

  pub fn info(&mut self) -> Result<String> {
    self.request("show info")
  }

  pub fn sess(&mut self, id: Option<&str>) -> Result<String> {
    self.request(&format!("show sess {}", match id {
      Some(id) => id,
            _  => ""
    }))
  }

  pub fn errors(&mut self, fallible: Fallible) -> Result<String> {
    self.request(&format!("show errors {}", match fallible {
      Fallible::Id(id) => id,
                   _   => "".to_string()
    }))
  }

  pub fn shutdown_session(&mut self, id: &str) -> Result<String> {
    self.request(&format!("shutdown session {}", id))
  }

  pub fn shutdown_sessions(&mut self, backend: String, server: String) -> Result<String> {
    self.request(&format!("shutdown sessions {}/{}", backend, server))
  }

  pub fn stat(&mut self, proxy: Proxy, statable: Statable, server: Server) -> Result<String> {
    self.request(&format!("show stat {} {} {}", match proxy {
      Proxy::Id(id) => id,
                  _ => "-1".to_string()
    }, statable as i8, match server {
      Server::Id(id) => id,
                   _ => "-1".to_string()
    }))
  }

  pub fn map(&mut self, name: &str, key: &str, value: &str) -> Result<String> {
    self.request(&format!("map {} {} {}", name, key, value))
  }

  pub fn map_set(&mut self, name: &str, key: &str, value: &str) -> Result<String> {
    self.request(&format!("map set {} {} {}", name, key, value))
  }

  pub fn map_clear(&mut self, name: &str) -> Result<String> {
    self.request(&format!("clear map {}", name))
  }

  // todo: tables...

  pub fn disable_agent(&mut self, backend: &str, server: &str) -> Result<String> {
    self.request(&format!("disable agent {}/{}", backend, server))
  }

  pub fn disable_frontend(&mut self, name: &str) -> Result<String> {
    self.request(&format!("disable frontend {}", name))
  }

  pub fn shutdown_frontend(&mut self, name: &str) -> Result<String> {
    self.request(&format!("shutdown frontend {}", name))
  }

  pub fn enable_frontend(&mut self, name: &str) -> Result<String> {
    self.request(&format!("enable frontend {}", name))
  }

  pub fn max_frontend_connections(&mut self, name: &str, max: &u32) -> Result<String> {
    self.request(&format!("set maxcon frontend {} {}", name, max))
  }

  pub fn disable_server(&mut self, backend: &str, server: &str) -> Result<String> {
    self.request(&format!("disable server {}/{}", backend, server))
  }

  pub fn enable_agent(&mut self, backend: &str, server: &str) -> Result<String> {
    self.request(&format!("enable agent {}/{}", backend, server))
  }

  pub fn enable_server(&mut self, backend: &str, server: &str) -> Result<String> {
    self.request(&format!("enable server {}/{}", backend, server))
  }

  pub fn weight(&mut self, backend: &str, server: &str) -> Result<String> {
    self.request(&format!("get weight {}/{}", backend, server))
  }

  pub fn max_global_connections(&mut self, max: &u32) -> Result<String> {
    self.request(&format!("set maxconn global {}", max))
  }

  pub fn rate_limit_global_connections(&mut self, max: &u32) -> Result<String> {
    self.request(&format!("set rate-limit connections global {}", max))
  }

  pub fn rate_limit_global_http_compression(&mut self, max: &u32) -> Result<String> {
    self.request(&format!("set rate-limit http-compression global {}", max))
  }

  pub fn rate_limit_global_sessions(&mut self, max: &u32, ssl: bool) -> Result<String> {
    self.request(&format!("set rate-limit {}sessions global {}",
                          match ssl {
                              true => "ssl_",
                              _ => ""
                          }, max))
  }

  /// http://cbonte.github.io/haproxy-dconv/configuration-1.5.html#9.2-set%20weight
  pub fn set_weight(&mut self, backend: &str, server: &str, weight: &Weight) -> Result<String> {
    self.request(&format!("set weight {}/{} {}", backend, server, weight.value))
  }

  fn request(&mut self, cmd: &str) -> Result<String> {
    try!(self.transport.write_line(&format!("{};", cmd)));
    self.transport.read_to_string()
  }
}

#[test]
fn it_caps_abs_over_weights() {
  assert_eq!(Weight::abs(300).value, "256".to_string())
}

#[test]
fn it_caps_abs_under_weights() {
  // negatives overflow
  assert_eq!(Weight::abs(-1).value, "256".to_string())
}

#[test]
fn it_abs_ok_weights() {
  assert_eq!(Weight::abs(10).value, "10".to_string())
}

#[test]
fn it_caps_rel_over_weights() {
  assert_eq!(Weight::rel(101).value, "100%".to_string())
}

#[test]
fn it_caps_rel_under_weights() {
  // negatives overflow
  assert_eq!(Weight::rel(-1).value, "100%".to_string())
}

#[test]
fn it_rels_ok_weights() {
  assert_eq!(Weight::rel(10).value, "10%".to_string())
}

#[test]
fn it_represents_statables() {
  assert_eq!(Statable::Frontends as i8, 1);
  assert_eq!(Statable::Backends as i8, 2);
  assert_eq!(Statable::Servers as i8, 4);
  assert_eq!(Statable::Any as i8, -1);
}

#![feature(std_misc)]

extern crate unix_socket;

use std::io::{ Error, Read, Write };
use std::path::AsPath;
use unix_socket::UnixStream;
use std::collections::HashMap;

// fixme can just be IoError
pub type Result<T> = std::result::Result<T, Error>;

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
  /// Weighting value for proxy targets
  /// indicated as an absoluate value
  /// from 0 to 256
  pub fn abs(value: u16) -> Weight {
    let normalized = match value {
      over if over > 256 => 256,
      ok => ok
    };
    Weight { value: normalized.to_string() }
  }

  /// Weighting value for proxy targets
  /// indicated as relative percent weights
  /// from 0 to 100
  pub fn rel(value: u8) -> Weight {
    let normalized = match value {
      over if over > 100 => 100,
      ok => ok
    };
    Weight { value: format!("{}%", normalized) }
  }
}

pub struct Stats {
  data: HashMap<String, String>
}

impl Stats {
  pub fn new(data: HashMap<String, String>) -> Stats {
    Stats { data: data }
  }

  pub fn get(self, key: &str) -> Option<String> {
    self.data.get(key).map(|v| (*v).clone())
  }

  fn get_str(self, name: &str) -> String {
    self.get(name).unwrap()
  }

  pub fn pxname(self) -> String {
    self.get_str("# pxname")
  }

  pub fn svname(self) -> String {
    self.get_str("svname")
  }

  pub fn qcur(self) -> String {
    self.get_str("qcur")
  }

  pub fn qmax(self) -> String {
    self.get_str("qmax")
  }

  pub fn scur(self) -> String {
    self.get_str("scur")
  }

  pub fn smax(self) -> String {
    self.get_str("max")
  }

  pub fn slim(self) -> String {
    self.get_str("slim")
  }

  pub fn slot(self) -> String {
    self.get_str("slot")
  }

  pub fn bin(self) -> String {
    self.get_str("bin")
  }

  pub fn bout(self) -> String {
    self.get_str("bout")
  }

  pub fn dreq(self) -> String {
    self.get_str("dreq")
  }

  pub fn ereq(self) -> String {
    self.get_str("dreq")
  }

  pub fn econ(self) -> String {
    self.get_str("econ")
  }

  pub fn eresp(self) -> String {
    self.get_str("eresp")
  }

  pub fn wretr(self) -> String {
    self.get_str("wretr")
  }

  pub fn wredis(self) -> String {
    self.get_str("wredis")
  }

  pub fn status(self) -> String {
    self.get_str("status")
  }

  pub fn weight(self) -> String {
    self.get_str("weight")
  }

  pub fn act(self) -> String {
    self.get_str("act")
  }

  pub fn bck(self) -> String {
    self.get_str("bck")
  }

  pub fn chkfail(self) -> String {
    self.get_str("chkfail")
  }

  pub fn chkdown(self) -> String {
    self.get_str("chkdown")
  }

  pub fn lastchg(self) -> String {
    self.get_str("lastchg")
  }

  pub fn downtime(self) -> String {
    self.get_str("downtime")
  }

  pub fn qlimit(self) -> String {
    self.get_str("qlimit")
  }

  pub fn pid(self) -> String {
    self.get_str("pid")
  }

  pub fn iid(self) -> String {
    self.get_str("iid")
  }

  pub fn sid(self) -> String {
    self.get_str("sid")
  }

  pub fn throttle(self) -> String {
    self.get_str("throttle")
  }

  pub fn lbtot(self) -> String {
    self.get_str("lbtot")
  }

  pub fn tracked(self) -> String {
    self.get_str("tracked")
  }

  pub fn typ(self) -> String {
    self.get_str("typ")
  }

  pub fn rate(self) -> String {
    self.get_str("rate")
  }

  pub fn rate_lim(self) -> String {
    self.get_str("rate_lim")
  }

  pub fn rate_max(self) -> String {
    self.get_str("rate_max")
  }

  pub fn check_status(self) -> String {
    self.get_str("check_status")
  }

  pub fn check_duration(self) -> String {
    self.get_str("check_duration")
  }

  pub fn hrsp_1xx(self) -> String {
    self.get_str("hrsp_1xx")
  }

  pub fn hrsp_2xx(self) -> String {
    self.get_str("hrsp_2xx")
  }

  pub fn hrsp_3xx(self) -> String {
    self.get_str("hrsp_3xx")
  }

  pub fn hrsp_4xx(self) -> String {
    self.get_str("hrsp_4xx")
  }

  pub fn hrsp_5xx(self) -> String {
    self.get_str("hrsp_5xx")
  }

  pub fn hrsp_other(self) -> String {
    self.get_str("hrsp_other")
  }

  pub fn hanafail(self) -> String {
    self.get_str("hanafail")
  }

  pub fn req_rate(self) -> String {
    self.get_str("req_rate")
  }

  pub fn req_rate_max(self) -> String {
    self.get_str("req_rate_max")
  }

  pub fn req_tot(self) -> String {
    self.get_str("req_tot")
  }

  pub fn cli_abrt(self) -> String {
    self.get_str("cli_abrt")
  }

  pub fn srv_abrt(self) -> String {
    self.get_str("srv_abrt")
  }
}


pub struct Map<'a, 'b> {
  control: &'a mut Control,
  name: &'b str
}

impl<'a, 'b> Map<'a, 'b> {

  pub fn new(control: &'a mut Control, name: &'b str) -> Map<'a, 'b> {
    Map { control: control, name: name }
  }

  pub fn map(self, key: &str, value: &str) -> Result<String> {
    self.control.request(&format!("map {} {} {}", self.name, key, value))
  }

  pub fn set(self, key: &str, value: &str) -> Result<String> {
    self.control.request(&format!("map set {} {} {}", self.name, key, value))
  }

  pub fn clear(self) -> Result<String> {
    self.control.request(&format!("clear map {}", self.name))
  }
}

pub struct FrontEnd<'a, 'b> {
  control: &'a mut Control,
  name: &'b str
}

impl<'a, 'b> FrontEnd<'a, 'b> {
  pub fn new(control: &'a mut Control, name: &'b str) -> FrontEnd<'a, 'b> {
    FrontEnd { control: control, name: name }
  }

  pub fn disable(self) -> Result<String> {
    self.control.request(&format!("disable frontend {}", self.name))
  }

  pub fn shutdown(self) -> Result<String> {
    self.control.request(&format!("shutdown frontend {}", self.name))
  }

  pub fn enable(self) -> Result<String> {
    self.control.request(&format!("enable frontend {}", self.name))
  }

  pub fn max_connections(self, max: &u32) -> Result<String> {
    self.control.request(&format!("set maxcon frontend {} {}", self.name, max))
  }
}

pub struct BackEnd<'a, 'b, 'c> {
  control: &'a mut Control,
  name: &'b str,
  server: &'c str
}

impl<'a, 'b, 'c> BackEnd<'a, 'b, 'c> {
  pub fn new(control: &'a mut Control, name: &'b str, server: &'c str) -> BackEnd<'a, 'b, 'c> {
    BackEnd { control: control, name: name, server: server }
  }

  pub fn disable_agent(self) -> Result<String> {
    self.control.request(&format!("disable agent {}/{}", self.name, self.server))
  }

  pub fn disable_server(self) -> Result<String> {
    self.control.request(&format!("disable server {}/{}", self.name, self.server))
  }

  pub fn enable_agent(self) -> Result<String> {
    self.control.request(&format!("enable agent {}/{}", self.name, self.server))
  }

  pub fn enable_server(self) -> Result<String> {
    self.control.request(&format!("enable server {}/{}", self.name, self.server))
  }

  pub fn weight(self) -> Result<String> {
    self.control.request(&format!("get weight {}/{}", self.name, self.server))
  }

  /// http://cbonte.github.io/haproxy-dconv/configuration-1.5.html#9.2-set%20weight
  pub fn set_weight(self, weight: &Weight) -> Result<String> {
    self.control.request(&format!("set weight {}/{} {}", self.name, self.server, weight.value))
  }

  pub fn shutdown_sessions(self) -> Result<String> {
    self.control.request(&format!("shutdown sessions {}/{}", self.name, self.server))
  }
}

impl Control {

  /// Creates a new Control given a unix domain socket path
  pub fn new<P: AsPath>(path: P) -> Control {
    let transport = match UnixStream::connect(path) {
      Err(e) => panic!("failed to connect to socket: {:?}", e),
      Ok(s)  => s
    };
    Control { transport: transport }
  }

  pub fn frontend<'a>(&'a mut self, name: &'a str) -> FrontEnd {
    FrontEnd::new(self, name)
  }

  pub fn backend<'a>(&'a mut self, name: &'a str, server: &'a str) -> BackEnd {
    BackEnd::new(self, name, server)
  }

  pub fn map<'a>(&'a mut self, name: &'a str) -> Map {
    Map::new(self, name)
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

  // todo: add structure
  pub fn stat(
    &mut self, proxy: Proxy, statable: Statable, server: Server) -> Result<Vec<Stats>> {
    let s = try!(self.request(
      &format!("show stat {} {} {}", match proxy {
          Proxy::Id(id) => id,
          _ => "-1".to_string()
      }, statable as i8, match server {
          Server::Id(id) => id,
          _ => "-1".to_string()
      })));
    let mut lines = s.lines();
    let names: Vec<String> =
      lines.next().unwrap().split(',')
           .map(|s| s.to_string())
           .collect();
    //println!("names {:?}", names);
    let data: Vec<Stats> = lines.map(|line| {
      let cols = line.split(",");
      let mut map = HashMap::new();
      for (ref name, col) in names.iter().zip(cols) {
        map.insert(name.to_string(), col.to_string());
      }
      Stats::new(map)
    }).collect();
    Ok(data)
  }

  // todo: tables...


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

  fn request(&mut self, cmd: &str) -> Result<String> {
    try!(self.transport.write_all(format!("{};", cmd).as_bytes()));
    let mut result = String::new();
    self.transport.read_to_string(&mut result).map(|_| result)
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

extern crate airtraffic;

use airtraffic::{ Control, Proxy, Server, Statable };

#[cfg(not(test))]
fn main() {
  let mut atc = Control::new(
      "/Users/dougtangren/code/rust/airtraffic/haproxy.stat");
  println!("atc");
  match atc.stat(Proxy::Any, Statable::Any, Server::Any) {
    Ok(stats) => {
        for s in stats {
           println!("{:?}", s.pxname())
        }
    },         
    Err(e) => panic!("whoops {}", e)
  }
}

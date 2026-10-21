use world_of_individuals::{preparation as m, experiment013};
fn main() {
 let args:Vec<_>=std::env::args().collect();
 if args[1]=="validate-snapshot" {
  let snapshot:experiment013::Snapshot=serde_json::from_slice(&std::fs::read(&args[2]).unwrap()).unwrap();
  match snapshot.validate() {Ok(())=>println!("Unchanged full typed Snapshot validator PASS on observer-only namespace translation"),Err(e)=>panic!("Full translated snapshot validation: {e}")};return;
 }
 let c:m::Config=serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap();let _=m::trial(c);panic!("diagnostic expected original validation failure");
}

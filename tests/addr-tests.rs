extern crate aximate;

const RON: &str = "ronin:0000000000000000000000000000000000000000";
const ETH: &str = "0x0000000000000000000000000000000000000000";

#[test]
fn ron_to_eth() {
  assert_eq!(aximate::addr::ron_to_eth(RON), ETH)
}

#[test]
fn eth_to_ron() {
  assert_eq!(aximate::addr::eth_to_ron(ETH), RON)
}

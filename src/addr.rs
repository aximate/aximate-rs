pub fn ron_to_eth(ron: &str) -> String {
  return ron.replace("ronin:", "0x");
}

pub fn eth_to_ron(eth: &str) -> String {
  return eth.replace("0x", "ronin:");
}

extern crate aximate;

const RONIN: &str = "ronin:0000000000000000000000000000000000000000";

const ACCOUNT_JSON: &str = r#"
{
  "discord_id": "12345",
  "name": "test",
  "ronin": "ronin:0000000000000000000000000000000000000000"
}
"#;

const ACCOUNT_JSON_COMPACT: &str = r#"{"discord_id":"12345","name":"test","ronin":"ronin:0000000000000000000000000000000000000000"}"#;

fn gen_account() -> aximate::Account {
  return aximate::Account {
    discord_id: "12345".to_string(),
    name: "test".to_string(),
    ronin: RONIN.to_string(),
  };
}

#[test]
fn struct_definition() {
  let account = aximate::Account {
    discord_id: "12345".to_string(),
    name: "test".to_string(),
    ronin: RONIN.to_string(),
  };

  assert_eq!(account.discord_id, "12345".to_string());
  assert_eq!(account.name, "test".to_string());
  assert_eq!(account.ronin, RONIN.to_string());
}

#[test]
fn json_decode() {
  let account: aximate::Account = serde_json::from_str(ACCOUNT_JSON).unwrap();

  assert_eq!(account.discord_id, "12345".to_string());
  assert_eq!(account.name, "test".to_string());
  assert_eq!(account.ronin, RONIN.to_string());
}

#[test]
fn json_encode() {
  let encoded: &str = &serde_json::to_string(&gen_account()).unwrap();
  assert_eq!(encoded, ACCOUNT_JSON_COMPACT);
}

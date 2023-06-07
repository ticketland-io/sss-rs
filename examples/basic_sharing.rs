use std::str::from_utf8;
use blake3::hash;
use sss_rs::basic_sharing::{from_secrets_no_points, reconstruct_secrets_no_points};

fn basic_sharing() {
  println!("Basic Sharing");

  let secret = b"this is a secret seed";

  // The second coefficient is predictable and is  constructed from secret answers to user selected questions
  let first_pet = "Arnold";
  let favourite_animal = "crab";
  let coefficients = vec![
    hash(format!("{first_pet}{favourite_animal}").as_bytes()).as_bytes().to_vec()
  ];
  let shares_to_create = 3;
  let shares_required = 2;

  let shares = from_secrets_no_points(
    secret,
    shares_required,
    shares_to_create,
    Some(coefficients),
    None,
  ).unwrap();

  let hex_shares = shares.iter().map(|s| hex::encode(&s)).collect::<Vec<_>>();
  println!("Shares {:?}", hex_shares);

  // Recover secret
  let secret = reconstruct_secrets_no_points(shares).unwrap();
  println!("Restored Secret {:?}", from_utf8(&secret));
}

fn basic_sharing_with_shares() {
  println!("Basic Sharing with shares");

  // Shares in hex format
  let shares = vec![
    "01bf9648af43fedf293d6ff33345902534a839080327",
    "02ff892bd6e65a3632d9be6ec92fabe5f42de7bfa9e2",
    "0334770a0a85cd9a3b85f1ee9f0949a5b4a5add2cfa1"
  ];

  let shares = shares.iter().map(|s| hex::decode(s).unwrap()).collect::<Vec<_>>();

  let secret = reconstruct_secrets_no_points(shares).unwrap();
  println!("Restored Secret {:?}", from_utf8(&secret));
}

fn main() {
  basic_sharing();
  basic_sharing_with_shares();
}

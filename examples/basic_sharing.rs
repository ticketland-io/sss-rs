use blake3::hash;
use sss_rs::{
  derived_share::{with_derived_share, derived_share_hex},
  basic_sharing::{reconstruct_secrets_no_points,},
};


fn basic_sharing() {
  println!("Basic Sharing");

  let secret = hash(b"this is a secret seed");

  // The second coefficient is predictable and is  constructed from secret answers to user selected questions
  let first_pet = "Arnold";
  let favourite_animal = "crab";

  let shares = with_derived_share(
    secret.as_bytes(),
    vec![favourite_animal, first_pet],
    2,
    3,
    None
  );
  println!("Shares {:?}", shares);
}

fn basic_sharing_with_shares() {
  println!("Basic Sharing with shares");

  // User answers the following question to derive the share f(3)
  let first_pet = "Arnold";
  let favourite_animal = "crab";
  let derived_share = derived_share_hex(vec![first_pet, favourite_animal]);
  // prepend the share with number 3 since it represents the thirs share
  let derived_share = format!("03{derived_share}");

  // Shares in hex format
  let shares = vec![
    // This share will is stored on the device
    "015a2d54d7e40eadf7d82635a2fb120ffa7a0b9bf9e07e2943f87346034e5ae1ba",
    &derived_share,
  ];

  let shares = shares.iter().map(|s| hex::decode(s).unwrap()).collect::<Vec<_>>();
  let secret = reconstruct_secrets_no_points(shares).unwrap();

  println!("Restored Secret {:?}", hex::encode(&secret));
}

fn main() {
  basic_sharing();
  basic_sharing_with_shares();
}

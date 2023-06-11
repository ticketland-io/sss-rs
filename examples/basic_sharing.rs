use blake3::hash;
use sss_rs::{
  geometry::Coeff,
  basic_sharing::{
    from_secrets_no_points, reconstruct_secrets_no_points,
  },
};

/// This will calculate `a1 = (f3 - a0) / 3`
fn calculate_derived_coeff(secret: &[u8], derived_share: &[u8]) -> Vec<u8> {
  let mut coeff = vec![];
  
  for (i, s) in secret.iter().enumerate() {
    let f3 = Coeff(derived_share[i]);
    let a0 = Coeff(*s);
    // operations are done in the Galois finite field
    let a1 = (f3 - a0) / Coeff(3);

    coeff.push(a1.0);
  }

  coeff
}

fn basic_sharing() {
  println!("Basic Sharing");

  let secret = hash(b"this is a secret seed");
  // The second coefficient is predictable and is  constructed from secret answers to user selected questions
  let first_pet = "Arnold";
  let favourite_animal = "crab";
  // THe third share (f(3)) which is derived form user answers
  let derived_share = hash(format!("{first_pet}{favourite_animal}").as_bytes());
  let coeff = calculate_derived_coeff(secret.as_bytes(), derived_share.as_bytes());
  
  println!("secret {:?}", secret);
  println!("Derived share: {}", derived_share);

  let coefficients = vec![coeff];
  let shares_to_create = 3;
  let shares_required = 2;

  let shares = from_secrets_no_points(
    secret.as_bytes(),
    shares_required,
    shares_to_create,
    Some(coefficients),
    None,
  ).unwrap();

  let hex_shares = shares.iter().map(|s| hex::encode(&s)).collect::<Vec<_>>();
  println!("Shares {:?}", hex_shares);
}

fn basic_sharing_with_shares() {
  println!("Basic Sharing with shares");

  // User answers the following question to derive the share f(3)
  let first_pet = "Arnold";
  let favourite_animal = "crab";
  let derived_share = hash(format!("{first_pet}{favourite_animal}").as_bytes());
  // pre-end the share with number 3 since it represents the thirs share
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

use blake3::hash;
use sss_rs::{
  geometry::Coeff,
  basic_sharing::{
    from_secrets_no_points, reconstruct_secrets_no_points,
  },
};

/// This will calculate `a1 = (f3 - a0) / 3`
/// The way it works is fairly simple. In a 2 of N scheme we SSS constructs a polynomial of degree 2 - 1.
/// A polynomial of 1 degree is essentially a line i.e. `f(x) = a0 + a1*x`.
/// The trick to create a derived share is as follows:
/// 
/// 1. let the derived share be the share 3 i.e. f(3) = 100
/// 2. given f(3) find the coefficient a1 from the above polynomial. If we solve the polynomial for a1 and x=3
///    we get `a1 = (f3 - a0) / 3`
/// 3. Now we have both the static coefficients a0, which is the secret, as well as, the second coefficient a1.
/// 
/// Now we can restore the secret by using 2 shares. One share can be stored on a device of in an encrypted form,
/// on some decentralized storage. The second share can be a hash of answers to questions users know. With 2 shares
/// user can restore the secret.
fn calculate_derived_coeff(secret: &[u8], derived_share: &[u8]) -> Vec<u8> {
  let mut coeff = vec![];
  
  // Note! operations must take place in the Galois finite field
  for (i, s) in secret.iter().enumerate() {
    let f3 = Coeff(derived_share[i]);
    let a0 = Coeff(*s);
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
  // Note if secret is smaller then 32 bytes the we need to truncate `derived_share` to match it's length;
  let derived_share = hash(format!("{first_pet}{favourite_animal}").as_bytes());
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

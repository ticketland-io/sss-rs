use rand::RngCore;
use blake3::hash;
use eyre::Result;
use crate::{geometry::Coeff, basic_sharing::from_secrets_no_points};

/// This will calculate `a1 = (f3 - a0) / 3`
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

pub fn derived_share_hex(seeds: Vec<&str>) -> String {
  hex::encode(derived_share(seeds))
}

pub fn derived_share(mut seeds: Vec<&str>) -> Vec<u8> {
  // make sure the seeds are sorted
  seeds.sort();

  let seed = seeds.into_iter()
  .fold("".to_string(), |acc, elem| format!("{}{}", acc, elem.to_lowercase()));

  hash(seed.as_bytes()).as_bytes().to_vec()
} 

/// Creates SSS shares.
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
pub fn with_derived_share(
  secret: &[u8],
  seeds: Vec<&str>,
  shares_required: u8,
  shares_to_create: u8,
  rand: Option<&mut dyn RngCore>,
) -> Result<Vec<String>> {
  
  // The third share (f(3)) which is derived form the hash of the given seeds
  let derived_share = derived_share(seeds);
  let coeff = calculate_derived_coeff(secret, &derived_share);

  let shares = from_secrets_no_points(
    secret,
    shares_required,
    shares_to_create,
    Some(vec![coeff]),
    rand,
  )?;
  let hex_shares = shares.iter().map(|s| hex::encode(&s)).collect::<Vec<_>>();

  Ok(hex_shares)
}

use bcrypt::{hash, verify, DEFAULT_COST, BcryptResult};

pub fn encrypt_password(password: &str) -> BcryptResult<String> {
  hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hashed_password: &str) -> BcryptResult<bool> {
  verify(password, hashed_password)
}
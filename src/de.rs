use crate::config::Config;
use base64::{decode, encode};
use rand::prelude::ThreadRng;
use rand::rngs::OsRng;
use rsa::{
    pkcs1::FromRsaPrivateKey, pkcs1::FromRsaPublicKey, pkcs1::ToRsaPrivateKey,
    pkcs1::ToRsaPublicKey, RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::result::Result;

#[derive(Clone, Debug)]
pub struct De {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl De {
    pub fn new(private_key: RsaPrivateKey, public_key: RsaPublicKey) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    pub fn public_key_string(&self) -> String {
        self.public_key.to_pkcs1_pem().unwrap()
    }

    pub fn sign(&self, data: &str) -> String {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(data);
        let digest = hasher.finalize();
        let signature: Vec<u8> =
            rsa_fdh::sign::<Sha256, _>(&mut rng, &self.private_key, &digest).unwrap();

        encode(signature)
    }
}

pub fn check_sign(data: &str, public_key_string: &str, signature_string: &str) -> bool {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let public_key: RsaPublicKey =
        RsaPublicKey::from_pkcs1_pem(public_key_string).expect("Failed to read key");
    let signature: Vec<u8> = decode(signature_string).unwrap();
    let verification = rsa_fdh::verify::<Sha256, _>(&public_key, &digest, &signature);

    match verification {
        Result::Ok(()) => true,
        Result::Err(_) => false,
    }
}

pub fn get(config: &Config) -> De {
    let private_key_path: String = format!("{dir}{file}", dir = config.etc_dir(), file = "key");
    let public_key_path: String = format!("{dir}{file}", dir = config.etc_dir(), file = "key.pub");
    let private_key: RsaPrivateKey = get_private_key(&private_key_path);
    let public_key: RsaPublicKey = get_public_key(&private_key, &public_key_path);
    De::new(private_key, public_key)
}

fn get_private_key(private_key_path: &str) -> RsaPrivateKey {
    if !Path::new(private_key_path).exists() {
        let mut rng: OsRng = OsRng;
        let bits: usize = 1024;
        let private_key: RsaPrivateKey =
            RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let mut private_key_file: File =
            File::create(&private_key_path).expect("Failed to create a file");
        private_key_file
            .write_all(private_key.to_pkcs1_pem().unwrap().as_bytes())
            .expect("Failed to write key");
        private_key
    } else {
        let mut private_key_file: File =
            File::open(&private_key_path).expect("Failed to open a file");
        let mut private_key_contents: String = String::new();
        private_key_file
            .read_to_string(&mut private_key_contents)
            .expect("Failed to read key file");
        let private_key: RsaPrivateKey =
            RsaPrivateKey::from_pkcs1_pem(&private_key_contents).expect("Failed to read key");
        private_key
    }
}

fn get_public_key(private_key: &RsaPrivateKey, public_key_path: &str) -> RsaPublicKey {
    if !Path::new(public_key_path).exists() {
        let public_key: RsaPublicKey = RsaPublicKey::from(private_key);
        let mut public_key_file: File =
            File::create(&public_key_path).expect("Failed to create a file");
        public_key_file
            .write_all(public_key.to_pkcs1_pem().unwrap().as_bytes())
            .expect("Failed to write key");
        public_key
    } else {
        let mut public_key_file: File =
            File::open(&public_key_path).expect("Failed to open a file");
        let mut public_key_string: String = String::new();
        public_key_file
            .read_to_string(&mut public_key_string)
            .expect("Failed to read key file");
        let public_key: RsaPublicKey =
            RsaPublicKey::from_pkcs1_pem(&public_key_string).expect("Failed to read key");
        public_key
    }
}

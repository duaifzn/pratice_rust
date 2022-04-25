use openssl::{
    pkey::{
        PKey,
        Private
    },
    rsa::Rsa, x509::{X509Name, X509}, nid::Nid, hash::MessageDigest, asn1::Asn1Time
};
use core::str;
use std::{fs::File, io::Write};
pub struct OpensslGenerator{
    keypair: PKey<Private>
}

impl OpensslGenerator{
    pub fn new_keypair() ->Self{
        let rsa = Rsa::generate(2048).unwrap();
        Self{
            keypair: PKey::from_rsa(rsa).unwrap()
        }
    }
    pub fn generate_private_key_file(&self){
        let private_key = self.keypair.private_key_to_pem_pkcs8().unwrap();
        let mut file = File::create("key.pem").unwrap();
        file.write_all(&private_key);
    }
    pub fn generate_public_key_file(&self){
        let public_key = self.keypair.public_key_to_pem().unwrap();
        let mut file = File::create("key.pub.pem").unwrap();
        file.write_all(&public_key);
    }
    pub fn generate_csr_file(&self){
        let mut name = X509Name::builder().unwrap();
        name.append_entry_by_nid(Nid::COMMONNAME, "foobar.com").unwrap();
        let name = name.build();

        let mut builder = X509::builder().unwrap();
        builder.set_version(2).unwrap();
        builder.set_subject_name(&name).unwrap();
        builder
        .set_not_before(&Asn1Time::days_from_now(0).unwrap()).unwrap();
        builder
        .set_not_after(&Asn1Time::days_from_now(365).unwrap()).unwrap();
        builder.set_issuer_name(&name).unwrap();
        builder.set_pubkey(&self.keypair).unwrap();
        builder.sign(&self.keypair, MessageDigest::sha256()).unwrap();

        let certificate: X509 = builder.build();
        let crt_key = certificate.to_pem().unwrap();
        // println!("{:?}", certificate);
        // println!("{}", std::str::from_utf8(&csr_key).unwrap());
        let mut file = File::create("key.crt").unwrap();
        file.write_all(&crt_key);
    }
}


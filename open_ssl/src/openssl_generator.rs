use openssl::{
    pkey::{
        PKey,
        Private
    },
    rsa::{Rsa, Padding}, x509::{X509Name, X509}, nid::Nid, hash::MessageDigest, asn1::Asn1Time,
};
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
    pub fn generate_crt_file(&self){
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
    pub fn crt_to_public_key(&self){
        let key = include_bytes!("../key.crt");
        let crt = X509::from_pem(key).unwrap();
        println!("{:?}", std::str::from_utf8(&crt.public_key().unwrap().public_key_to_pem().unwrap()).unwrap());
    }
    pub fn priv_encryp_pub_decryp(){
        let pkey = include_bytes!("../key.pem");
        let private_key = Rsa::private_key_from_pem(pkey).unwrap();
        let mut result = vec![0; private_key.size() as usize];
        println!("{:?}", result);
        let original_data = b"This is test";
        let len = private_key
            .private_encrypt(original_data, &mut result, Padding::PKCS1)
            .unwrap();
        assert_eq!(len, 256);
        println!("{:?}", std::str::from_utf8(&result));
        let key = include_bytes!("../key.pub.pem");
        let public_key = Rsa::public_key_from_pem(key).unwrap();
        let mut dec_result = vec![0; public_key.size() as usize];
        let len = private_key
            .public_decrypt(&result, &mut dec_result, Padding::PKCS1)
            .unwrap();

        println!("{:?}", std::str::from_utf8(&dec_result[..len]));
        assert_eq!(&dec_result[..len], original_data);
    }
    pub fn pub_encryp_priv_decryp(){
        //same as openssl::encrypt::{Encrypter, Decrypter}
        let key = include_bytes!("../key.pub.pem");
        let public_key = Rsa::public_key_from_pem(key).unwrap();
        let mut result = vec![0; public_key.size() as usize];
        let original_data = b"This is test";
        let len = public_key
            .public_encrypt(original_data, &mut result, Padding::PKCS1)
            .unwrap();
        assert_eq!(len, 256);

        let pkey = include_bytes!("../key.pem");
        let private_key = Rsa::private_key_from_pem(pkey).unwrap();
        let mut dec_result = vec![0; private_key.size() as usize];
        let len = private_key
            .private_decrypt(&result, &mut dec_result, Padding::PKCS1)
            .unwrap();

        println!("{:?}", std::str::from_utf8(&dec_result[..len]));
        assert_eq!(&dec_result[..len], original_data);
    }
    //rsa digital signature
    //use openssl::sign::{Signer, Verifier};
}


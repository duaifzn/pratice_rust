use openssl_generator::OpensslGenerator;

mod openssl_generator;

fn main(){
    //let openssl_gen = openssl_generator::OpensslGenerator::new_keypair();
    // openssl_gen.generate_crt_file();
    OpensslGenerator::priv_encryp_pub_decryp();
}
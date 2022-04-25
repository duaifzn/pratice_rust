mod openssl_generator;

fn main(){
    let openssl_gen = openssl_generator::OpensslGenerator::new_keypair();
    openssl_gen.generate_csr_file();

}
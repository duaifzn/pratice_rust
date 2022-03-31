pub mod config_local;
pub mod config_dev;
pub mod config_prod;
use crate::config::config_local::local;
use crate::config::config_dev::dev;
use crate::config::config_prod::prod;
use std::env;

pub struct Config{
    pub jwt_secret: String,
    pub jwt_expire: i64,
}

impl Config{
    pub fn load() ->Self{
        let args: Vec<String> = env::args().collect();
        match args.len(){
            0 | 1 => return local(),
            _ => {}
        }
        let env = args[1].clone();
        match env.as_ref(){
            "local" => return local(),
            "dev" => return dev(),
            "prod" => return prod(),
            _ => return local()
        }
    }
}
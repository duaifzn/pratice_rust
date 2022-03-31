use crate::config::Config;

pub fn prod() ->Config{
    Config { 
        jwt_secret: "secret".to_string(),
        jwt_expire: 24*60*60 
    }
}
use crate::config::Config;

pub fn local() ->Config{
    Config { 
        jwt_secret: "secret".to_string(),
        jwt_expire: 24*60*60 
    }
}
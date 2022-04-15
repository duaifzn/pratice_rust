use std::str::FromStr;
use web3::types::{H160, Address, U256, BlockId};
use web3::{self, Web3, Result};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use hex_literal::hex;

pub struct EthNode{
    pub node_url: String,
    pub web3: Web3<Http>
}

impl EthNode{
    fn connect() ->Self{
        let transport = Http::new("http://localhost:8545").unwrap();
        let web3 = Web3::new(transport);
        
        Self{
            node_url: "http://localhost:8545".to_string(),
            web3: web3
        }
    }
    async fn get_accounts(&self) ->Result<Vec<H160>>{
        let accounts = self.web3.eth().accounts().await?;
        Ok(accounts)
    }
    async fn get_account_balance(&self, account:Address) ->Result<U256>{
        let balance = self.web3.eth().balance(account, None).await?;
        Ok(balance)
    }
    async fn connect_contract_proof_of_existence(&self, address: &str) ->Contract<Http>{
        let contract_address = Address::from_str(address).unwrap();
        let contract = Contract::from_json(
            self.web3.eth(),
            contract_address,
            include_bytes!("../contract/aaa.json")
        ).unwrap();
        contract
    }
}
#[tokio::main]
async fn main() -> web3::Result<()> {
    let eth_node = EthNode::connect();
    let mut accounts = eth_node.get_accounts().await?;
    println!("Accounts: {:?}", accounts);
    let a = eth_node.get_account_balance(accounts[0]).await?;
    println!("{:?}", a);
    let contract = eth_node.connect_contract_proof_of_existence("eA4c45d4e0f2517016774f7ce45B03606B753aE8").await;
    println!("{:?}\n\n", contract);
    let bb = contract.query::<String, _, _, _>(
        "isIssuer",
        accounts[1],
        accounts[1],
        Options::default(),
        None).await;
    println!("{:?}", bb);
    //0x9ac114c906a4c24daa334c9fbfb6f5ac0cba053e14a94b6e283b8e378ce85242
    Ok(())
}


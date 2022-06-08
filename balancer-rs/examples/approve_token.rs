extern crate balancer_rs;

mod helpers;
mod sample_data;

use balancer_rs::helpers;
use balancer_rs::helpers::get_env_var;
use balancer_rs::*;

#[tokio::main]
async fn main() {
    print_start_new_example("Vault#setApproval");

    let rpc_url: String = get_env_var("RPC_URL");
    let transport = ethcontract::web3::transports::Http::new(&rpc_url).unwrap();
    let web3 = ethcontract::Web3::new(transport);

    let token = addr!(sample_data::kovan::USDC_ADDRESS);
    let private_key = PrivateKey::from_str(&get_env_var("PRIVATE_KEY")).unwrap();
    let token_approver = TokenApprover::new(web3, private_key);

    token_approver.approve(token, u256!("1000000000000000000"))
    .await {
        Ok(any) => any,
        Err(e) => {
            println!("Failed to approve token: {}", token);
            println!("{:#?}", e);
            return;
        }
    };
}

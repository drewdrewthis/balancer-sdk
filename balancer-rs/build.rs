use ethcontract_generate::loaders::TruffleLoader;
use ethcontract_generate::ContractBuilder;

fn main() {
  println!("Building contracts");

  // Prepare filesystem paths.
  let dest = std::path::Path::new("./src/").join("generated_test_contract.rs");

  // Load a contract.
  let contract = TruffleLoader::new()
    .load_contract_from_file("./src/abis/SimpleTestContract.json")
    .unwrap();

  // Generate bindings for it.
  ContractBuilder::new()
    .generate(&contract)
    .unwrap()
    .write_to_file(dest)
    .unwrap();
}

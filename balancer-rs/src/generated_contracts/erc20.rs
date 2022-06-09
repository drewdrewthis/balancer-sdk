# [allow (dead_code)] pub mod erc20 { # [rustfmt :: skip] use ethcontract as ethcontract ; # [doc = "Generated by `ethcontract`"] # [derive (Clone)] pub struct Contract { methods : Methods , } impl Contract { # [doc = r" Retrieves the raw contract instance used to generate the type safe"] # [doc = r" API for this contract."] pub fn raw_contract () -> & 'static self :: ethcontract :: Contract { use self :: ethcontract :: common :: artifact :: truffle :: TruffleLoader ; use self :: ethcontract :: private :: lazy_static ; use self :: ethcontract :: Contract ; lazy_static ! { pub static ref CONTRACT : Contract = { # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"ERC20\",\"abi\":[{\"type\":\"function\",\"name\":\"allowance\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\"},{\"name\":\"_spender\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"approve\",\"inputs\":[{\"name\":\"_spender\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"balanceOf\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"balance\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"decimals\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"name\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"symbol\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"totalSupply\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transfer\",\"inputs\":[{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferFrom\",\"inputs\":[{\"name\":\"_from\",\"type\":\"address\"},{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Approval\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"indexed\":true},{\"name\":\"spender\",\"type\":\"address\",\"indexed\":true},{\"name\":\"value\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Transfer\",\"inputs\":[{\"name\":\"from\",\"type\":\"address\",\"indexed\":true},{\"name\":\"to\",\"type\":\"address\",\"indexed\":true},{\"name\":\"value\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"fallback\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ; contract } ; } & CONTRACT } # [doc = r" Creates a new contract instance with the specified `web3`"] # [doc = r" provider at the given `Address`."] # [doc = r""] # [doc = r" Note that this does not verify that a contract with a matching"] # [doc = r" `Abi` is actually deployed at the given address."] pub fn at < F , B , T > (web3 : & self :: ethcontract :: web3 :: api :: Web3 < T > , address : self :: ethcontract :: Address ,) -> Self where F : std :: future :: Future < Output = Result < self :: ethcontract :: json :: Value , self :: ethcontract :: web3 :: Error , > , > + Send + 'static , B : std :: future :: Future < Output = Result < Vec < Result < self :: ethcontract :: json :: Value , self :: ethcontract :: web3 :: Error , > , > , self :: ethcontract :: web3 :: Error , > , > + Send + 'static , T : self :: ethcontract :: web3 :: Transport < Out = F > + self :: ethcontract :: web3 :: BatchTransport < Batch = B > + Send + Sync + 'static , { Contract :: with_deployment_info (web3 , address , None) } # [doc = r" Creates a new contract instance with the specified `web3` provider with"] # [doc = r" the given `Abi` at the given `Address` and an optional transaction hash."] # [doc = r" This hash is used to retrieve contract related information such as the"] # [doc = r" creation block (which is useful for fetching all historic events)."] # [doc = r""] # [doc = r" Note that this does not verify that a contract with a matching `Abi` is"] # [doc = r" actually deployed at the given address nor that the transaction hash,"] # [doc = r" when provided, is actually for this contract deployment."] pub fn with_deployment_info < F , B , T > (web3 : & self :: ethcontract :: web3 :: api :: Web3 < T > , address : self :: ethcontract :: Address , deployment_information : Option < ethcontract :: common :: DeploymentInformation > ,) -> Self where F : std :: future :: Future < Output = Result < self :: ethcontract :: json :: Value , self :: ethcontract :: web3 :: Error , > , > + Send + 'static , B : std :: future :: Future < Output = Result < Vec < Result < self :: ethcontract :: json :: Value , self :: ethcontract :: web3 :: Error , > , > , self :: ethcontract :: web3 :: Error , > , > + Send + 'static , T : self :: ethcontract :: web3 :: Transport < Out = F > + self :: ethcontract :: web3 :: BatchTransport < Batch = B > + Send + Sync + 'static , { use self :: ethcontract :: Instance ; use self :: ethcontract :: transport :: DynTransport ; use self :: ethcontract :: web3 :: api :: Web3 ; let transport = DynTransport :: new (web3 . transport () . clone ()) ; let web3 = Web3 :: new (transport) ; let abi = Self :: raw_contract () . abi . clone () ; let instance = Instance :: with_deployment_info (web3 , abi , address , deployment_information) ; Contract :: from_raw (instance) } # [doc = r" Creates a contract from a raw instance."] fn from_raw (instance : self :: ethcontract :: dyns :: DynInstance) -> Self { let methods = Methods { instance } ; Contract { methods } } # [doc = r" Returns the contract address being used by this instance."] pub fn address (& self) -> self :: ethcontract :: Address { self . raw_instance () . address () } # [doc = r" Returns the deployment information of the contract"] # [doc = r" if it is known, `None` otherwise."] pub fn deployment_information (& self) -> Option < ethcontract :: common :: DeploymentInformation > { self . raw_instance () . deployment_information () } # [doc = r" Returns a reference to the default method options used by this"] # [doc = r" contract."] pub fn defaults (& self) -> & self :: ethcontract :: contract :: MethodDefaults { & self . raw_instance () . defaults } # [doc = r" Returns a mutable reference to the default method options used"] # [doc = r" by this contract."] pub fn defaults_mut (& mut self) -> & mut self :: ethcontract :: contract :: MethodDefaults { & mut self . raw_instance_mut () . defaults } # [doc = r" Returns a reference to the raw runtime instance used by this"] # [doc = r" contract."] pub fn raw_instance (& self) -> & self :: ethcontract :: dyns :: DynInstance { & self . methods . instance } # [doc = r" Returns a mutable reference to the raw runtime instance used by"] # [doc = r" this contract."] fn raw_instance_mut (& mut self) -> & mut self :: ethcontract :: dyns :: DynInstance { & mut self . methods . instance } } impl std :: fmt :: Debug for Contract { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple (stringify ! (ERC20)) . field (& self . address ()) . finish () } } impl Contract { # [doc = r" Returns an object that allows accessing typed method signatures."] pub fn signatures () -> Signatures { Signatures } # [doc = r" Retrieves a reference to type containing all the generated"] # [doc = r" contract methods. This can be used for methods where the name"] # [doc = r" would collide with a common method (like `at` or `deployed`)."] pub fn methods (& self) -> & Methods { & self . methods } } # [doc = r" Type containing signatures for all methods for generated contract type."] # [derive (Clone , Copy)] pub struct Signatures ; impl Signatures { # [doc = "Returns signature for method `allowance(address,address):(uint256)`."] # [allow (clippy :: type_complexity)] pub fn allowance (& self) -> self :: ethcontract :: contract :: Signature < (self :: ethcontract :: Address , self :: ethcontract :: Address ,) , self :: ethcontract :: U256 > { self :: ethcontract :: contract :: Signature :: new ([221 , 98 , 237 , 62]) } # [doc = "Returns signature for method `approve(address,uint256):(bool)`."] # [allow (clippy :: type_complexity)] pub fn approve (& self) -> self :: ethcontract :: contract :: Signature < (self :: ethcontract :: Address , self :: ethcontract :: U256 ,) , bool > { self :: ethcontract :: contract :: Signature :: new ([9 , 94 , 167 , 179]) } # [doc = "Returns signature for method `balanceOf(address):(uint256)`."] # [allow (clippy :: type_complexity)] pub fn balance_of (& self) -> self :: ethcontract :: contract :: Signature < (self :: ethcontract :: Address ,) , self :: ethcontract :: U256 > { self :: ethcontract :: contract :: Signature :: new ([112 , 160 , 130 , 49]) } # [doc = "Returns signature for method `decimals():(uint8)`."] # [allow (clippy :: type_complexity)] pub fn decimals (& self) -> self :: ethcontract :: contract :: Signature < () , u8 > { self :: ethcontract :: contract :: Signature :: new ([49 , 60 , 229 , 103]) } # [doc = "Returns signature for method `name():(string)`."] # [allow (clippy :: type_complexity)] pub fn name (& self) -> self :: ethcontract :: contract :: Signature < () , String > { self :: ethcontract :: contract :: Signature :: new ([6 , 253 , 222 , 3]) } # [doc = "Returns signature for method `symbol():(string)`."] # [allow (clippy :: type_complexity)] pub fn symbol (& self) -> self :: ethcontract :: contract :: Signature < () , String > { self :: ethcontract :: contract :: Signature :: new ([149 , 216 , 155 , 65]) } # [doc = "Returns signature for method `totalSupply():(uint256)`."] # [allow (clippy :: type_complexity)] pub fn total_supply (& self) -> self :: ethcontract :: contract :: Signature < () , self :: ethcontract :: U256 > { self :: ethcontract :: contract :: Signature :: new ([24 , 22 , 13 , 221]) } # [doc = "Returns signature for method `transfer(address,uint256):(bool)`."] # [allow (clippy :: type_complexity)] pub fn transfer (& self) -> self :: ethcontract :: contract :: Signature < (self :: ethcontract :: Address , self :: ethcontract :: U256 ,) , bool > { self :: ethcontract :: contract :: Signature :: new ([169 , 5 , 156 , 187]) } # [doc = "Returns signature for method `transferFrom(address,address,uint256):(bool)`."] # [allow (clippy :: type_complexity)] pub fn transfer_from (& self) -> self :: ethcontract :: contract :: Signature < (self :: ethcontract :: Address , self :: ethcontract :: Address , self :: ethcontract :: U256 ,) , bool > { self :: ethcontract :: contract :: Signature :: new ([35 , 184 , 114 , 221]) } } # [doc = r" Type containing all contract methods for generated contract type."] # [derive (Clone)] pub struct Methods { instance : self :: ethcontract :: dyns :: DynInstance , } # [allow (clippy :: too_many_arguments , clippy :: type_complexity)] impl Methods { # [doc = "Generated by `ethcontract`"] pub fn allowance (& self , owner : self :: ethcontract :: Address , spender : self :: ethcontract :: Address) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < self :: ethcontract :: U256 > { self . instance . view_method ([221 , 98 , 237 , 62] , (owner , spender ,)) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn approve (& self , spender : self :: ethcontract :: Address , value : self :: ethcontract :: U256) -> self :: ethcontract :: dyns :: DynMethodBuilder < bool > { self . instance . method ([9 , 94 , 167 , 179] , (spender , value ,)) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn balance_of (& self , owner : self :: ethcontract :: Address) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < self :: ethcontract :: U256 > { self . instance . view_method ([112 , 160 , 130 , 49] , (owner ,)) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn decimals (& self) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < u8 > { self . instance . view_method ([49 , 60 , 229 , 103] , ()) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn name (& self) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < String > { self . instance . view_method ([6 , 253 , 222 , 3] , ()) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn symbol (& self) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < String > { self . instance . view_method ([149 , 216 , 155 , 65] , ()) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn total_supply (& self) -> self :: ethcontract :: dyns :: DynViewMethodBuilder < self :: ethcontract :: U256 > { self . instance . view_method ([24 , 22 , 13 , 221] , ()) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn transfer (& self , to : self :: ethcontract :: Address , value : self :: ethcontract :: U256) -> self :: ethcontract :: dyns :: DynMethodBuilder < bool > { self . instance . method ([169 , 5 , 156 , 187] , (to , value ,)) . expect ("generated call") } # [doc = "Generated by `ethcontract`"] pub fn transfer_from (& self , from : self :: ethcontract :: Address , to : self :: ethcontract :: Address , value : self :: ethcontract :: U256) -> self :: ethcontract :: dyns :: DynMethodBuilder < bool > { self . instance . method ([35 , 184 , 114 , 221] , (from , to , value ,)) . expect ("generated call") } } impl std :: ops :: Deref for Contract { type Target = Methods ; fn deref (& self) -> & Self :: Target { & self . methods } } impl Contract { # [doc = r" Returns a method builder to setup a call to a smart"] # [doc = r" contract's fallback function."] pub fn fallback < D > (& self , data : D) -> self :: ethcontract :: dyns :: DynMethodBuilder < () > where D : Into < Vec < u8 >> , { self . raw_instance () . fallback (data) . expect ("generated fallback method") } } # [doc = r" Module containing all generated data models for this contract's"] # [doc = r" events."] pub mod event_data { use super :: ethcontract ; # [derive (Clone , Debug , Default , Eq , PartialEq ,)] pub struct Approval { pub owner : self :: ethcontract :: Address , pub spender : self :: ethcontract :: Address , pub value : self :: ethcontract :: U256 , } impl Approval { # [doc = r" Retrieves the signature for the event this data corresponds to."] # [doc = r" This signature is the Keccak-256 hash of the ABI signature of"] # [doc = r" this event."] pub fn signature () -> self :: ethcontract :: H256 { self :: ethcontract :: H256 ([140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37]) } # [doc = r" Retrieves the ABI signature for the event this data corresponds"] # [doc = r" to. For this event the value should always be:"] # [doc = r""] # [doc = "`Approval(address,address,uint256)`"] pub fn abi_signature () -> & 'static str { "Approval(address,address,uint256)" } } impl self :: ethcontract :: tokens :: Tokenize for Approval { fn from_token (token : self :: ethcontract :: common :: abi :: Token ,) -> Result < Self , self :: ethcontract :: tokens :: Error > { let (owner , spender , value ,) = self :: ethcontract :: tokens :: Tokenize :: from_token (token) ? ; Ok (Approval { owner , spender , value }) } fn into_token (self) -> self :: ethcontract :: common :: abi :: Token { unimplemented ! ("events are only decoded, not encoded") } } # [derive (Clone , Debug , Default , Eq , PartialEq ,)] pub struct Transfer { pub from : self :: ethcontract :: Address , pub to : self :: ethcontract :: Address , pub value : self :: ethcontract :: U256 , } impl Transfer { # [doc = r" Retrieves the signature for the event this data corresponds to."] # [doc = r" This signature is the Keccak-256 hash of the ABI signature of"] # [doc = r" this event."] pub fn signature () -> self :: ethcontract :: H256 { self :: ethcontract :: H256 ([221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239]) } # [doc = r" Retrieves the ABI signature for the event this data corresponds"] # [doc = r" to. For this event the value should always be:"] # [doc = r""] # [doc = "`Transfer(address,address,uint256)`"] pub fn abi_signature () -> & 'static str { "Transfer(address,address,uint256)" } } impl self :: ethcontract :: tokens :: Tokenize for Transfer { fn from_token (token : self :: ethcontract :: common :: abi :: Token ,) -> Result < Self , self :: ethcontract :: tokens :: Error > { let (from , to , value ,) = self :: ethcontract :: tokens :: Tokenize :: from_token (token) ? ; Ok (Transfer { from , to , value }) } fn into_token (self) -> self :: ethcontract :: common :: abi :: Token { unimplemented ! ("events are only decoded, not encoded") } } } impl Contract { # [doc = r" Retrieves a handle to a type containing for creating event"] # [doc = r" streams for all the contract events."] pub fn events (& self) -> Events < '_ > { Events { instance : self . raw_instance () , } } } pub struct Events < 'a > { instance : & 'a self :: ethcontract :: dyns :: DynInstance , } impl Events < '_ > { # [doc = r" Generated by `ethcontract`."] pub fn approval (& self) -> self :: event_builders :: ApprovalBuilder { self :: event_builders :: ApprovalBuilder (self . instance . event (self :: ethcontract :: H256 ([140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37])) . expect ("generated event filter") ,) } # [doc = r" Generated by `ethcontract`."] pub fn transfer (& self) -> self :: event_builders :: TransferBuilder { self :: event_builders :: TransferBuilder (self . instance . event (self :: ethcontract :: H256 ([221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239])) . expect ("generated event filter") ,) } } # [doc = r" Module containing the generated event stream builders with type safe"] # [doc = r" filter methods for this contract's events."] pub mod event_builders { use super :: ethcontract ; use super :: event_data ; # [doc = "A builder for creating a filtered stream of `Approval` events."] pub struct ApprovalBuilder (# [doc = r" The inner event builder."] pub self :: ethcontract :: dyns :: DynEventBuilder < self :: event_data :: Approval > ,) ; impl ApprovalBuilder { # [doc = r" Sets the starting block from which to stream logs for."] # [doc = r""] # [doc = r" If left unset defaults to the latest block."] # [allow (clippy :: wrong_self_convention)] pub fn from_block (mut self , block : self :: ethcontract :: BlockNumber) -> Self { self . 0 = (self . 0) . from_block (block) ; self } # [doc = r" Sets the last block from which to stream logs for."] # [doc = r""] # [doc = r" If left unset defaults to the streaming until the end of days."] # [allow (clippy :: wrong_self_convention)] pub fn to_block (mut self , block : self :: ethcontract :: BlockNumber) -> Self { self . 0 = (self . 0) . to_block (block) ; self } # [doc = r" Limits the number of events that can be retrieved by this filter."] # [doc = r""] # [doc = r" Note that this parameter is non-standard."] pub fn limit (mut self , value : usize) -> Self { self . 0 = (self . 0) . limit (value) ; self } # [doc = r" Sets the polling interval. This is used as the interval between"] # [doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."] pub fn poll_interval (mut self , value : std :: time :: Duration) -> Self { self . 0 = (self . 0) . poll_interval (value) ; self } # [doc = "Adds a filter for the owner event parameter."] pub fn owner (mut self , topic : self :: ethcontract :: Topic < self :: ethcontract :: Address >) -> Self { self . 0 = (self . 0) . topic0 (topic) ; self } # [doc = "Adds a filter for the spender event parameter."] pub fn spender (mut self , topic : self :: ethcontract :: Topic < self :: ethcontract :: Address >) -> Self { self . 0 = (self . 0) . topic1 (topic) ; self } # [doc = r" Returns a future that resolves with a collection of all existing"] # [doc = r" logs matching the builder parameters."] pub async fn query (self) -> std :: result :: Result < std :: vec :: Vec < self :: ethcontract :: Event < self :: event_data :: Approval >> , self :: ethcontract :: errors :: EventError , > { (self . 0) . query () . await } # [doc = r" Creates an event stream from the current event builder."] pub fn stream (self) -> impl self :: ethcontract :: futures :: stream :: Stream < Item = std :: result :: Result < self :: ethcontract :: StreamEvent < self :: event_data :: Approval > , self :: ethcontract :: errors :: EventError , > , > { (self . 0) . stream () } } # [doc = "A builder for creating a filtered stream of `Transfer` events."] pub struct TransferBuilder (# [doc = r" The inner event builder."] pub self :: ethcontract :: dyns :: DynEventBuilder < self :: event_data :: Transfer > ,) ; impl TransferBuilder { # [doc = r" Sets the starting block from which to stream logs for."] # [doc = r""] # [doc = r" If left unset defaults to the latest block."] # [allow (clippy :: wrong_self_convention)] pub fn from_block (mut self , block : self :: ethcontract :: BlockNumber) -> Self { self . 0 = (self . 0) . from_block (block) ; self } # [doc = r" Sets the last block from which to stream logs for."] # [doc = r""] # [doc = r" If left unset defaults to the streaming until the end of days."] # [allow (clippy :: wrong_self_convention)] pub fn to_block (mut self , block : self :: ethcontract :: BlockNumber) -> Self { self . 0 = (self . 0) . to_block (block) ; self } # [doc = r" Limits the number of events that can be retrieved by this filter."] # [doc = r""] # [doc = r" Note that this parameter is non-standard."] pub fn limit (mut self , value : usize) -> Self { self . 0 = (self . 0) . limit (value) ; self } # [doc = r" Sets the polling interval. This is used as the interval between"] # [doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."] pub fn poll_interval (mut self , value : std :: time :: Duration) -> Self { self . 0 = (self . 0) . poll_interval (value) ; self } # [doc = "Adds a filter for the from event parameter."] pub fn from (mut self , topic : self :: ethcontract :: Topic < self :: ethcontract :: Address >) -> Self { self . 0 = (self . 0) . topic0 (topic) ; self } # [doc = "Adds a filter for the to event parameter."] pub fn to (mut self , topic : self :: ethcontract :: Topic < self :: ethcontract :: Address >) -> Self { self . 0 = (self . 0) . topic1 (topic) ; self } # [doc = r" Returns a future that resolves with a collection of all existing"] # [doc = r" logs matching the builder parameters."] pub async fn query (self) -> std :: result :: Result < std :: vec :: Vec < self :: ethcontract :: Event < self :: event_data :: Transfer >> , self :: ethcontract :: errors :: EventError , > { (self . 0) . query () . await } # [doc = r" Creates an event stream from the current event builder."] pub fn stream (self) -> impl self :: ethcontract :: futures :: stream :: Stream < Item = std :: result :: Result < self :: ethcontract :: StreamEvent < self :: event_data :: Transfer > , self :: ethcontract :: errors :: EventError , > , > { (self . 0) . stream () } } } impl Contract { # [doc = r" Returns a log stream with all events."] pub fn all_events (& self) -> self :: ethcontract :: dyns :: DynAllEventsBuilder < Event > { self :: ethcontract :: dyns :: DynAllEventsBuilder :: new (self . raw_instance () . web3 () , self . address () , self . deployment_information () ,) } } # [doc = r" A contract event."] # [derive (Clone , Debug , Eq , PartialEq ,)] pub enum Event { Approval (self :: event_data :: Approval) , Transfer (self :: event_data :: Transfer) , } impl self :: ethcontract :: contract :: ParseLog for Event { fn parse_log (log : self :: ethcontract :: RawLog ,) -> Result < Self , self :: ethcontract :: errors :: ExecutionError > { let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37]) => Ok (Event :: Approval (log . clone () . decode (Contract :: raw_contract () . abi . event ("Approval") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239]) => Ok (Event :: Transfer (log . clone () . decode (Contract :: raw_contract () . abi . event ("Transfer") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ; if let Some (Ok (data)) = standard_event { return Ok (data) ; } Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) } } } pub use self :: erc20 :: Contract as ERC20 ;
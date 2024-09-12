// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(bytes="vec", tag="1")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(bytes="vec", tag="4")]
    pub new_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub timestamp: u64,
    #[prost(bytes="vec", tag="6")]
    pub sequencer_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub l1_gas_price: ::core::option::Option<ResourcePrice>,
    #[prost(message, optional, tag="8")]
    pub l1_data_gas_price: ::core::option::Option<ResourcePrice>,
    #[prost(enumeration="L1DaMode", tag="9")]
    pub l1_da_mode: i32,
    #[prost(string, tag="10")]
    pub starknet_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="11")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionWithReceipt>,
    #[prost(message, optional, tag="12")]
    pub state_update: ::core::option::Option<StateUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcePrice {
    #[prost(bytes="vec", tag="1")]
    pub price_in_fri: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub price_in_wei: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionWithReceipt {
    #[prost(message, optional, tag="12")]
    pub receipt: ::core::option::Option<TransactionReceipt>,
    #[prost(oneof="transaction_with_receipt::Transaction", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub transaction: ::core::option::Option<transaction_with_receipt::Transaction>,
}
/// Nested message and enum types in `TransactionWithReceipt`.
pub mod transaction_with_receipt {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transaction {
        #[prost(message, tag="1")]
        InvokeTransactionV0(super::InvokeTransactionV0),
        #[prost(message, tag="2")]
        InvokeTransactionV1(super::InvokeTransactionV1),
        #[prost(message, tag="3")]
        InvokeTransactionV3(super::InvokeTransactionV3),
        #[prost(message, tag="4")]
        L1HandlerTransaction(super::L1HandlerTransaction),
        #[prost(message, tag="5")]
        DeclareTransactionV0(super::DeclareTransactionV0),
        #[prost(message, tag="6")]
        DeclareTransactionV1(super::DeclareTransactionV1),
        #[prost(message, tag="7")]
        DeclareTransactionV2(super::DeclareTransactionV2),
        #[prost(message, tag="8")]
        DeclareTransactionV3(super::DeclareTransactionV3),
        #[prost(message, tag="9")]
        DeployTransactionV0(super::DeployTransactionV0),
        #[prost(message, tag="10")]
        DeployAccountTransactionV1(super::DeployAccountTransactionV1),
        #[prost(message, tag="11")]
        DeployAccountTransactionV3(super::DeployAccountTransactionV3),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceipt {
    #[prost(bytes="vec", tag="1")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub actual_fee: ::core::option::Option<ActualFee>,
    #[prost(enumeration="ExecutionStatus", tag="3")]
    pub execution_status: i32,
    #[prost(string, tag="4")]
    pub revert_reason: ::prost::alloc::string::String,
    #[prost(enumeration="TransactionType", tag="5")]
    pub r#type: i32,
    #[prost(string, tag="6")]
    pub message_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub messages_sent: ::prost::alloc::vec::Vec<MessagesSent>,
    #[prost(message, repeated, tag="8")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, optional, tag="9")]
    pub execution_resources: ::core::option::Option<ExecutionResources>,
    #[prost(bytes="vec", tag="10")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagesSent {
    #[prost(bytes="vec", tag="1")]
    pub from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub payload: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(bytes="vec", tag="1")]
    pub from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionResources {
    #[prost(uint64, tag="1")]
    pub steps: u64,
    #[prost(uint64, tag="2")]
    pub memory_holes: u64,
    #[prost(uint64, tag="3")]
    pub range_check_builtin_applications: u64,
    #[prost(uint64, tag="4")]
    pub pedersen_builtin_applications: u64,
    #[prost(uint64, tag="5")]
    pub poseidon_builtin_applications: u64,
    #[prost(uint64, tag="6")]
    pub ec_op_builtin_applications: u64,
    #[prost(uint64, tag="7")]
    pub ecdsa_builtin_applications: u64,
    #[prost(uint64, tag="8")]
    pub bitwise_builtin_applications: u64,
    #[prost(uint64, tag="9")]
    pub keccak_builtin_applications: u64,
    #[prost(uint64, tag="10")]
    pub segment_arena_builtin: u64,
    #[prost(message, optional, tag="11")]
    pub data_availability: ::core::option::Option<DataAvailability>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeTransactionV0 {
    #[prost(bytes="vec", tag="2")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="4")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="5")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub entry_point_selector: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeTransactionV1 {
    #[prost(bytes="vec", tag="1")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="4")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeTransactionV3 {
    #[prost(bytes="vec", tag="2")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="5")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub resource_bounds: ::core::option::Option<ResourceBounds>,
    #[prost(bytes="vec", tag="8")]
    pub tip: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="9")]
    pub paymaster_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="10")]
    pub account_deployment_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="11")]
    pub nonce_data_availability_mode: i32,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="12")]
    pub fee_data_availability_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L1HandlerTransaction {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub entry_point_selector: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclareTransactionV0 {
    #[prost(bytes="vec", tag="2")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="5")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclareTransactionV1 {
    #[prost(bytes="vec", tag="2")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="5")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclareTransactionV2 {
    #[prost(bytes="vec", tag="1")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub compiled_class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="5")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclareTransactionV3 {
    #[prost(bytes="vec", tag="2")]
    pub sender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub compiled_class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="5")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="8")]
    pub resource_bounds: ::core::option::Option<ResourceBounds>,
    #[prost(bytes="vec", tag="9")]
    pub tip: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="10")]
    pub paymaster_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="11")]
    pub account_deployment_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="12")]
    pub nonce_data_availability_mode: i32,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="13")]
    pub fee_data_availability_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployTransactionV0 {
    #[prost(bytes="vec", tag="1")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub contract_address_salt: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="4")]
    pub constructor_calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployAccountTransactionV1 {
    #[prost(bytes="vec", tag="1")]
    pub max_fee: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="4")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub contract_address_salt: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub constructor_calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployAccountTransactionV3 {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub contract_address_salt: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="5")]
    pub constructor_calldata: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub resource_bounds: ::core::option::Option<ResourceBounds>,
    #[prost(bytes="vec", tag="8")]
    pub tip: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="9")]
    pub paymaster_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="11")]
    pub nonce_data_availability_mode: i32,
    #[prost(enumeration="FeeDataAvailabilityMode", tag="12")]
    pub fee_data_availability_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceBounds {
    #[prost(message, optional, tag="1")]
    pub l1_gas: ::core::option::Option<Resource>,
    #[prost(message, optional, tag="2")]
    pub l2_gas: ::core::option::Option<Resource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag="1")]
    pub max_amount: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub max_price_per_unit: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(message, optional, tag="1")]
    pub actual_fee: ::core::option::Option<ActualFee>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActualFee {
    #[prost(bytes="vec", tag="1")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub unit: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAvailability {
    #[prost(uint64, tag="1")]
    pub l1_gas: u64,
    #[prost(uint64, tag="2")]
    pub l1_data_gas: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateUpdate {
    #[prost(bytes="vec", tag="2")]
    pub new_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="1")]
    pub old_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub state_diff: ::core::option::Option<StateDiff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateDiff {
    #[prost(message, repeated, tag="1")]
    pub storage_diffs: ::prost::alloc::vec::Vec<ContractStorageDiff>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub deprecated_declared_classes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="3")]
    pub declared_classes: ::prost::alloc::vec::Vec<DeclaredClass>,
    #[prost(message, repeated, tag="4")]
    pub deployed_contracts: ::prost::alloc::vec::Vec<DeployedContract>,
    #[prost(message, repeated, tag="5")]
    pub replaced_classes: ::prost::alloc::vec::Vec<ReplacedClass>,
    #[prost(message, repeated, tag="6")]
    pub nonces: ::prost::alloc::vec::Vec<NonceDiff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonceDiff {
    #[prost(bytes="vec", tag="1")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplacedClass {
    #[prost(bytes="vec", tag="1")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployedContract {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclaredClass {
    #[prost(bytes="vec", tag="1")]
    pub class_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub compiled_class_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractStorageDiff {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub storage_entries: ::prost::alloc::vec::Vec<StorageEntries>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageEntries {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum L1DaMode {
    Unknown = 0,
    Calldata = 1,
    Blob = 2,
}
impl L1DaMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            L1DaMode::Unknown => "L1_DA_MODE_UNKNOWN",
            L1DaMode::Calldata => "CALLDATA",
            L1DaMode::Blob => "BLOB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "L1_DA_MODE_UNKNOWN" => Some(Self::Unknown),
            "CALLDATA" => Some(Self::Calldata),
            "BLOB" => Some(Self::Blob),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeeDataAvailabilityMode {
    Unknown = 0,
    L1 = 1,
    L2 = 2,
}
impl FeeDataAvailabilityMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeeDataAvailabilityMode::Unknown => "FEE_DATA_AVAILABILITY_MODE_UNKNOWN",
            FeeDataAvailabilityMode::L1 => "L1",
            FeeDataAvailabilityMode::L2 => "L2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FEE_DATA_AVAILABILITY_MODE_UNKNOWN" => Some(Self::Unknown),
            "L1" => Some(Self::L1),
            "L2" => Some(Self::L2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    Unknown = 0,
    Invoke = 1,
    Declare = 2,
    Deploy = 3,
    DeployAccount = 4,
    L1Handler = 5,
}
impl TransactionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionType::Unknown => "TRANSACTION_TYPE_UNKNOWN",
            TransactionType::Invoke => "INVOKE",
            TransactionType::Declare => "DECLARE",
            TransactionType::Deploy => "DEPLOY",
            TransactionType::DeployAccount => "DEPLOY_ACCOUNT",
            TransactionType::L1Handler => "L1_HANDLER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTION_TYPE_UNKNOWN" => Some(Self::Unknown),
            "INVOKE" => Some(Self::Invoke),
            "DECLARE" => Some(Self::Declare),
            "DEPLOY" => Some(Self::Deploy),
            "DEPLOY_ACCOUNT" => Some(Self::DeployAccount),
            "L1_HANDLER" => Some(Self::L1Handler),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionStatus {
    Unknown = 0,
    Success = 1,
    Reverted = 2,
}
impl ExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionStatus::Unknown => "EXECUTION_STATUS_UNKNOWN",
            ExecutionStatus::Success => "EXECUTION_STATUS_SUCCESS",
            ExecutionStatus::Reverted => "EXECUTION_STATUS_REVERTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_STATUS_UNKNOWN" => Some(Self::Unknown),
            "EXECUTION_STATUS_SUCCESS" => Some(Self::Success),
            "EXECUTION_STATUS_REVERTED" => Some(Self::Reverted),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)

// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, optional, tag="1")]
    pub clock: ::core::option::Option<super::super::super::v1::Clock>,
    #[prost(message, repeated, tag="2")]
    pub transactions_with_receipt: ::prost::alloc::vec::Vec<super::super::super::super::starknet::r#type::v1::TransactionWithReceipt>,
}
// @@protoc_insertion_point(module)

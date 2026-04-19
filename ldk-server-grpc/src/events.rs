// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

/// EventEnvelope wraps different event types in a single message to be used by EventPublisher.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEnvelope {
	#[prost(oneof = "event_envelope::Event", tags = "2, 3, 4, 6, 7, 8, 9, 10, 11")]
	pub event: ::core::option::Option<event_envelope::Event>,
}
/// Nested message and enum types in `EventEnvelope`.
pub mod event_envelope {
	#[cfg_attr(feature = "serde", derive(serde::Serialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
	#[allow(clippy::derive_partial_eq_without_eq)]
	#[derive(Clone, PartialEq, ::prost::Oneof)]
	pub enum Event {
		#[prost(message, tag = "2")]
		PaymentReceived(super::PaymentReceived),
		#[prost(message, tag = "3")]
		PaymentSuccessful(super::PaymentSuccessful),
		#[prost(message, tag = "4")]
		PaymentFailed(super::PaymentFailed),
		#[prost(message, tag = "6")]
		PaymentForwarded(super::PaymentForwarded),
		#[prost(message, tag = "7")]
		PaymentClaimable(super::PaymentClaimable),
		/// Channel-lifecycle events (RFC #134 parent scope).
		#[prost(message, tag = "8")]
		ChannelPending(super::ChannelPending),
		#[prost(message, tag = "9")]
		ChannelReady(super::ChannelReady),
		#[prost(message, tag = "10")]
		ChannelClosed(super::ChannelClosed),
		/// Per-commitment-update event carrying the cryptographic attestation bundle.
		#[prost(message, tag = "11")]
		ChannelCommitmentUpdated(super::ChannelCommitmentUpdated),
	}
}
/// PaymentReceived indicates a payment has been received.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentReceived {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentSuccessful indicates a sent payment was successful.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentSuccessful {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentFailed indicates a sent payment has failed.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentFailed {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentClaimable indicates a payment has arrived and is waiting to be manually claimed or failed.
/// This event is only emitted for payments created via `Bolt11ReceiveForHash`.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentClaimable {
	/// The payment details for the claimable payment.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentForwarded indicates a payment was forwarded through the node.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentForwarded {
	#[prost(message, optional, tag = "1")]
	pub forwarded_payment: ::core::option::Option<super::types::ForwardedPayment>,
}
/// ChannelPending indicates a new channel has been created and is awaiting
/// on-chain funding confirmation.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPending {
	/// Hex-encoded channel id.
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	/// Hex-encoded user-assigned channel id (16 bytes).
	#[prost(string, tag = "2")]
	pub user_channel_id: ::prost::alloc::string::String,
	/// Pubkey of the counterparty node, hex-encoded.
	#[prost(string, tag = "3")]
	pub counterparty_node_id: ::prost::alloc::string::String,
	/// Funding outpoint (txid:vout) as observed on chain.
	#[prost(string, tag = "4")]
	pub funding_txid: ::prost::alloc::string::String,
	#[prost(uint32, tag = "5")]
	pub funding_output_index: u32,
}
/// ChannelReady indicates a channel has reached the confirmed / usable state.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReady {
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub user_channel_id: ::prost::alloc::string::String,
	/// May be absent in edge cases (see ldk-node docs).
	#[prost(string, optional, tag = "3")]
	pub counterparty_node_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// ChannelClosed indicates a channel has been closed.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClosed {
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub user_channel_id: ::prost::alloc::string::String,
	#[prost(string, optional, tag = "3")]
	pub counterparty_node_id: ::core::option::Option<::prost::alloc::string::String>,
	/// Free-form closure reason string as surfaced by ldk-node.
	#[prost(string, optional, tag = "4")]
	pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// ChannelCommitmentUpdated fires once per commitment-state update and carries
/// the cryptographic bundle an external attestor needs to verify channel state
/// against the counterparty's funding pubkey.
///
/// See the RFC filed on ldk-server:
///    "include commitment-bundle data on channel-state events (extends #134)"
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCommitmentUpdated {
	/// Channel identity.
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub counterparty_node_id: ::prost::alloc::string::String,
	#[prost(string, tag = "3")]
	pub funding_txid: ::prost::alloc::string::String,
	#[prost(uint32, tag = "4")]
	pub funding_outnum: u32,
	/// Monotonic — increments with every new state.
	#[prost(uint64, tag = "5")]
	pub commitment_number: u64,
	/// The cryptographic bundle the attestor cares about.
	#[prost(message, optional, tag = "6")]
	pub bundle: ::core::option::Option<ChannelCommitmentBundle>,
}
/// ChannelCommitmentBundle is the verifiable per-channel-state payload.
///
/// An external verifier reconstructs the BIP-143 sighash over
/// holder_commitment_tx using the 2-of-2 funding redeem script assembled from
/// holder_funding_pubkey / counterparty_funding_pubkey and verifies
/// counterparty_signature against counterparty_funding_pubkey. That closes
/// the loop independent of the operator.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCommitmentBundle {
	/// Raw wire-format unsigned commitment tx (no witnesses).
	#[prost(bytes = "bytes", tag = "1")]
	pub holder_commitment_tx: ::prost::bytes::Bytes,
	/// Counterparty's ECDSA signature over holder_commitment_tx (BIP-143
	/// sighash, SIGHASH_ALL). Receivers should try DER first and fall back to
	/// a 64-byte raw (r||s) encoding.
	#[prost(bytes = "bytes", tag = "2")]
	pub counterparty_signature: ::prost::bytes::Bytes,
	/// 33-byte compressed pubkeys of both funding-output participants.
	#[prost(bytes = "bytes", tag = "3")]
	pub holder_funding_pubkey: ::prost::bytes::Bytes,
	#[prost(bytes = "bytes", tag = "4")]
	pub counterparty_funding_pubkey: ::prost::bytes::Bytes,
	#[prost(uint64, tag = "5")]
	pub capacity_sats: u64,
	#[prost(uint64, tag = "6")]
	pub holder_balance_msat: u64,
	#[prost(uint64, tag = "7")]
	pub counterparty_balance_msat: u64,
	#[prost(uint64, tag = "8")]
	pub commit_fee_sats: u64,
	#[prost(message, repeated, tag = "9")]
	pub pending_htlcs: ::prost::alloc::vec::Vec<AttestationHtlc>,
	/// Channel type marker, e.g. "anchors", "static_remotekey", "taproot".
	#[prost(string, tag = "10")]
	pub channel_type: ::prost::alloc::string::String,
}
/// AttestationHtlc describes a single pending HTLC as recorded on the commitment.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationHtlc {
	/// True if we sent this HTLC (offered), false if received.
	#[prost(bool, tag = "1")]
	pub offered: bool,
	#[prost(uint64, tag = "2")]
	pub amount_msat: u64,
	#[prost(bytes = "bytes", tag = "3")]
	pub payment_hash: ::prost::bytes::Bytes,
	#[prost(uint32, tag = "4")]
	pub cltv_expiry: u32,
}

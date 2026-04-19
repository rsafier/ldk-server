// This file is Copyright its original authors, visible in version control history.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option.

use std::sync::Arc;

use bytes::Bytes;
use hex::prelude::*;
use ldk_node::bitcoin::consensus::encode as btc_encode;
use ldk_node::lightning::ln::types::ChannelId;
use ldk_server_grpc::api::{GetChannelAttestationsRequest, GetChannelAttestationsResponse};
use ldk_server_grpc::events::{
	AttestationHtlc, ChannelCommitmentBundle, ChannelCommitmentUpdated,
};

use crate::api::error::{LdkServerError, LdkServerErrorCode};
use crate::service::Context;

/// Returns per-channel cryptographic attestation bundles for the caller-requested channels
/// (all open channels if none specified).
///
/// Backed by ldk-node's `Node::export_channel_attestation` — emits the unsigned commitment
/// transaction, counterparty signature, both funding pubkeys, balances, and pending HTLCs.
pub(crate) async fn handle_get_channel_attestations_request(
	context: Arc<Context>, request: GetChannelAttestationsRequest,
) -> Result<GetChannelAttestationsResponse, LdkServerError> {
	let want: Vec<ChannelId> = request
		.channel_ids
		.iter()
		.map(|hex_id| decode_channel_id(hex_id))
		.collect::<Result<Vec<_>, _>>()?;

	let attestations: Vec<ChannelCommitmentUpdated> = if want.is_empty() {
		// Materialize per-channel so we can surface errors instead of silently
		// dropping them (the filter_map(.ok()) in Node::list_channel_attestations
		// makes debugging extraction bugs opaque).
		let mut out = Vec::new();
		for c in context.node.list_channels() {
			match context.node.export_channel_attestation(&c.channel_id) {
				Ok(att) => out.push(attestation_to_proto(att)),
				Err(e) => log::warn!(
					"export_channel_attestation({}) failed: {:?}",
					c.channel_id.0.to_lower_hex_string(),
					e
				),
			}
		}
		out
	} else {
		let mut out = Vec::new();
		for cid in &want {
			match context.node.export_channel_attestation(cid) {
				Ok(att) => out.push(attestation_to_proto(att)),
				Err(e) => {
					return Err(LdkServerError::new(
						LdkServerErrorCode::InternalServerError,
						format!("export_channel_attestation failed for {}: {:?}",
							cid.0.to_lower_hex_string(), e),
					));
				},
			}
		}
		out
	};

	Ok(GetChannelAttestationsResponse { attestations })
}

fn decode_channel_id(hex_id: &str) -> Result<ChannelId, LdkServerError> {
	let bytes = <[u8; 32]>::from_hex(hex_id).map_err(|e| {
		LdkServerError::new(
			LdkServerErrorCode::InvalidRequestError,
			format!("invalid channel_id hex: {e}"),
		)
	})?;
	Ok(ChannelId(bytes))
}

fn attestation_to_proto(
	att: ldk_node::attestation::ChannelAttestation,
) -> ChannelCommitmentUpdated {
	let commit_tx_bytes = btc_encode::serialize(&att.holder_commitment_tx);
	let bundle = ChannelCommitmentBundle {
		holder_commitment_tx: Bytes::from(commit_tx_bytes),
		counterparty_signature: Bytes::from(att.counterparty_signature.serialize_der().to_vec()),
		holder_funding_pubkey: Bytes::from(att.holder_funding_pubkey.serialize().to_vec()),
		counterparty_funding_pubkey: Bytes::from(
			att.counterparty_funding_pubkey.serialize().to_vec(),
		),
		capacity_sats: att.capacity_sats,
		holder_balance_msat: att.holder_balance_msat,
		counterparty_balance_msat: att.counterparty_balance_msat,
		commit_fee_sats: 0, // derivable from tx outputs by the consumer
		pending_htlcs: att
			.pending_htlcs
			.into_iter()
			.map(|h| AttestationHtlc {
				offered: h.offered,
				amount_msat: h.amount_msat,
				payment_hash: Bytes::from(h.payment_hash.to_vec()),
				cltv_expiry: h.cltv_expiry,
			})
			.collect(),
		channel_type: String::new(),
	};
	ChannelCommitmentUpdated {
		channel_id: att.channel_id.0.to_lower_hex_string(),
		counterparty_node_id: att.counterparty_node_id.to_string(),
		funding_txid: att.funding_outpoint.txid.to_string(),
		funding_outnum: att.funding_outpoint.vout,
		commitment_number: att.commitment_number,
		bundle: Some(bundle),
	}
}

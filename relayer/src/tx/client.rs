use prost_types::Any;
use std::convert::TryInto;
use std::time::Duration;

use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState, AnyHeader};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics02_client::msgs::create_client::MsgCreateAnyClient;
use ibc::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::ics07_tendermint::header::Header as TendermintHeader;
use ibc::ics24_host::identifier::{ClientId, ChainId};
use ibc::ics24_host::Path::ClientState as ClientStatePath;
use ibc::ics24_host::Path::ClientConsensusState;

use ibc::tx_msg::Msg;

use crate::chain::cosmos::block_on;
use crate::chain::{query_header_at_height, query_latest_header, Chain, CosmosSDKChain};
use crate::config::ChainConfig;
use crate::error::{Error, Kind};
use ibc::Height;

#[derive(Clone, Debug)]
pub struct CreateClientOptions {
    pub dest_client_id: ClientId,
    pub dest_chain_config: ChainConfig,
    pub src_chain_config: ChainConfig,
}

pub fn create_client(opts: CreateClientOptions) -> Result<(), Error> {
    // Get the destination chain
    let dest_chain = CosmosSDKChain::from_config(opts.clone().dest_chain_config)?;

    // Query the client state on destination chain.
    let response = dest_chain.query(
        ClientStatePath(opts.clone().dest_client_id),
        tendermint::block::Height::from(0_u32),
        false,
    );

    if response.is_ok() {
        return Err(Into::<Error>::into(Kind::CreateClient(
            opts.dest_client_id,
            "client already exists".into(),
        )));
    }

    // Get the latest header from the source chain and build the consensus state.
    let src_chain = CosmosSDKChain::from_config(opts.clone().src_chain_config)?;
    let tm_latest_header =
        block_on(query_latest_header::<CosmosSDKChain>(&src_chain)).map_err(|e| {
            Kind::CreateClient(
                opts.dest_client_id.clone(),
                "failed to get the latest header".into(),
            )
            .context(e)
        })?;

    let height = u64::from(tm_latest_header.signed_header.header().height);
    let version = tm_latest_header.signed_header.header().chain_id.to_string();

    let tm_consensus_state = ibc::ics07_tendermint::consensus_state::ConsensusState::from(
        tm_latest_header.signed_header,
    );

    let any_consensus_state = AnyConsensusState::Tendermint(tm_consensus_state);

    // Build the client state.
    let any_client_state = ibc::ics07_tendermint::client_state::ClientState::new(
        src_chain.id().to_string(),
        src_chain.trusting_period(),
        src_chain.unbonding_period(),
        Duration::from_millis(3000),
        Height::new(ChainId::chain_version(version.clone()), height),
        Height::new(ChainId::chain_version(version), 0),
        "".to_string(),
        false,
        false,
    )
    .map_err(|e| {
        Kind::CreateClient(
            opts.dest_client_id.clone(),
            "failed to build the client state".into(),
        )
        .context(e)
    })
    .map(AnyClientState::Tendermint)?;

    let signer = dest_chain.config().account_prefix.parse().map_err(|e| {
        Kind::CreateClient(opts.dest_client_id.clone(), "bad signer".into()).context(e)
    })?;

    // Build the domain type message
    let new_msg = MsgCreateAnyClient::new(
        opts.dest_client_id,
        any_client_state,
        any_consensus_state,
        signer,
    )
    .map_err(|e| {
        Kind::MessageTransaction("failed to build the create client message".into()).context(e)
    })?;

    // Create a proto any message
    let mut proto_msgs: Vec<Any> = Vec::new();
    let any_msg = Any {
        // TODO - add get_url_type() to prepend proper string to get_type()
        type_url: "/ibc.client.MsgCreateClient".to_ascii_lowercase(),
        value: new_msg.get_sign_bytes(),
    };

    proto_msgs.push(any_msg);
    dest_chain.send(&proto_msgs)
}

#[derive(Clone, Debug)]
pub struct UpdateClientOptions {
    pub dest_client_id: ClientId,
    pub dest_chain_config: ChainConfig,
    pub src_chain_config: ChainConfig,
}

pub fn update_client(opts: UpdateClientOptions) -> Result<(), Error> {
    // Get the destination
    let dest_chain = CosmosSDKChain::from_config(opts.clone().dest_chain_config)?;

    // Check if the client exists on destination chain.
    let last_state = dest_chain
        .query(
            ClientStatePath(opts.clone().dest_client_id),
            0_u64.try_into().map_err(|e| Kind::BadParameter.context(e))?,
            false,
        )
        .map_err(|e| Kind::Query.context(e).into())
        .and_then(|v| AnyClientState::decode_vec(&v).map_err(|e| Kind::Query.context(e).into()))
        .map_err(|e| Kind::Query.context(e).into())?;

    // Query the last consensus state.
    let last_consensus = dest_chain
        .query(
            ClientConsensusState {
                client_id: opts.dest_client_id,
                epoch: last_state.latest_height().version_number,
                height: last_state.latest_height().version_height,
            },
            0.try_into()?,
            false,
        )
        .map_err(|e| Kind::Query.context(e).into())
        .and_then(|v| AnyConsensusState::decode_vec(&v).map_err(|e| Kind::Query.context(e).into()))
        .map_err(|e| Kind::Query.context(e).into())?;

    match (last_state, last_consensus) {
        (
            Some(AnyClientState::Tendermint(tm_last_state)),
            Some(AnyConsensusState::Tendermint(tm_last_consensus)),
        ) => {
            let src_chain = CosmosSDKChain::from_config(opts.clone().src_chain_config)?;
            let tm_header = block_on(query_header_at_height::<CosmosSDKChain>(
                &src_chain,
                last_state.latest_height().version_height.try_into().unwrap(),
            ))
            .map_err(|e| {
                Kind::UpdateClient(
                    opts.dest_client_id.clone(),
                    "failed to get the latest header".into(),
                )
                .context(e)
            })?;

            let header = AnyHeader::Tendermint(TendermintHeader {
                signed_header: tm_header.signed_header,
                validator_set: tm_header.validators.clone(),
                trusted_height: tm_last_state.latest_height,
                trusted_validator_set: tm_header.next_validators, // TODO - just to compile, FIXME
            });

            let signer = dest_chain.config().account_prefix.parse().map_err(|e| {
                Kind::CreateClient(opts.dest_client_id.clone(), "bad signer".into()).context(e)
            })?;

            // Build the domain type message
            let new_msg = MsgUpdateAnyClient {
                client_id: opts.dest_client_id.clone(),
                header,
                signer,
            };

            // Create a proto any message
            let mut proto_msgs: Vec<Any> = Vec::new();
            let any_msg = Any {
                // TODO - add get_url_type() to prepend proper string to get_type()
                type_url: "/ibc.client.MsgUpdateClient".to_ascii_lowercase(),
                value: new_msg.get_sign_bytes(),
            };

            proto_msgs.push(any_msg);
            dest_chain.send(&proto_msgs)
        }
        _ => Err(Kind::UpdateClient(opts.dest_client_id.clone(), "bad chain".into()).into())
    }
}

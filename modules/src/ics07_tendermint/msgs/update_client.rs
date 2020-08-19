use crate::ics07_tendermint::header::Header;
use crate::ics24_host::identifier::ClientId;
use crate::tx_msg::Msg;

use crate::ics07_tendermint::consensus_state::ConsensusState;
use serde_derive::{Deserialize, Serialize};
use tendermint::account::Id as AccountId;

pub const TYPE_MSG_UPDATE_CLIENT: &str = "update_client";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgUpdateClient {
    client_id: ClientId,
    header: Header,
    signer: AccountId,
}

impl MsgUpdateClient {
    pub fn new(client_id: ClientId, header: Header, signer: AccountId) -> Self {
        Self {
            client_id,
            header,
            signer,
        }
    }

    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn consensus_state(&self) -> ConsensusState {
        self.header.consensus_state()
    }
}

impl Msg for MsgUpdateClient {
    type ValidationError = crate::ics24_host::error::ValidationError;

    fn route(&self) -> String {
        crate::keys::ROUTER_KEY.to_string()
    }

    fn get_type(&self) -> String {
        TYPE_MSG_UPDATE_CLIENT.to_string()
    }

    fn validate_basic(&self) -> Result<(), Self::ValidationError> {
        // Nothing to validate since both ClientId and AccountId perform validation on creation.
        Ok(())
    }

    fn get_sign_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn get_signers(&self) -> Vec<AccountId> {
        vec![self.signer]
    }
}

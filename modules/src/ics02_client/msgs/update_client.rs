use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;
use std::convert::TryFrom;
use tendermint::account::Id as AccountId;
use tendermint_proto::{DomainType, Error, Kind};

use crate::ics02_client::client_def::AnyHeader;
use crate::ics02_client::error;
use crate::ics24_host::identifier::ClientId;
use crate::tx_msg::Msg;
use std::str::FromStr;

const TYPE_MSG_UPDATE_CLIENT: &str = "update_client";

/// A type of message that triggers the update of an on-chain (IBC) client with new headers.
#[derive(Clone, Debug)]
pub struct MsgUpdateAnyClient {
    pub client_id: ClientId,
    pub header: AnyHeader,
    pub signer: AccountId,
}

impl Msg for MsgUpdateAnyClient {
    type ValidationError = crate::ics24_host::error::ValidationError;

    fn route(&self) -> String {
        crate::keys::ROUTER_KEY.to_string()
    }

    fn get_type(&self) -> String {
        TYPE_MSG_UPDATE_CLIENT.to_string()
    }

    fn validate_basic(&self) -> Result<(), Self::ValidationError> {
        // Nothing to validate since all fields are validated on creation.
        Ok(())
    }

    fn get_sign_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let raw_msg: RawMsgUpdateClient = self.clone().into();
        prost::Message::encode(&raw_msg, &mut buf).unwrap();
        buf
    }

    fn get_signers(&self) -> Vec<AccountId> {
        vec![self.signer]
    }
}

impl DomainType<RawMsgUpdateClient> for MsgUpdateAnyClient {}

impl TryFrom<RawMsgUpdateClient> for MsgUpdateAnyClient {
    type Error = Error;

    fn try_from(raw: RawMsgUpdateClient) -> Result<Self, Self::Error> {
        let raw_header = raw
            .header
            .ok_or_else(|| Kind::DecodeMessage.context(error::Kind::InvalidRawHeader))?;

        Ok(MsgUpdateAnyClient {
            client_id: raw.client_id.parse().unwrap(),
            header: AnyHeader::try_from(raw_header).unwrap(),
            signer: AccountId::from_str(raw.signer.as_str())
                .map_err(|e| Kind::DecodeMessage.context(e))?,
        })
    }
}

impl From<MsgUpdateAnyClient> for RawMsgUpdateClient {
    fn from(ics_msg: MsgUpdateAnyClient) -> Self {
        RawMsgUpdateClient {
            client_id: ics_msg.client_id.to_string(),
            header: Some(ics_msg.header.into()),
            signer: ics_msg.signer.to_string(),
        }
    }
}

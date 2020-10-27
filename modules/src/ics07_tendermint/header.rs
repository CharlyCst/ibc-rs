use serde_derive::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

use tendermint::block::signed_header::SignedHeader;
use tendermint::validator::Set as ValidatorSet;
use tendermint_proto::DomainType;

use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;

use crate::Height;

use crate::ics02_client::client_type::ClientType;
use crate::ics07_tendermint::consensus_state::ConsensusState;
use crate::ics07_tendermint::error::{Error, Kind};
use crate::ics23_commitment::commitment::CommitmentRoot;
use crate::ics24_host::identifier::ChainId;

/// Tendermint consensus header
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub signed_header: SignedHeader, // contains the commitment root
    pub validator_set: ValidatorSet, // the validator set that signed Header
    pub trusted_height: Height, // the height of a trusted header seen by client less than or equal to Header
    pub trusted_validator_set: ValidatorSet, // the last trusted validator set at trusted height
}

impl Header {
    pub(crate) fn consensus_state(&self) -> ConsensusState {
        ConsensusState {
            timestamp: self.signed_header.header().time,
            root: CommitmentRoot::from_bytes(self.signed_header.header().app_hash.as_ref()),
            next_validators_hash: self.signed_header.header().next_validators_hash,
        }
    }
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Tendermint
    }

    fn height(&self) -> Height {
        Height::new(
            ChainId::chain_version(self.signed_header.header().chain_id.to_string()),
            u64::from(self.signed_header.header().height),
        )
    }

    // fn consensus_state(&self) -> &dyn crate::ics02_client::state::ConsensusState {
    //     &self.consensus_state()
    // }
}

impl DomainType<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        let sh = raw
            .signed_header
            .ok_or_else(|| Kind::InvalidRawHeader.context("missing signed header"))?;

        let signed_header: SignedHeader = sh
            .try_into()
            .map_err(|_| Kind::InvalidHeader.context("signed header conversion"))?;
        Ok(Self {
            signed_header,
            validator_set: raw
                .validator_set
                .ok_or_else(|| Kind::InvalidRawHeader.context("missing validator set"))?
                .try_into()
                .map_err(|e| Kind::InvalidRawHeader.context(e))?,
            trusted_height: raw
                .trusted_height
                .ok_or_else(|| Kind::InvalidRawHeader.context("missing height"))?
                .try_into()
                .map_err(|e| Kind::InvalidRawHeight.context(e))?,
            trusted_validator_set: raw
                .trusted_validators
                .ok_or_else(|| Kind::InvalidRawHeader.context("missing trusted validator set"))?
                .try_into()
                .map_err(|e| Kind::InvalidRawHeader.context(e))?,
        })
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        RawHeader {
            signed_header: Some(value.signed_header.into()),
            validator_set: Some(value.validator_set.into()),
            trusted_height: Some(value.trusted_height.into()),
            trusted_validators: Some(value.trusted_validator_set.into()),
        }
    }
}

#[cfg(test)]
pub mod test_util {
    use subtle_encoding::hex;

    use tendermint::block::signed_header::SignedHeader;
    use tendermint::validator::Info as ValidatorInfo;
    use tendermint::validator::Set as ValidatorSet;
    use tendermint::PublicKey;

    use crate::ics07_tendermint::header::Header;
    use crate::Height;
    use std::convert::TryInto;

    // TODO: This should be replaced with a ::default() or ::produce().
    // The implementation of this function comprises duplicate code (code borrowed from
    // `tendermint-rs` for assembling a Header).
    // See https://github.com/informalsystems/tendermint-rs/issues/381.
    pub fn get_dummy_header() -> Header {
        // Build a SignedHeader from a JSON file.
        let shdr = serde_json::from_str::<SignedHeader>(include_str!(
            "../../tests/support/signed_header.json"
        ))
        .unwrap();

        // Build a set of validators.
        // Below are test values inspired form `test_validator_set()` in tendermint-rs.
        let v1: ValidatorInfo = ValidatorInfo::new(
            PublicKey::from_raw_ed25519(
                &hex::decode_upper(
                    "F349539C7E5EF7C49549B09C4BFC2335318AB0FE51FBFAA2433B4F13E816F4A7",
                )
                .unwrap(),
            )
            .unwrap(),
            281_815_u64.try_into().unwrap(),
        );

        let vs = ValidatorSet::new(vec![v1], None, 281_815_u64.try_into().unwrap());

        Header {
            signed_header: shdr,
            validator_set: vs.clone(),
            trusted_height: Height::new(0, 9),
            trusted_validator_set: vs,
        }
    }
}

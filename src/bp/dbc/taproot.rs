// LNP/BP Rust Library
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use bitcoin::hashes::sha256;
use bitcoin::secp256k1;

use super::{Container, Error, Proof, ProofSuppl, PubkeyCommitment};
use crate::commit_verify::CommitEmbedVerify;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display_from(Debug)]
pub struct TaprootContainer {
    pub script_root: sha256::Hash,
    pub intermediate_key: secp256k1::PublicKey,
}

impl Container for TaprootContainer {
    type Supplement = Option<()>;
    type Commitment = Option<()>;

    fn restore(proof: &Proof, _: &Self::Supplement, _: &Self::Commitment) -> Result<Self, Error> {
        if let ProofSuppl::Taproot(tapscript_root) = proof.suppl {
            Ok(Self {
                script_root: tapscript_root,
                intermediate_key: proof.pubkey,
            })
        } else {
            Err(Error::InvalidProofSupplement)
        }
    }

    fn to_proof(&self) -> Proof {
        Proof {
            pubkey: self.intermediate_key.clone(),
            suppl: ProofSuppl::Taproot(self.script_root.clone()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display_from(Debug)]
pub struct TaprootCommitment {
    pub script_root: sha256::Hash,
    pub intermediate_ke_commitment: PubkeyCommitment,
}

impl<MSG> CommitEmbedVerify<MSG> for TaprootCommitment
where
    MSG: AsRef<[u8]>,
{
    type Container = TaprootContainer;
    type Error = Error;

    fn commit_embed(container: Self::Container, msg: &MSG) -> Result<Self, Self::Error> {
        let cmt = PubkeyCommitment::commit_embed(container.intermediate_key, msg)?;
        Ok(Self {
            script_root: container.script_root,
            intermediate_ke_commitment: cmt,
        })
    }
}
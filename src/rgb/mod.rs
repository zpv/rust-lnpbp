// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
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

pub mod bech32;
pub mod contract;
pub mod schema;
pub mod stash;
pub mod validation;
pub mod vm;

pub mod prelude {
    pub use super::bech32::{Bech32, FromBech32, ToBech32};
    use super::*;
    pub use super::{bech32, schema, vm};
    pub use contract::{
        data, seal, value, Assignments, AtomicValue, AutoConceal,
        ConfidentialState, ContractId, DeclarativeStrategy, Extension, Genesis,
        HashStrategy, Metadata, NoDataError, Node, NodeId, OwnedRights,
        OwnedState, ParentOwnedRights, ParentPublicRights, PedersenStrategy,
        RevealedState, SealDefinition, StateTypes, Transition,
    };
    pub use schema::{
        script, AssignmentAbi, AssignmentAction, ExtensionAbi, ExtensionAction,
        ExtensionSchema, ExtensionType, GenesisAbi, GenesisAction,
        PublicRightType, PublicRightsStructure, Schema, SchemaId,
        SimplicityScript, TransitionAbi, TransitionAction,
    };
    pub use stash::{
        Anchor, AnchorId, Consignment, ConsignmentEndpoints, Disclosure, Stash,
        TransitionData, PSBT_OUT_PUBKEY, PSBT_OUT_TWEAK,
    };
    pub use validation::{Validator, Validity};
    pub use vm::VirtualMachine;
}

pub use prelude::*;

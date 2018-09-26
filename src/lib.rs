// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


/// for POOL Subprotocol
/// verify transaction input by user then output transaction used by chain
pub trait VerifyTransaction {
    type UnverifiedTransaction;
    type SignedTransaction;

    fn verify(utx: Self::UnverifiedTransaction) -> Option<Self::SignedTransaction>;
}

/// for RPC Subprotocol
/// extract user input transaction from transaction on chain
pub trait ExtractTransaction {
    type UnverifiedTransaction;
    type SignedTransaction;

    fn extract(utx: Self::SignedTransaction) -> Option<Self::UnverifiedTransaction>;
}

/// for Consensus Subprotocol
/// Consensus will define ProposalBlock itself then convert ProposalBlock into Block
pub trait GenerateBlock {
    type Block;

    fn generate_block(&self) -> Option<Self::Block>;
}

/// for Consensus Subprotocol
/// Consensus will define Votes itself then convert Votes into Proof
pub trait GenerateProof {
    type Proof;

    fn generate_proof(&self) -> Option<Self::Proof>;
}

/// for Consensus Subprotocol
/// extract votes from proof and verify it
pub trait VerifyProof {
    type Proof;

    fn verify_proof(proof: Self::Proof) -> Bool;
}

/// for Chain Subprotocol
/// verify the block from peer
pub trait VerifyBlock {
    type Block;

    fn verify_block(block: Self::Block) -> Bool;
}

/// for Chain Subprotocol
/// input chain and signed transaction, modify the state of chain follow vm, output receipt
pub trait EvalTransaction {
    type SignedTransaction;
    type Receipt;

    fn eval(&mut self, stx: Self::SignedTransaction) -> Self::Receipt;
}

/// for Chain Subprotocol
/// add a new Block to chain
pub trait AddBlock {
    type Block;

    fn add_block(&mut self, block: Self::Block);
}






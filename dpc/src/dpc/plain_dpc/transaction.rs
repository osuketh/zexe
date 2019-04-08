use crate::{
    crypto_primitives::{CommitmentScheme, NIZK, PRF},
    dpc::{plain_dpc::PlainDPCComponents, Transaction},
};
// use algebra::bytes::ToBytes;

#[derive(Derivative)]
#[derivative(
    Clone(bound = "C: PlainDPCComponents"),
    PartialEq(bound = "C: PlainDPCComponents"),
    Eq(bound = "C: PlainDPCComponents")
)]
pub struct DPCTransaction<C: PlainDPCComponents> {
    pub old_serial_numbers: Vec<<C::P as PRF>::Output>,
    pub new_commitments:    Vec<<C::RecC as CommitmentScheme>::Output>,
    pub memorandum:         [u8; 32],
    pub stuff:          DPCStuff<C>,
}

#[derive(Derivative)]
#[derivative(
    Clone(bound = "C: PlainDPCComponents"),
    PartialEq(bound = "C: PlainDPCComponents"),
    Eq(bound = "C: PlainDPCComponents")
)]
pub struct DPCStuff<C: PlainDPCComponents> {
    pub digest: C::D,
    #[derivative(PartialEq = "ignore")]
    pub core_proof: <C::MainNIZK as NIZK>::Proof,
    #[derivative(PartialEq = "ignore")]
    pub predicate_proof: <C::ProofCheckNIZK as NIZK>::Proof,
    #[derivative(PartialEq = "ignore")]
    pub predicate_comm: <C::PredVkComm as CommitmentScheme>::Output,
    #[derivative(PartialEq = "ignore")]
    pub local_data_comm: <C::LocalDataComm as CommitmentScheme>::Output,
}

// impl<C: PlainDPCComponents> DPCStuff<C> {
//     pub fn write<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         // writer.write_all(self.digest.)
//         self.digest.write(writer).unwrap();
//         self.core_proof.write(writer).unwrap();
//         self.predicate_proof.write(writer).unwrap();
//         self.predicate_comm.write(writer).unwrap();
//         self.local_data_comm.write(writer).unwrap();
//         Ok(())
//     }
// }

impl<C: PlainDPCComponents> DPCTransaction<C> {
    pub fn new(
        old_serial_numbers: Vec<<Self as Transaction>::SerialNumber>,
        new_commitments: Vec<<Self as Transaction>::Commitment>,
        memorandum: <Self as Transaction>::Memorandum,
        digest: C::D,
        core_proof: <C::MainNIZK as NIZK>::Proof,
        predicate_proof: <C::ProofCheckNIZK as NIZK>::Proof,
        predicate_comm: <C::PredVkComm as CommitmentScheme>::Output,
        local_data_comm: <C::LocalDataComm as CommitmentScheme>::Output,
    ) -> Self {
        let stuff = DPCStuff {
            digest,
            core_proof,
            predicate_proof,
            predicate_comm,
            local_data_comm,
        };
        DPCTransaction {
            old_serial_numbers,
            new_commitments,
            memorandum,
            stuff,
        }
    }
}

impl<C: PlainDPCComponents> Transaction for DPCTransaction<C> {
    type Stuff = DPCStuff<C>;
    type SerialNumber = <C::P as PRF>::Output;
    type Commitment = <C::RecC as CommitmentScheme>::Output;
    type Memorandum = [u8; 32];

    fn old_serial_numbers(&self) -> &[Self::SerialNumber] {
        self.old_serial_numbers.as_slice()
    }

    fn new_commitments(&self) -> &[Self::Commitment] {
        self.new_commitments.as_slice()
    }

    fn memorandum(&self) -> &Self::Memorandum {
        &self.memorandum
    }

    fn stuff(&self) -> &Self::Stuff {
        &self.stuff
    }
}

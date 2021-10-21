use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProofOfComputationData {
    id: String,
    partial_proof_of_computation: f64,
}

impl ProofOfComputationData {
    pub fn new(id: String, partial_proof_of_computation: f64) -> Self {
        Self {
            id,
            partial_proof_of_computation,
        }
    }

    pub fn partial_proof_of_computation(&self) -> &f64 {
        &self.partial_proof_of_computation
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseGetProofOfComputation {
    data: Vec<ProofOfComputationData>,
    host_id: String,
    proof_of_computation: f64,
    public_key: String,
}

impl ResponseGetProofOfComputation {
    pub fn new(
        data: Vec<ProofOfComputationData>,
        host_id: String,
        proof_of_computation: f64,
        public_key: String,
    ) -> Self {
        Self {
            data,
            host_id,
            proof_of_computation,
            public_key,
        }
    }
}

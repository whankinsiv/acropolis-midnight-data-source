use sidechain_domain::McBlockHash;

pub mod candidates_data_source;
pub mod cnight_observation_data_source;
pub mod mc_hash_data_source;
pub mod sidechain_rpc_data_source;

#[derive(thiserror::Error, Debug)]
pub enum AcropolisDataSourceError {
    #[error("Error querying gRPC `{0}`")]
    GRPCQueryError(tonic::Status),
    #[error("missing reference for block hash `{0}` in acropolis")]
    MissingBlockReference(McBlockHash),
}

mod data_sources;
mod grpc;
pub use data_sources::{
    candidates_data_source::CandidatesDataSourceGrpcImpl,
    cnight_observation_data_source::MidnightCNightObservationGrpcImpl,
    mc_hash_data_source::McHashDataSourceGrpcImpl,
    sidechain_rpc_data_source::SidechainRpcDataSourceGrpcImpl,
};
pub use grpc::client::MidnightGrpcClient;
mod midnight_state {
    tonic::include_proto!("midnight_state");
}

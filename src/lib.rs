mod data_sources;
mod grpc;
pub use data_sources::{
    candidates_data_source::AuthoritySelectionDataSourceGrpcImpl,
    cnight_observation_data_source::MidnightCNightObservationGrpcImpl,
    federated_authority_data_source::FederatedAuthorityObservationGrpcImpl,
    mc_hash_data_source::McHashDataSourceGrpcImpl,
    sidechain_rpc_data_source::SidechainRpcDataSourceGrpcImpl,
};
pub use grpc::client::MidnightGrpcClient;
mod midnight_state {
    tonic::include_proto!("midnight_state");
}

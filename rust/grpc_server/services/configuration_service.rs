use std::sync::Arc;

use common_proto::StatusResponseProto;
use configuration_proto::{
    AcceptRequestToJoinGroupProto, AccountConfigurationProto, AccountCreationRequestProto,
    AccountDeletionRequestProto, DenyRequestToJoinGroupProto, ListConfigurationEventsRequestProto,
    ListConfigurationEventsResponseProto, ListGroupsRequestProto, ListGroupsResponseProto,
    MemberDeletionRequestProto, RequestToJoinGroupProto,
};
use configuration_service_proto::configuration_service::configuration_server::Configuration;
use direct_messenger::MessengerRoute;

use crate::messenger::DirectMessenger;

#[derive(Clone, Debug)]
pub struct Dummy {}

#[derive(Clone, Debug)]
pub(crate) struct ConfigurationService {
    pub state: Arc<DirectMessenger>,
    pub dummy: Arc<Dummy>,
}

#[tonic::async_trait]
impl Configuration for ConfigurationService {
    /// Create a new account
    async fn create_account(
        &self,
        request: tonic::Request<AccountCreationRequestProto>,
    ) -> std::result::Result<tonic::Response<AccountConfigurationProto>, tonic::Status> {
        let req = request.into_inner();

        let resp = self.state.route(&req).await;

        Ok(tonic::Response::new(resp))
    }

    /// Delete an account
    async fn delete_account(
        &self,
        request: tonic::Request<AccountDeletionRequestProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// Request to join a group
    async fn request_to_join_group(
        &self,
        request: tonic::Request<RequestToJoinGroupProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// List all configuration events
    async fn list_configuration_events(
        &self,
        request: tonic::Request<ListConfigurationEventsRequestProto>,
    ) -> std::result::Result<tonic::Response<ListConfigurationEventsResponseProto>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// Accept a request to join a group
    async fn accept_request_to_join_group(
        &self,
        request: tonic::Request<AcceptRequestToJoinGroupProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// Deny a request to join a group
    async fn deny_request_to_join_group(
        &self,
        request: tonic::Request<DenyRequestToJoinGroupProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// List all groups for a user
    async fn list_groups(
        &self,
        request: tonic::Request<ListGroupsRequestProto>,
    ) -> std::result::Result<tonic::Response<ListGroupsResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// Delete a member from a group
    async fn delete_member(
        &self,
        request: tonic::Request<MemberDeletionRequestProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}

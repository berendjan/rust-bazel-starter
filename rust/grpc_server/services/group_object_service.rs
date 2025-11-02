use std::sync::Arc;

use common_proto::StatusResponseProto;
use group_object_proto::{
    GetGroupObjectVersionRequestProto, GetGroupObjectVersionResponseProto,
    GroupObjectUpdatesRequestProto, GroupObjectUpdatesResponseProto,
    ListGroupObjectEventsRequestProto, ListGroupObjectEventsResponseProto,
    UpsertGroupObjectRequestProto,
};
use group_object_service_proto::group_object_service::group_object_server::GroupObject;

use crate::messenger::DirectMessenger;

pub(crate) struct GroupObjectService {
    pub state: Arc<DirectMessenger>,
}

#[tonic::async_trait]
impl GroupObject for GroupObjectService {
    /// Get versions of all group objects in a group
    async fn get_group_object_version(
        &self,
        request: tonic::Request<GetGroupObjectVersionRequestProto>,
    ) -> std::result::Result<tonic::Response<GetGroupObjectVersionResponseProto>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// Get missing group objects in a group
    async fn group_object_updates(
        &self,
        request: tonic::Request<GroupObjectUpdatesRequestProto>,
    ) -> std::result::Result<tonic::Response<GroupObjectUpdatesResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// List all group object events for a group
    async fn list_group_object_events(
        &self,
        request: tonic::Request<ListGroupObjectEventsRequestProto>,
    ) -> std::result::Result<tonic::Response<ListGroupObjectEventsResponseProto>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    /// List all group object events for a group
    async fn upsert_group_object(
        &self,
        request: tonic::Request<UpsertGroupObjectRequestProto>,
    ) -> std::result::Result<tonic::Response<StatusResponseProto>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}

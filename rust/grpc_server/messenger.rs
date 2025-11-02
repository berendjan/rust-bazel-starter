use direct_messenger::direct_messenger;
use protobuf::proto;
use uuid::Uuid;

use common_proto::ConfigurationIdProto;
use configuration_proto::{AccountConfigurationProto, AccountCreationRequestProto};

#[derive(Clone, Debug)]
pub struct ConfigurationRepository;

impl ConfigurationRepository {
    async fn handle(
        &self,
        message: &AccountCreationRequestProto,
        _router: &DirectMessenger,
    ) -> AccountConfigurationProto {
        // For demonstration purposes, we'll return a dummy response.

        println!("Creating account with name: {}", message.name());

        return proto!(AccountConfigurationProto {
            account_id: proto!(ConfigurationIdProto {
                id: Uuid::new_v4().as_bytes(),
                r#type: 1
            }),
        });
    }
}

#[derive(Clone, Debug)]
struct GroupObjectRepository;

direct_messenger!(
    derive: [Clone, Debug],
    handlers: [
        configuration_repository: ConfigurationRepository,
        dummy: std::sync::Arc<crate::services::configuration_service::Dummy>,
        // group_object_repository: GroupObjectRepository,
        // group_object_service: crate::services::group_object_service::GroupObjectService,
    ]
    routes: [
        // ConfigurationService routes
        crate::services::configuration_service::ConfigurationService, AccountCreationRequestProto, AccountConfigurationProto: [configuration_repository],
        // crate::services::configuration_service::ConfigurationService, configuration::AccountCreationRequestProto: [configuration_service],
        // tonic::Request<configuration::GetAccountRequestProto>, configuration::GetAccountRequestProto, configuration::GetAccountResponseProto: [configuration_repository],

        // GroupObjectService routes
        // tonic::Request<group_object::GetGroupObjectVersionRequestProto>, group_object::GetGroupObjectVersionRequestProto, group_object::GetGroupObjectVersionResponseProto: [group_object_repository],
        // tonic::Request<group_object::GroupObjectUpdatesRequestProto>, group_object::GroupObjectUpdatesRequestProto, group_object::GroupObjectUpdatesResponseProto: [group_object_repository],
        // tonic::Request<group_object::ListGroupObjectEventsRequestProto>, group_object::ListGroupObjectEventsRequestProto, group_object::ListGroupObjectEventsResponseProto: [group_object_repository],
        // tonic::Request<group_object::UpsertGroupObjectRequestProto>, group_object::UpsertGroupObjectRequestProto, common::StatusResponseProto: [group_object_repository]
    ]
);

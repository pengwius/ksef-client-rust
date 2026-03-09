use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions;
use crate::client::permissions::get_authorizations_permissions::{
    GetAuthorizationsPermissionsRequest, GetAuthorizationsPermissionsResponse,
};
use crate::client::permissions::get_entities_permissions::{
    GetEntitiesPermissionsRequest, GetEntitiesPermissionsResponse,
};
use crate::client::permissions::get_entity_roles::GetEntityRolesResponse;
use crate::client::permissions::get_eu_entities_permissions::{
    GetEuEntitiesPermissionsRequest, GetEuEntitiesPermissionsResponse,
};
use crate::client::permissions::get_operation_status::OperationStatusResponse;
use crate::client::permissions::get_personal_permissions::{
    GetPersonalPermissionsRequest, GetPersonalPermissionsResponse,
};
use crate::client::permissions::get_persons_permissions::{
    GetPersonsPermissionsResponse, PersonsPermissionsRequest,
};
use crate::client::permissions::get_subordinate_entities_roles::{
    GetSubordinateEntitiesRolesRequest, GetSubordinateEntitiesRolesResponse,
};
use crate::client::permissions::get_subunits_permissions::{
    GetSubunitsPermissionsRequest, GetSubunitsPermissionsResponse,
};
use crate::client::permissions::grant_authorization_permissions::GrantAuthorizationPermissionsRequest;
use crate::client::permissions::grant_entity_permissions::GrantEntityPermissionsRequest;
use crate::client::permissions::grant_eu_entity_permissions::{
    GrantEuEntityPermissionsRequest, GrantEuEntityPermissionsResponse,
};
use crate::client::permissions::grant_eu_entity_representative_permissions::{
    GrantEuEntityRepresentativePermissionsRequest, GrantEuEntityRepresentativePermissionsResponse,
};
use crate::client::permissions::grant_indirect_entity_permissions::{
    GrantIndirectEntityPermissionsRequest, GrantIndirectEntityPermissionsResponse,
};
use crate::client::permissions::grant_person_permissions::GrantPersonPermissionsRequest;
use crate::client::permissions::grant_subunit_permissions::GrantSubunitPermissionsRequest;
use async_trait::async_trait;

#[async_trait]
pub trait KsefPermissions {
    async fn grant_person_permissions(
        &self,
        request: GrantPersonPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn grant_entity_permissions(
        &self,
        request: GrantEntityPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn grant_authorization_permissions(
        &self,
        request: GrantAuthorizationPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn get_authorizations_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: GetAuthorizationsPermissionsRequest,
    ) -> Result<GetAuthorizationsPermissionsResponse, KsefError>;

    async fn get_entities_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetEntitiesPermissionsRequest>,
    ) -> Result<GetEntitiesPermissionsResponse, KsefError>;

    async fn get_eu_entities_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetEuEntitiesPermissionsRequest>,
    ) -> Result<GetEuEntitiesPermissionsResponse, KsefError>;

    async fn get_entity_roles(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GetEntityRolesResponse, KsefError>;

    async fn get_subordinate_entities_roles(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetSubordinateEntitiesRolesRequest>,
    ) -> Result<GetSubordinateEntitiesRolesResponse, KsefError>;

    async fn grant_indirect_entity_permissions(
        &self,
        request: GrantIndirectEntityPermissionsRequest,
    ) -> Result<GrantIndirectEntityPermissionsResponse, KsefError>;

    async fn grant_subunit_permissions(
        &self,
        request: GrantSubunitPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn grant_eu_entity_permissions(
        &self,
        request: GrantEuEntityPermissionsRequest,
    ) -> Result<GrantEuEntityPermissionsResponse, KsefError>;

    async fn grant_eu_entity_representative_permissions(
        &self,
        request: GrantEuEntityRepresentativePermissionsRequest,
    ) -> Result<GrantEuEntityRepresentativePermissionsResponse, KsefError>;

    async fn revoke_authorizations_permission(
        &self,
        permission_id: &str,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn revoke_common_permission(
        &self,
        permission_id: &str,
    ) -> Result<OperationStatusResponse, KsefError>;

    async fn get_common_permissions(&self) -> Result<serde_json::Value, KsefError>;

    async fn get_personal_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<GetPersonalPermissionsRequest>,
    ) -> Result<GetPersonalPermissionsResponse, KsefError>;

    async fn get_persons_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<PersonsPermissionsRequest>,
    ) -> Result<GetPersonsPermissionsResponse, KsefError>;

    async fn get_subunits_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<GetSubunitsPermissionsRequest>,
    ) -> Result<GetSubunitsPermissionsResponse, KsefError>;
}

#[async_trait]
impl KsefPermissions for KsefClient {
    async fn grant_person_permissions(
        &self,
        request: GrantPersonPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::grant_person_permissions::grant_person_permissions(self, request).await
    }

    async fn grant_entity_permissions(
        &self,
        request: GrantEntityPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::grant_entity_permissions::grant_entity_permissions(self, request).await
    }

    async fn grant_authorization_permissions(
        &self,
        request: GrantAuthorizationPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::grant_authorization_permissions::grant_authorization_permissions(self, request)
            .await
    }

    async fn get_authorizations_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: GetAuthorizationsPermissionsRequest,
    ) -> Result<GetAuthorizationsPermissionsResponse, KsefError> {
        permissions::get_authorizations_permissions::get_authorizations_permissions(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    async fn get_entities_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetEntitiesPermissionsRequest>,
    ) -> Result<GetEntitiesPermissionsResponse, KsefError> {
        permissions::get_entities_permissions::get_entities_permissions(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    async fn get_eu_entities_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetEuEntitiesPermissionsRequest>,
    ) -> Result<GetEuEntitiesPermissionsResponse, KsefError> {
        permissions::get_eu_entities_permissions::get_eu_entities_permissions(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    async fn get_entity_roles(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GetEntityRolesResponse, KsefError> {
        permissions::get_entity_roles::get_entity_roles(self, page_offset, page_size).await
    }

    async fn get_subordinate_entities_roles(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<GetSubordinateEntitiesRolesRequest>,
    ) -> Result<GetSubordinateEntitiesRolesResponse, KsefError> {
        permissions::get_subordinate_entities_roles::get_subordinate_entities_roles(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    async fn grant_indirect_entity_permissions(
        &self,
        request: GrantIndirectEntityPermissionsRequest,
    ) -> Result<GrantIndirectEntityPermissionsResponse, KsefError> {
        permissions::grant_indirect_entity_permissions::grant_indirect_entity_permissions(
            self, request,
        )
        .await
    }

    async fn grant_subunit_permissions(
        &self,
        request: GrantSubunitPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::grant_subunit_permissions::grant_subunit_permissions(self, request).await
    }

    async fn grant_eu_entity_permissions(
        &self,
        request: GrantEuEntityPermissionsRequest,
    ) -> Result<GrantEuEntityPermissionsResponse, KsefError> {
        permissions::grant_eu_entity_permissions::grant_eu_entity_permissions(self, request).await
    }

    async fn grant_eu_entity_representative_permissions(
        &self,
        request: GrantEuEntityRepresentativePermissionsRequest,
    ) -> Result<GrantEuEntityRepresentativePermissionsResponse, KsefError> {
        permissions::grant_eu_entity_representative_permissions::grant_eu_entity_representative_permissions(
            self, request,
        ).await
    }

    async fn revoke_authorizations_permission(
        &self,
        permission_id: &str,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::revoke_authorizations_permission::revoke_authorizations_permission(
            self,
            permission_id,
        )
        .await
    }

    async fn revoke_common_permission(
        &self,
        permission_id: &str,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::revoke_common_permission::revoke_common_permission(self, permission_id).await
    }

    async fn get_common_permissions(&self) -> Result<serde_json::Value, KsefError> {
        permissions::revoke_common_permission::get_common_permissions(self).await
    }

    async fn get_personal_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<GetPersonalPermissionsRequest>,
    ) -> Result<GetPersonalPermissionsResponse, KsefError> {
        permissions::get_personal_permissions::get_personal_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }

    async fn get_persons_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<PersonsPermissionsRequest>,
    ) -> Result<GetPersonsPermissionsResponse, KsefError> {
        permissions::get_persons_permissions::get_persons_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }

    async fn get_subunits_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<GetSubunitsPermissionsRequest>,
    ) -> Result<GetSubunitsPermissionsResponse, KsefError> {
        permissions::get_subunits_permissions::get_subunits_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }
}

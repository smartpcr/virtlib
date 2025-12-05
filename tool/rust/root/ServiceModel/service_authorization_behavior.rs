// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceAuthorizationBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceAuthorizationBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// A value that controls whether the service attempts to impersonate using the credentials provided by the incoming message. 
    #[serde(rename = "ImpersonateCallerForAllOperations")]
    pub impersonate_caller_for_all_operations: Option<bool>,

/// A value that controls whether the service attempts to impersonate using the credentials provided by the incoming message while serializing the body of the response message. 
    #[serde(rename = "ImpersonateOnSerializingReply")]
    pub impersonate_on_serializing_reply: Option<bool>,

/// The principal used to carry out operations on the server. 
    #[serde(rename = "PrincipalPermissionMode")]
    pub principal_permission_mode: Option<String>,

/// The name of the ASP .Net role provider.
    #[serde(rename = "RoleProvider")]
    pub role_provider: Option<String>,

/// The authorization manager used for custom authorization.
    #[serde(rename = "ServiceAuthorizationManager")]
    pub service_authorization_manager: Option<String>,
}

impl ServiceAuthorizationBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            impersonate_caller_for_all_operations: None,
            impersonate_on_serializing_reply: None,
            principal_permission_mode: None,
            role_provider: None,
            service_authorization_manager: None,
        }
    }


    /// Sets the value of ImpersonateCallerForAllOperations
    pub fn set_impersonate_caller_for_all_operations(&mut self, value: bool) {
        self.impersonate_caller_for_all_operations = Some(value);
    }

    /// Gets the value of ImpersonateCallerForAllOperations
    pub fn get_impersonate_caller_for_all_operations(&self) -> Option<&bool> {
        self.impersonate_caller_for_all_operations.as_ref()
    }

    /// Sets the value of ImpersonateOnSerializingReply
    pub fn set_impersonate_on_serializing_reply(&mut self, value: bool) {
        self.impersonate_on_serializing_reply = Some(value);
    }

    /// Gets the value of ImpersonateOnSerializingReply
    pub fn get_impersonate_on_serializing_reply(&self) -> Option<&bool> {
        self.impersonate_on_serializing_reply.as_ref()
    }

    /// Sets the value of PrincipalPermissionMode
    pub fn set_principal_permission_mode(&mut self, value: String) {
        self.principal_permission_mode = Some(value);
    }

    /// Gets the value of PrincipalPermissionMode
    pub fn get_principal_permission_mode(&self) -> Option<&String> {
        self.principal_permission_mode.as_ref()
    }

    /// Sets the value of RoleProvider
    pub fn set_role_provider(&mut self, value: String) {
        self.role_provider = Some(value);
    }

    /// Gets the value of RoleProvider
    pub fn get_role_provider(&self) -> Option<&String> {
        self.role_provider.as_ref()
    }

    /// Sets the value of ServiceAuthorizationManager
    pub fn set_service_authorization_manager(&mut self, value: String) {
        self.service_authorization_manager = Some(value);
    }

    /// Gets the value of ServiceAuthorizationManager
    pub fn get_service_authorization_manager(&self) -> Option<&String> {
        self.service_authorization_manager.as_ref()
    }
}


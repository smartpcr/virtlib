// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceSecurityAuditBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceSecurityAuditBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// The location of the audit log. 
    #[serde(rename = "AuditLogLocation")]
    pub audit_log_location: Option<String>,

/// The type of message authentication level that is used to log audit events.
    #[serde(rename = "MessageAuthenticationAuditLevel")]
    pub message_authentication_audit_level: Option<String>,

/// The types of authorization events that are recorded in the audit log. 
    #[serde(rename = "ServiceAuthorizationAuditLevel")]
    pub service_authorization_audit_level: Option<String>,

/// A boolean value that specifies the behavior for suppressing failures of writing to the audit log.
    #[serde(rename = "SuppressAuditFailure")]
    pub suppress_audit_failure: Option<bool>,
}

impl ServiceSecurityAuditBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            audit_log_location: None,
            message_authentication_audit_level: None,
            service_authorization_audit_level: None,
            suppress_audit_failure: None,
        }
    }


    /// Sets the value of AuditLogLocation
    pub fn set_audit_log_location(&mut self, value: String) {
        self.audit_log_location = Some(value);
    }

    /// Gets the value of AuditLogLocation
    pub fn get_audit_log_location(&self) -> Option<&String> {
        self.audit_log_location.as_ref()
    }

    /// Sets the value of MessageAuthenticationAuditLevel
    pub fn set_message_authentication_audit_level(&mut self, value: String) {
        self.message_authentication_audit_level = Some(value);
    }

    /// Gets the value of MessageAuthenticationAuditLevel
    pub fn get_message_authentication_audit_level(&self) -> Option<&String> {
        self.message_authentication_audit_level.as_ref()
    }

    /// Sets the value of ServiceAuthorizationAuditLevel
    pub fn set_service_authorization_audit_level(&mut self, value: String) {
        self.service_authorization_audit_level = Some(value);
    }

    /// Gets the value of ServiceAuthorizationAuditLevel
    pub fn get_service_authorization_audit_level(&self) -> Option<&String> {
        self.service_authorization_audit_level.as_ref()
    }

    /// Sets the value of SuppressAuditFailure
    pub fn set_suppress_audit_failure(&mut self, value: bool) {
        self.suppress_audit_failure = Some(value);
    }

    /// Gets the value of SuppressAuditFailure
    pub fn get_suppress_audit_failure(&self) -> Option<&bool> {
        self.suppress_audit_failure.as_ref()
    }
}


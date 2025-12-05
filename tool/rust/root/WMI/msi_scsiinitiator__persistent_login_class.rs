// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_PersistentLoginClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_PersistentLoginClass {

/// 
    #[serde(rename = "InitiatorInstance")]
    pub initiator_instance: Option<String>,

/// 
    #[serde(rename = "InitiatorPortNumber")]
    pub initiator_port_number: Option<u32>,

/// 
    #[serde(rename = "IsInformationalSession")]
    pub is_informational_session: Option<bool>,

/// 
    #[serde(rename = "LoginOptions")]
    pub login_options: Option<MSiSCSIInitiator_TargetLoginOptions>,

/// 
    #[serde(rename = "Mappings")]
    pub mappings: Option<MSiSCSIInitiator_TargetMappings>,

/// 
    #[serde(rename = "SecurityFlags")]
    pub security_flags: Option<u64>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,

/// 
    #[serde(rename = "TargetPortal")]
    pub target_portal: Option<MSiSCSIInitiator_Portal>,
}

impl MSiSCSIInitiator_PersistentLoginClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_instance: None,
            initiator_port_number: None,
            is_informational_session: None,
            login_options: None,
            mappings: None,
            security_flags: None,
            target_name: None,
            target_portal: None,
        }
    }


    /// Sets the value of InitiatorInstance
    pub fn set_initiator_instance(&mut self, value: String) {
        self.initiator_instance = Some(value);
    }

    /// Gets the value of InitiatorInstance
    pub fn get_initiator_instance(&self) -> Option<&String> {
        self.initiator_instance.as_ref()
    }

    /// Sets the value of InitiatorPortNumber
    pub fn set_initiator_port_number(&mut self, value: u32) {
        self.initiator_port_number = Some(value);
    }

    /// Gets the value of InitiatorPortNumber
    pub fn get_initiator_port_number(&self) -> Option<&u32> {
        self.initiator_port_number.as_ref()
    }

    /// Sets the value of IsInformationalSession
    pub fn set_is_informational_session(&mut self, value: bool) {
        self.is_informational_session = Some(value);
    }

    /// Gets the value of IsInformationalSession
    pub fn get_is_informational_session(&self) -> Option<&bool> {
        self.is_informational_session.as_ref()
    }

    /// Sets the value of LoginOptions
    pub fn set_login_options(&mut self, value: MSiSCSIInitiator_TargetLoginOptions) {
        self.login_options = Some(value);
    }

    /// Gets the value of LoginOptions
    pub fn get_login_options(&self) -> Option<&MSiSCSIInitiator_TargetLoginOptions> {
        self.login_options.as_ref()
    }

    /// Sets the value of Mappings
    pub fn set_mappings(&mut self, value: MSiSCSIInitiator_TargetMappings) {
        self.mappings = Some(value);
    }

    /// Gets the value of Mappings
    pub fn get_mappings(&self) -> Option<&MSiSCSIInitiator_TargetMappings> {
        self.mappings.as_ref()
    }

    /// Sets the value of SecurityFlags
    pub fn set_security_flags(&mut self, value: u64) {
        self.security_flags = Some(value);
    }

    /// Gets the value of SecurityFlags
    pub fn get_security_flags(&self) -> Option<&u64> {
        self.security_flags.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of TargetPortal
    pub fn set_target_portal(&mut self, value: MSiSCSIInitiator_Portal) {
        self.target_portal = Some(value);
    }

    /// Gets the value of TargetPortal
    pub fn get_target_portal(&self) -> Option<&MSiSCSIInitiator_Portal> {
        self.target_portal.as_ref()
    }
}


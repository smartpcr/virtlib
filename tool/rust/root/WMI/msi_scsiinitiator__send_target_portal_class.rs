// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_SendTargetPortalClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_SendTargetPortalClass {

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "InitiatorPortNumber")]
    pub initiator_port_number: Option<u32>,

/// 
    #[serde(rename = "LoginOptions")]
    pub login_options: Option<MSiSCSIInitiator_TargetLoginOptions>,

/// 
    #[serde(rename = "PortalAddress")]
    pub portal_address: Option<String>,

/// 
    #[serde(rename = "PortalIdentifierString")]
    pub portal_identifier_string: Option<String>,

/// 
    #[serde(rename = "PortalPort")]
    pub portal_port: Option<u16>,

/// 
    #[serde(rename = "PortalSymbolicName")]
    pub portal_symbolic_name: Option<String>,

/// 
    #[serde(rename = "SecurityFlags")]
    pub security_flags: Option<u64>,
}

impl MSiSCSIInitiator_SendTargetPortalClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_name: None,
            initiator_port_number: None,
            login_options: None,
            portal_address: None,
            portal_identifier_string: None,
            portal_port: None,
            portal_symbolic_name: None,
            security_flags: None,
        }
    }


    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of InitiatorPortNumber
    pub fn set_initiator_port_number(&mut self, value: u32) {
        self.initiator_port_number = Some(value);
    }

    /// Gets the value of InitiatorPortNumber
    pub fn get_initiator_port_number(&self) -> Option<&u32> {
        self.initiator_port_number.as_ref()
    }

    /// Sets the value of LoginOptions
    pub fn set_login_options(&mut self, value: MSiSCSIInitiator_TargetLoginOptions) {
        self.login_options = Some(value);
    }

    /// Gets the value of LoginOptions
    pub fn get_login_options(&self) -> Option<&MSiSCSIInitiator_TargetLoginOptions> {
        self.login_options.as_ref()
    }

    /// Sets the value of PortalAddress
    pub fn set_portal_address(&mut self, value: String) {
        self.portal_address = Some(value);
    }

    /// Gets the value of PortalAddress
    pub fn get_portal_address(&self) -> Option<&String> {
        self.portal_address.as_ref()
    }

    /// Sets the value of PortalIdentifierString
    pub fn set_portal_identifier_string(&mut self, value: String) {
        self.portal_identifier_string = Some(value);
    }

    /// Gets the value of PortalIdentifierString
    pub fn get_portal_identifier_string(&self) -> Option<&String> {
        self.portal_identifier_string.as_ref()
    }

    /// Sets the value of PortalPort
    pub fn set_portal_port(&mut self, value: u16) {
        self.portal_port = Some(value);
    }

    /// Gets the value of PortalPort
    pub fn get_portal_port(&self) -> Option<&u16> {
        self.portal_port.as_ref()
    }

    /// Sets the value of PortalSymbolicName
    pub fn set_portal_symbolic_name(&mut self, value: String) {
        self.portal_symbolic_name = Some(value);
    }

    /// Gets the value of PortalSymbolicName
    pub fn get_portal_symbolic_name(&self) -> Option<&String> {
        self.portal_symbolic_name.as_ref()
    }

    /// Sets the value of SecurityFlags
    pub fn set_security_flags(&mut self, value: u64) {
        self.security_flags = Some(value);
    }

    /// Gets the value of SecurityFlags
    pub fn get_security_flags(&self) -> Option<&u64> {
        self.security_flags.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn refresh(&self) -> Result<(), WmiError> {
        self.invoke_method("Refresh", &[])

    }

}


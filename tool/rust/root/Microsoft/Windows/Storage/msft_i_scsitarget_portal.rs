// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSITargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSITargetPortal {

/// 
    #[serde(rename = "InitiatorInstanceName")]
    pub initiator_instance_name: Option<String>,

/// 
    #[serde(rename = "InitiatorPortalAddress")]
    pub initiator_portal_address: Option<String>,

/// 
    #[serde(rename = "IsDataDigest")]
    pub is_data_digest: Option<bool>,

/// 
    #[serde(rename = "IsHeaderDigest")]
    pub is_header_digest: Option<bool>,

/// 
    #[serde(rename = "TargetPortalAddress")]
    pub target_portal_address: Option<String>,

/// 
    #[serde(rename = "TargetPortalPortNumber")]
    pub target_portal_port_number: Option<u16>,
}

impl MSFT_iSCSITargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_instance_name: None,
            initiator_portal_address: None,
            is_data_digest: None,
            is_header_digest: None,
            target_portal_address: None,
            target_portal_port_number: None,
        }
    }


    /// Sets the value of InitiatorInstanceName
    pub fn set_initiator_instance_name(&mut self, value: String) {
        self.initiator_instance_name = Some(value);
    }

    /// Gets the value of InitiatorInstanceName
    pub fn get_initiator_instance_name(&self) -> Option<&String> {
        self.initiator_instance_name.as_ref()
    }

    /// Sets the value of InitiatorPortalAddress
    pub fn set_initiator_portal_address(&mut self, value: String) {
        self.initiator_portal_address = Some(value);
    }

    /// Gets the value of InitiatorPortalAddress
    pub fn get_initiator_portal_address(&self) -> Option<&String> {
        self.initiator_portal_address.as_ref()
    }

    /// Sets the value of IsDataDigest
    pub fn set_is_data_digest(&mut self, value: bool) {
        self.is_data_digest = Some(value);
    }

    /// Gets the value of IsDataDigest
    pub fn get_is_data_digest(&self) -> Option<&bool> {
        self.is_data_digest.as_ref()
    }

    /// Sets the value of IsHeaderDigest
    pub fn set_is_header_digest(&mut self, value: bool) {
        self.is_header_digest = Some(value);
    }

    /// Gets the value of IsHeaderDigest
    pub fn get_is_header_digest(&self) -> Option<&bool> {
        self.is_header_digest.as_ref()
    }

    /// Sets the value of TargetPortalAddress
    pub fn set_target_portal_address(&mut self, value: String) {
        self.target_portal_address = Some(value);
    }

    /// Gets the value of TargetPortalAddress
    pub fn get_target_portal_address(&self) -> Option<&String> {
        self.target_portal_address.as_ref()
    }

    /// Sets the value of TargetPortalPortNumber
    pub fn set_target_portal_port_number(&mut self, value: u16) {
        self.target_portal_port_number = Some(value);
    }

    /// Gets the value of TargetPortalPortNumber
    pub fn get_target_portal_port_number(&self) -> Option<&u16> {
        self.target_portal_port_number.as_ref()
    }

/// 

    /// * `authentication_type` -  (String)
    /// * `chap_secret` -  (String)
    /// * `chap_username` -  (String)
    /// * `initiator_instance_name` -  (String)
    /// * `initiator_portal_address` -  (String)
    /// * `is_data_digest` -  (bool)
    /// * `is_header_digest` -  (bool)
    /// * `target_portal_address` -  (String)
    /// * `target_portal_port_number` -  (u16)

    /// * `created_target_portal` -  (MSFT_iSCSITargetPortal)
    /// * `return_value` -  (u32)
    pub fn new(&self, target_portal_address: &String, target_portal_port_number: u16, initiator_instance_name: &String, initiator_portal_address: &String, authentication_type: &String, chap_username: &String, chap_secret: &String, is_header_digest: bool, is_data_digest: bool, created_target_portal: &mut MSFT_iSCSITargetPortal) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortalAddress".to_string(), value: target_portal_address.into() });
        args.push(MethodParameter { name: "TargetPortalPortNumber".to_string(), value: target_portal_port_number.into() });
        args.push(MethodParameter { name: "InitiatorInstanceName".to_string(), value: initiator_instance_name.into() });
        args.push(MethodParameter { name: "InitiatorPortalAddress".to_string(), value: initiator_portal_address.into() });
        args.push(MethodParameter { name: "AuthenticationType".to_string(), value: authentication_type.into() });
        args.push(MethodParameter { name: "ChapUsername".to_string(), value: chap_username.into() });
        args.push(MethodParameter { name: "ChapSecret".to_string(), value: chap_secret.into() });
        args.push(MethodParameter { name: "IsHeaderDigest".to_string(), value: is_header_digest.into() });
        args.push(MethodParameter { name: "IsDataDigest".to_string(), value: is_data_digest.into() });

        let result = self.invoke_method("New", &args)?;
        let created_target_portal = result.get_value("CreatedTargetPortal")?;
        Ok(result.return_value)

    }


/// 

    /// * `initiator_instance_name` -  (String)
    /// * `initiator_portal_address` -  (String)
    /// * `target_portal_address` -  (String)
    /// * `target_portal_port_number` -  (u16)

    /// * `return_value` -  (u32)
    pub fn remove(&self, initiator_instance_name: &String, initiator_portal_address: &String, target_portal_port_number: u16, target_portal_address: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorInstanceName".to_string(), value: initiator_instance_name.into() });
        args.push(MethodParameter { name: "InitiatorPortalAddress".to_string(), value: initiator_portal_address.into() });
        args.push(MethodParameter { name: "TargetPortalPortNumber".to_string(), value: target_portal_port_number.into() });
        args.push(MethodParameter { name: "TargetPortalAddress".to_string(), value: target_portal_address.into() });
        self.invoke_method("Remove", &args)

    }


/// 

    /// * `initiator_instance_name` -  (String)
    /// * `initiator_portal_address` -  (String)
    /// * `target_portal_address` -  (String)
    /// * `target_portal_port_number` -  (u16)

    /// * `return_value` -  (u32)
    pub fn update(&self, initiator_instance_name: &String, initiator_portal_address: &String, target_portal_address: &String, target_portal_port_number: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorInstanceName".to_string(), value: initiator_instance_name.into() });
        args.push(MethodParameter { name: "InitiatorPortalAddress".to_string(), value: initiator_portal_address.into() });
        args.push(MethodParameter { name: "TargetPortalAddress".to_string(), value: target_portal_address.into() });
        args.push(MethodParameter { name: "TargetPortalPortNumber".to_string(), value: target_portal_port_number.into() });
        self.invoke_method("Update", &args)

    }

}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSITarget struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSITarget {

/// 
    #[serde(rename = "IsConnected")]
    pub is_connected: Option<bool>,

/// 
    #[serde(rename = "NodeAddress")]
    pub node_address: Option<String>,
}

impl MSFT_iSCSITarget {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_connected: None,
            node_address: None,
        }
    }


    /// Sets the value of IsConnected
    pub fn set_is_connected(&mut self, value: bool) {
        self.is_connected = Some(value);
    }

    /// Gets the value of IsConnected
    pub fn get_is_connected(&self) -> Option<&bool> {
        self.is_connected.as_ref()
    }

    /// Sets the value of NodeAddress
    pub fn set_node_address(&mut self, value: String) {
        self.node_address = Some(value);
    }

    /// Gets the value of NodeAddress
    pub fn get_node_address(&self) -> Option<&String> {
        self.node_address.as_ref()
    }

/// 

    /// * `session_identifier` -  (String)

    /// * `return_value` -  (u32)
    pub fn disconnect(&self, session_identifier: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SessionIdentifier".to_string(), value: session_identifier.into() });
        self.invoke_method("Disconnect", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn update(&self) -> Result<(), WmiError> {
        self.invoke_method("Update", &[])

    }


/// 

    /// * `authentication_type` -  (String)
    /// * `chap_secret` -  (String)
    /// * `chap_username` -  (String)
    /// * `initiator_instance_name` -  (String)
    /// * `initiator_portal_address` -  (String)
    /// * `is_data_digest` -  (bool)
    /// * `is_header_digest` -  (bool)
    /// * `is_multipath_enabled` -  (bool)
    /// * `is_persistent` -  (bool)
    /// * `node_address` -  (String)
    /// * `report_to_pn_p` -  (bool)
    /// * `target_portal_address` -  (String)
    /// * `target_portal_port_number` -  (u16)

    /// * `createdi_scsisession` -  (MSFT_iSCSISession)
    /// * `return_value` -  (u32)
    pub fn connect(&self, node_address: &String, target_portal_address: &String, target_portal_port_number: u16, initiator_portal_address: &String, is_data_digest: bool, is_header_digest: bool, report_to_pn_p: bool, authentication_type: &String, chap_username: &String, chap_secret: &String, is_multipath_enabled: bool, is_persistent: bool, initiator_instance_name: &String, createdi_scsisession: &mut MSFT_iSCSISession) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeAddress".to_string(), value: node_address.into() });
        args.push(MethodParameter { name: "TargetPortalAddress".to_string(), value: target_portal_address.into() });
        args.push(MethodParameter { name: "TargetPortalPortNumber".to_string(), value: target_portal_port_number.into() });
        args.push(MethodParameter { name: "InitiatorPortalAddress".to_string(), value: initiator_portal_address.into() });
        args.push(MethodParameter { name: "IsDataDigest".to_string(), value: is_data_digest.into() });
        args.push(MethodParameter { name: "IsHeaderDigest".to_string(), value: is_header_digest.into() });
        args.push(MethodParameter { name: "ReportToPnP".to_string(), value: report_to_pn_p.into() });
        args.push(MethodParameter { name: "AuthenticationType".to_string(), value: authentication_type.into() });
        args.push(MethodParameter { name: "ChapUsername".to_string(), value: chap_username.into() });
        args.push(MethodParameter { name: "ChapSecret".to_string(), value: chap_secret.into() });
        args.push(MethodParameter { name: "IsMultipathEnabled".to_string(), value: is_multipath_enabled.into() });
        args.push(MethodParameter { name: "IsPersistent".to_string(), value: is_persistent.into() });
        args.push(MethodParameter { name: "InitiatorInstanceName".to_string(), value: initiator_instance_name.into() });

        let result = self.invoke_method("Connect", &args)?;
        let createdi_scsisession = result.get_value("CreatediSCSISession")?;
        Ok(result.return_value)

    }

}


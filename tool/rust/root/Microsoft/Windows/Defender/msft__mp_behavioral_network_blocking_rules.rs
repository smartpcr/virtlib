// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Defender
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpBehavioralNetworkBlockingRules struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpBehavioralNetworkBlockingRules {
    #[serde(flatten)]
    pub base: BaseStatus,

/// 
    #[serde(rename = "BlockingAction")]
    pub blocking_action: Option<String>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<String>,

/// 
    #[serde(rename = "FilterGUID")]
    pub filter_guid: Option<String>,

/// 
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,

/// 
    #[serde(rename = "LocalPorts")]
    pub local_ports: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

/// 
    #[serde(rename = "RemotePorts")]
    pub remote_ports: Option<String>,
}

impl MSFT_MpBehavioralNetworkBlockingRules {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BaseStatus::new(),
            blocking_action: None,
            direction: None,
            filter_guid: None,
            ip_address: None,
            local_ports: None,
            protocol: None,
            remote_ports: None,
        }
    }


    /// Sets the value of BlockingAction
    pub fn set_blocking_action(&mut self, value: String) {
        self.blocking_action = Some(value);
    }

    /// Gets the value of BlockingAction
    pub fn get_blocking_action(&self) -> Option<&String> {
        self.blocking_action.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: String) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&String> {
        self.direction.as_ref()
    }

    /// Sets the value of FilterGUID
    pub fn set_filter_guid(&mut self, value: String) {
        self.filter_guid = Some(value);
    }

    /// Gets the value of FilterGUID
    pub fn get_filter_guid(&self) -> Option<&String> {
        self.filter_guid.as_ref()
    }

    /// Sets the value of IpAddress
    pub fn set_ip_address(&mut self, value: String) {
        self.ip_address = Some(value);
    }

    /// Gets the value of IpAddress
    pub fn get_ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    /// Sets the value of LocalPorts
    pub fn set_local_ports(&mut self, value: String) {
        self.local_ports = Some(value);
    }

    /// Gets the value of LocalPorts
    pub fn get_local_ports(&self) -> Option<&String> {
        self.local_ports.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: String) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemotePorts
    pub fn set_remote_ports(&mut self, value: String) {
        self.remote_ports = Some(value);
    }

    /// Gets the value of RemotePorts
    pub fn get_remote_ports(&self) -> Option<&String> {
        self.remote_ports.as_ref()
    }

/// 

    /// * `filter_guid` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove(&self, filter_guid: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilterGUID".to_string(), value: filter_guid.into() });
        self.invoke_method("Remove", &args)

    }

}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTransportFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTransportFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: Option<String>,

/// 
    #[serde(rename = "LocalPortEnd")]
    pub local_port_end: Option<u16>,

/// 
    #[serde(rename = "LocalPortStart")]
    pub local_port_start: Option<u16>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u16>,

/// 
    #[serde(rename = "RemotePortEnd")]
    pub remote_port_end: Option<u16>,

/// 
    #[serde(rename = "RemotePortStart")]
    pub remote_port_start: Option<u16>,

/// 
    #[serde(rename = "SettingName")]
    pub setting_name: Option<String>,
}

impl MSFT_NetTransportFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            destination_prefix: None,
            local_port_end: None,
            local_port_start: None,
            protocol: None,
            remote_port_end: None,
            remote_port_start: None,
            setting_name: None,
        }
    }


    /// Sets the value of DestinationPrefix
    pub fn set_destination_prefix(&mut self, value: String) {
        self.destination_prefix = Some(value);
    }

    /// Gets the value of DestinationPrefix
    pub fn get_destination_prefix(&self) -> Option<&String> {
        self.destination_prefix.as_ref()
    }

    /// Sets the value of LocalPortEnd
    pub fn set_local_port_end(&mut self, value: u16) {
        self.local_port_end = Some(value);
    }

    /// Gets the value of LocalPortEnd
    pub fn get_local_port_end(&self) -> Option<&u16> {
        self.local_port_end.as_ref()
    }

    /// Sets the value of LocalPortStart
    pub fn set_local_port_start(&mut self, value: u16) {
        self.local_port_start = Some(value);
    }

    /// Gets the value of LocalPortStart
    pub fn get_local_port_start(&self) -> Option<&u16> {
        self.local_port_start.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u16) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u16> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemotePortEnd
    pub fn set_remote_port_end(&mut self, value: u16) {
        self.remote_port_end = Some(value);
    }

    /// Gets the value of RemotePortEnd
    pub fn get_remote_port_end(&self) -> Option<&u16> {
        self.remote_port_end.as_ref()
    }

    /// Sets the value of RemotePortStart
    pub fn set_remote_port_start(&mut self, value: u16) {
        self.remote_port_start = Some(value);
    }

    /// Gets the value of RemotePortStart
    pub fn get_remote_port_start(&self) -> Option<&u16> {
        self.remote_port_start.as_ref()
    }

    /// Sets the value of SettingName
    pub fn set_setting_name(&mut self, value: String) {
        self.setting_name = Some(value);
    }

    /// Gets the value of SettingName
    pub fn get_setting_name(&self) -> Option<&String> {
        self.setting_name.as_ref()
    }
}


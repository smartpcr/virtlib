// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_NICConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_NICConfig {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Speed of network link in megabits per second.
    #[serde(rename = "LinkSpeed")]
    pub link_speed: Option<u32>,

/// Link State **typedef**
    #[serde(rename = "LinkState")]
    pub link_state: Option<NICConfig_LinkState>,

/// Ethernet MAC Address
    #[serde(rename = "MacAddress")]
    pub mac_address: Vec<u8>,

/// Maximum frame size
    #[serde(rename = "MaxFrameSize")]
    pub max_frame_size: Option<u32>,

/// Maximum Speed of network link in megabits per second.
    #[serde(rename = "MaxLinkSpeed")]
    pub max_link_speed: Option<u32>,
}

impl MSiSCSI_NICConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            link_speed: None,
            link_state: None,
            mac_address: Vec::new(),
            max_frame_size: None,
            max_link_speed: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LinkSpeed
    pub fn set_link_speed(&mut self, value: u32) {
        self.link_speed = Some(value);
    }

    /// Gets the value of LinkSpeed
    pub fn get_link_speed(&self) -> Option<&u32> {
        self.link_speed.as_ref()
    }

    /// Sets the value of LinkState
    pub fn set_link_state(&mut self, value: NICConfig_LinkState) {
        self.link_state = Some(value);
    }

    /// Gets the value of LinkState
    pub fn get_link_state(&self) -> Option<&NICConfig_LinkState> {
        self.link_state.as_ref()
    }

    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: Vec<u8>) {
        self.mac_address = value;
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> &Vec<u8> {
        &self.mac_address
    }

    /// Sets the value of MaxFrameSize
    pub fn set_max_frame_size(&mut self, value: u32) {
        self.max_frame_size = Some(value);
    }

    /// Gets the value of MaxFrameSize
    pub fn get_max_frame_size(&self) -> Option<&u32> {
        self.max_frame_size.as_ref()
    }

    /// Sets the value of MaxLinkSpeed
    pub fn set_max_link_speed(&mut self, value: u32) {
        self.max_link_speed = Some(value);
    }

    /// Gets the value of MaxLinkSpeed
    pub fn get_max_link_speed(&self) -> Option<&u32> {
        self.max_link_speed.as_ref()
    }
}


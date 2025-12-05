// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SyntheticEthernetPortSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SyntheticEthernetPortSettingData {
    #[serde(flatten)]
    pub base: CIM_EthernetPortAllocationSettingData,

/// 
    #[serde(rename = "AllowDirectTranslatedP2P")]
    pub allow_direct_translated_p2_p: Option<bool>,

/// 
    #[serde(rename = "AllowPacketDirect")]
    pub allow_packet_direct: Option<bool>,

/// 
    #[serde(rename = "ClusterMonitored")]
    pub cluster_monitored: Option<bool>,

/// 
    #[serde(rename = "DeviceNamingEnabled")]
    pub device_naming_enabled: Option<bool>,

/// 
    #[serde(rename = "InterruptModeration")]
    pub interrupt_moderation: Option<bool>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u32>,

/// 
    #[serde(rename = "NumaAwarePlacement")]
    pub numa_aware_placement: Option<bool>,

/// 
    #[serde(rename = "StaticMacAddress")]
    pub static_mac_address: Option<bool>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_SyntheticEthernetPortSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EthernetPortAllocationSettingData::new(),
            allow_direct_translated_p2_p: None,
            allow_packet_direct: None,
            cluster_monitored: None,
            device_naming_enabled: None,
            interrupt_moderation: None,
            media_type: None,
            numa_aware_placement: None,
            static_mac_address: None,
            virtual_system_identifiers: Vec::new(),
        }
    }


    /// Sets the value of AllowDirectTranslatedP2P
    pub fn set_allow_direct_translated_p2_p(&mut self, value: bool) {
        self.allow_direct_translated_p2_p = Some(value);
    }

    /// Gets the value of AllowDirectTranslatedP2P
    pub fn get_allow_direct_translated_p2_p(&self) -> Option<&bool> {
        self.allow_direct_translated_p2_p.as_ref()
    }

    /// Sets the value of AllowPacketDirect
    pub fn set_allow_packet_direct(&mut self, value: bool) {
        self.allow_packet_direct = Some(value);
    }

    /// Gets the value of AllowPacketDirect
    pub fn get_allow_packet_direct(&self) -> Option<&bool> {
        self.allow_packet_direct.as_ref()
    }

    /// Sets the value of ClusterMonitored
    pub fn set_cluster_monitored(&mut self, value: bool) {
        self.cluster_monitored = Some(value);
    }

    /// Gets the value of ClusterMonitored
    pub fn get_cluster_monitored(&self) -> Option<&bool> {
        self.cluster_monitored.as_ref()
    }

    /// Sets the value of DeviceNamingEnabled
    pub fn set_device_naming_enabled(&mut self, value: bool) {
        self.device_naming_enabled = Some(value);
    }

    /// Gets the value of DeviceNamingEnabled
    pub fn get_device_naming_enabled(&self) -> Option<&bool> {
        self.device_naming_enabled.as_ref()
    }

    /// Sets the value of InterruptModeration
    pub fn set_interrupt_moderation(&mut self, value: bool) {
        self.interrupt_moderation = Some(value);
    }

    /// Gets the value of InterruptModeration
    pub fn get_interrupt_moderation(&self) -> Option<&bool> {
        self.interrupt_moderation.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u32) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u32> {
        self.media_type.as_ref()
    }

    /// Sets the value of NumaAwarePlacement
    pub fn set_numa_aware_placement(&mut self, value: bool) {
        self.numa_aware_placement = Some(value);
    }

    /// Gets the value of NumaAwarePlacement
    pub fn get_numa_aware_placement(&self) -> Option<&bool> {
        self.numa_aware_placement.as_ref()
    }

    /// Sets the value of StaticMacAddress
    pub fn set_static_mac_address(&mut self, value: bool) {
        self.static_mac_address = Some(value);
    }

    /// Gets the value of StaticMacAddress
    pub fn get_static_mac_address(&self) -> Option<&bool> {
        self.static_mac_address.as_ref()
    }

    /// Sets the value of VirtualSystemIdentifiers
    pub fn set_virtual_system_identifiers(&mut self, value: Vec<String>) {
        self.virtual_system_identifiers = value;
    }

    /// Gets the value of VirtualSystemIdentifiers
    pub fn get_virtual_system_identifiers(&self) -> &Vec<String> {
        &self.virtual_system_identifiers
    }
}

impl Msvm_SyntheticEthernetPortSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}


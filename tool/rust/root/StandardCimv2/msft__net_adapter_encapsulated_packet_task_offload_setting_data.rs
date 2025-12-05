// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "EncapsulatedPacketTaskOffloadHardwareCapabilitiesNvgre")]
    pub encapsulated_packet_task_offload_hardware_capabilities_nvgre: Option<MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities>,

/// 
    #[serde(rename = "EncapsulatedPacketTaskOffloadHardwareCapabilitiesVxlan")]
    pub encapsulated_packet_task_offload_hardware_capabilities_vxlan: Option<MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx>,

/// 
    #[serde(rename = "EncapsulationType")]
    pub encapsulation_type: Option<u16>,

/// 
    #[serde(rename = "IsVxlanUDPPortConfigurable")]
    pub is_vxlan_udpport_configurable: Option<bool>,

/// 
    #[serde(rename = "NvgreEncapsulatedPacketTaskOffloadEnabled")]
    pub nvgre_encapsulated_packet_task_offload_enabled: Option<bool>,

/// 
    #[serde(rename = "VxlanEncapsulatedPacketTaskOffloadEnabled")]
    pub vxlan_encapsulated_packet_task_offload_enabled: Option<bool>,

/// 
    #[serde(rename = "VxlanUDPPortNumber")]
    pub vxlan_udpport_number: Option<u16>,
}

impl MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            encapsulated_packet_task_offload_hardware_capabilities_nvgre: None,
            encapsulated_packet_task_offload_hardware_capabilities_vxlan: None,
            encapsulation_type: None,
            is_vxlan_udpport_configurable: None,
            nvgre_encapsulated_packet_task_offload_enabled: None,
            vxlan_encapsulated_packet_task_offload_enabled: None,
            vxlan_udpport_number: None,
        }
    }


    /// Sets the value of EncapsulatedPacketTaskOffloadHardwareCapabilitiesNvgre
    pub fn set_encapsulated_packet_task_offload_hardware_capabilities_nvgre(&mut self, value: MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities) {
        self.encapsulated_packet_task_offload_hardware_capabilities_nvgre = Some(value);
    }

    /// Gets the value of EncapsulatedPacketTaskOffloadHardwareCapabilitiesNvgre
    pub fn get_encapsulated_packet_task_offload_hardware_capabilities_nvgre(&self) -> Option<&MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities> {
        self.encapsulated_packet_task_offload_hardware_capabilities_nvgre.as_ref()
    }

    /// Sets the value of EncapsulatedPacketTaskOffloadHardwareCapabilitiesVxlan
    pub fn set_encapsulated_packet_task_offload_hardware_capabilities_vxlan(&mut self, value: MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx) {
        self.encapsulated_packet_task_offload_hardware_capabilities_vxlan = Some(value);
    }

    /// Gets the value of EncapsulatedPacketTaskOffloadHardwareCapabilitiesVxlan
    pub fn get_encapsulated_packet_task_offload_hardware_capabilities_vxlan(&self) -> Option<&MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx> {
        self.encapsulated_packet_task_offload_hardware_capabilities_vxlan.as_ref()
    }

    /// Sets the value of EncapsulationType
    pub fn set_encapsulation_type(&mut self, value: u16) {
        self.encapsulation_type = Some(value);
    }

    /// Gets the value of EncapsulationType
    pub fn get_encapsulation_type(&self) -> Option<&u16> {
        self.encapsulation_type.as_ref()
    }

    /// Sets the value of IsVxlanUDPPortConfigurable
    pub fn set_is_vxlan_udpport_configurable(&mut self, value: bool) {
        self.is_vxlan_udpport_configurable = Some(value);
    }

    /// Gets the value of IsVxlanUDPPortConfigurable
    pub fn get_is_vxlan_udpport_configurable(&self) -> Option<&bool> {
        self.is_vxlan_udpport_configurable.as_ref()
    }

    /// Sets the value of NvgreEncapsulatedPacketTaskOffloadEnabled
    pub fn set_nvgre_encapsulated_packet_task_offload_enabled(&mut self, value: bool) {
        self.nvgre_encapsulated_packet_task_offload_enabled = Some(value);
    }

    /// Gets the value of NvgreEncapsulatedPacketTaskOffloadEnabled
    pub fn get_nvgre_encapsulated_packet_task_offload_enabled(&self) -> Option<&bool> {
        self.nvgre_encapsulated_packet_task_offload_enabled.as_ref()
    }

    /// Sets the value of VxlanEncapsulatedPacketTaskOffloadEnabled
    pub fn set_vxlan_encapsulated_packet_task_offload_enabled(&mut self, value: bool) {
        self.vxlan_encapsulated_packet_task_offload_enabled = Some(value);
    }

    /// Gets the value of VxlanEncapsulatedPacketTaskOffloadEnabled
    pub fn get_vxlan_encapsulated_packet_task_offload_enabled(&self) -> Option<&bool> {
        self.vxlan_encapsulated_packet_task_offload_enabled.as_ref()
    }

    /// Sets the value of VxlanUDPPortNumber
    pub fn set_vxlan_udpport_number(&mut self, value: u16) {
        self.vxlan_udpport_number = Some(value);
    }

    /// Gets the value of VxlanUDPPortNumber
    pub fn get_vxlan_udpport_number(&self) -> Option<&u16> {
        self.vxlan_udpport_number.as_ref()
    }

/// 

    /// * `encapsulation_type` -  (u16)

    /// * `cmdlet_output` -  (MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, encapsulation_type: u16, cmdlet_output: &mut MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EncapsulationType".to_string(), value: encapsulation_type.into() });

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `encapsulation_type` -  (u16)

    /// * `cmdlet_output` -  (MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, encapsulation_type: u16, cmdlet_output: &mut MSFT_NetAdapterEncapsulatedPacketTaskOffloadSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EncapsulationType".to_string(), value: encapsulation_type.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}


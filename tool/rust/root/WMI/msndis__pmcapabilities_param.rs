// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PMCapabilitiesParam struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PMCapabilitiesParam {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "DeviceSleepOnDisconnect")]
    pub device_sleep_on_disconnect: Option<MSNdis_PMCapabilityState>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "PMARPOffload")]
    pub pmarpoffload: Option<MSNdis_PMCapabilityState>,

/// 
    #[serde(rename = "PMNDOffload")]
    pub pmndoffload: Option<MSNdis_PMCapabilityState>,

/// 
    #[serde(rename = "PMWiFiRekeyOffload")]
    pub pmwi_fi_rekey_offload: Option<MSNdis_PMCapabilityState>,

/// 
    #[serde(rename = "WakeOnMagicPacket")]
    pub wake_on_magic_packet: Option<MSNdis_PMCapabilityState>,

/// 
    #[serde(rename = "WakeOnPattern")]
    pub wake_on_pattern: Option<MSNdis_PMCapabilityState>,
}

impl MSNdis_PMCapabilitiesParam {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            device_sleep_on_disconnect: None,
            header: None,
            pmarpoffload: None,
            pmndoffload: None,
            pmwi_fi_rekey_offload: None,
            wake_on_magic_packet: None,
            wake_on_pattern: None,
        }
    }


    /// Sets the value of DeviceSleepOnDisconnect
    pub fn set_device_sleep_on_disconnect(&mut self, value: MSNdis_PMCapabilityState) {
        self.device_sleep_on_disconnect = Some(value);
    }

    /// Gets the value of DeviceSleepOnDisconnect
    pub fn get_device_sleep_on_disconnect(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.device_sleep_on_disconnect.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of PMARPOffload
    pub fn set_pmarpoffload(&mut self, value: MSNdis_PMCapabilityState) {
        self.pmarpoffload = Some(value);
    }

    /// Gets the value of PMARPOffload
    pub fn get_pmarpoffload(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.pmarpoffload.as_ref()
    }

    /// Sets the value of PMNDOffload
    pub fn set_pmndoffload(&mut self, value: MSNdis_PMCapabilityState) {
        self.pmndoffload = Some(value);
    }

    /// Gets the value of PMNDOffload
    pub fn get_pmndoffload(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.pmndoffload.as_ref()
    }

    /// Sets the value of PMWiFiRekeyOffload
    pub fn set_pmwi_fi_rekey_offload(&mut self, value: MSNdis_PMCapabilityState) {
        self.pmwi_fi_rekey_offload = Some(value);
    }

    /// Gets the value of PMWiFiRekeyOffload
    pub fn get_pmwi_fi_rekey_offload(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.pmwi_fi_rekey_offload.as_ref()
    }

    /// Sets the value of WakeOnMagicPacket
    pub fn set_wake_on_magic_packet(&mut self, value: MSNdis_PMCapabilityState) {
        self.wake_on_magic_packet = Some(value);
    }

    /// Gets the value of WakeOnMagicPacket
    pub fn get_wake_on_magic_packet(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.wake_on_magic_packet.as_ref()
    }

    /// Sets the value of WakeOnPattern
    pub fn set_wake_on_pattern(&mut self, value: MSNdis_PMCapabilityState) {
        self.wake_on_pattern = Some(value);
    }

    /// Gets the value of WakeOnPattern
    pub fn get_wake_on_pattern(&self) -> Option<&MSNdis_PMCapabilityState> {
        self.wake_on_pattern.as_ref()
    }
}


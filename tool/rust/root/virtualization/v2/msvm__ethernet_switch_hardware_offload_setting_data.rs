// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchHardwareOffloadSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchHardwareOffloadSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchFeatureSettingData,

/// 
    #[serde(rename = "DefaultQueueVmmqEnabled")]
    pub default_queue_vmmq_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVmmqQueuePairs")]
    pub default_queue_vmmq_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "DefaultQueueVrssEnabled")]
    pub default_queue_vrss_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssExcludePrimaryProcessor")]
    pub default_queue_vrss_exclude_primary_processor: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssIndependentHostSpreading")]
    pub default_queue_vrss_independent_host_spreading: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssMinQueuePairs")]
    pub default_queue_vrss_min_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "DefaultQueueVrssQueueSchedulingMode")]
    pub default_queue_vrss_queue_scheduling_mode: Option<u32>,

/// 
    #[serde(rename = "RscOffloadEnabled")]
    pub rsc_offload_enabled: Option<bool>,

/// 
    #[serde(rename = "SoftwareRscEnabled")]
    pub software_rsc_enabled: Option<bool>,
}

impl Msvm_EthernetSwitchHardwareOffloadSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchFeatureSettingData::new(),
            default_queue_vmmq_enabled: None,
            default_queue_vmmq_queue_pairs: None,
            default_queue_vrss_enabled: None,
            default_queue_vrss_exclude_primary_processor: None,
            default_queue_vrss_independent_host_spreading: None,
            default_queue_vrss_min_queue_pairs: None,
            default_queue_vrss_queue_scheduling_mode: None,
            rsc_offload_enabled: None,
            software_rsc_enabled: None,
        }
    }


    /// Sets the value of DefaultQueueVmmqEnabled
    pub fn set_default_queue_vmmq_enabled(&mut self, value: bool) {
        self.default_queue_vmmq_enabled = Some(value);
    }

    /// Gets the value of DefaultQueueVmmqEnabled
    pub fn get_default_queue_vmmq_enabled(&self) -> Option<&bool> {
        self.default_queue_vmmq_enabled.as_ref()
    }

    /// Sets the value of DefaultQueueVmmqQueuePairs
    pub fn set_default_queue_vmmq_queue_pairs(&mut self, value: u32) {
        self.default_queue_vmmq_queue_pairs = Some(value);
    }

    /// Gets the value of DefaultQueueVmmqQueuePairs
    pub fn get_default_queue_vmmq_queue_pairs(&self) -> Option<&u32> {
        self.default_queue_vmmq_queue_pairs.as_ref()
    }

    /// Sets the value of DefaultQueueVrssEnabled
    pub fn set_default_queue_vrss_enabled(&mut self, value: bool) {
        self.default_queue_vrss_enabled = Some(value);
    }

    /// Gets the value of DefaultQueueVrssEnabled
    pub fn get_default_queue_vrss_enabled(&self) -> Option<&bool> {
        self.default_queue_vrss_enabled.as_ref()
    }

    /// Sets the value of DefaultQueueVrssExcludePrimaryProcessor
    pub fn set_default_queue_vrss_exclude_primary_processor(&mut self, value: bool) {
        self.default_queue_vrss_exclude_primary_processor = Some(value);
    }

    /// Gets the value of DefaultQueueVrssExcludePrimaryProcessor
    pub fn get_default_queue_vrss_exclude_primary_processor(&self) -> Option<&bool> {
        self.default_queue_vrss_exclude_primary_processor.as_ref()
    }

    /// Sets the value of DefaultQueueVrssIndependentHostSpreading
    pub fn set_default_queue_vrss_independent_host_spreading(&mut self, value: bool) {
        self.default_queue_vrss_independent_host_spreading = Some(value);
    }

    /// Gets the value of DefaultQueueVrssIndependentHostSpreading
    pub fn get_default_queue_vrss_independent_host_spreading(&self) -> Option<&bool> {
        self.default_queue_vrss_independent_host_spreading.as_ref()
    }

    /// Sets the value of DefaultQueueVrssMinQueuePairs
    pub fn set_default_queue_vrss_min_queue_pairs(&mut self, value: u32) {
        self.default_queue_vrss_min_queue_pairs = Some(value);
    }

    /// Gets the value of DefaultQueueVrssMinQueuePairs
    pub fn get_default_queue_vrss_min_queue_pairs(&self) -> Option<&u32> {
        self.default_queue_vrss_min_queue_pairs.as_ref()
    }

    /// Sets the value of DefaultQueueVrssQueueSchedulingMode
    pub fn set_default_queue_vrss_queue_scheduling_mode(&mut self, value: u32) {
        self.default_queue_vrss_queue_scheduling_mode = Some(value);
    }

    /// Gets the value of DefaultQueueVrssQueueSchedulingMode
    pub fn get_default_queue_vrss_queue_scheduling_mode(&self) -> Option<&u32> {
        self.default_queue_vrss_queue_scheduling_mode.as_ref()
    }

    /// Sets the value of RscOffloadEnabled
    pub fn set_rsc_offload_enabled(&mut self, value: bool) {
        self.rsc_offload_enabled = Some(value);
    }

    /// Gets the value of RscOffloadEnabled
    pub fn get_rsc_offload_enabled(&self) -> Option<&bool> {
        self.rsc_offload_enabled.as_ref()
    }

    /// Sets the value of SoftwareRscEnabled
    pub fn set_software_rsc_enabled(&mut self, value: bool) {
        self.software_rsc_enabled = Some(value);
    }

    /// Gets the value of SoftwareRscEnabled
    pub fn get_software_rsc_enabled(&self) -> Option<&bool> {
        self.software_rsc_enabled.as_ref()
    }
}

impl Msvm_EthernetSwitchHardwareOffloadSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}


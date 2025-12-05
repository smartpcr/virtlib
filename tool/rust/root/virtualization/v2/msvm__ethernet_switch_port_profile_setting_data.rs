// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortProfileSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortProfileSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "CdnLabelId")]
    pub cdn_label_id: Option<u32>,

/// 
    #[serde(rename = "CdnLabelString")]
    pub cdn_label_string: Option<String>,

/// 
    #[serde(rename = "NetCfgInstanceId")]
    pub net_cfg_instance_id: Option<String>,

/// 
    #[serde(rename = "PciBusNumber")]
    pub pci_bus_number: Option<u32>,

/// 
    #[serde(rename = "PciDeviceNumber")]
    pub pci_device_number: Option<u32>,

/// 
    #[serde(rename = "PciFunctionNumber")]
    pub pci_function_number: Option<u32>,

/// 
    #[serde(rename = "PciSegmentNumber")]
    pub pci_segment_number: Option<u32>,

/// 
    #[serde(rename = "ProfileData")]
    pub profile_data: Option<u32>,

/// 
    #[serde(rename = "ProfileId")]
    pub profile_id: Option<String>,

/// 
    #[serde(rename = "ProfileName")]
    pub profile_name: Option<String>,

/// 
    #[serde(rename = "VendorId")]
    pub vendor_id: Option<String>,

/// 
    #[serde(rename = "VendorName")]
    pub vendor_name: Option<String>,
}

impl Msvm_EthernetSwitchPortProfileSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            cdn_label_id: None,
            cdn_label_string: None,
            net_cfg_instance_id: None,
            pci_bus_number: None,
            pci_device_number: None,
            pci_function_number: None,
            pci_segment_number: None,
            profile_data: None,
            profile_id: None,
            profile_name: None,
            vendor_id: None,
            vendor_name: None,
        }
    }


    /// Sets the value of CdnLabelId
    pub fn set_cdn_label_id(&mut self, value: u32) {
        self.cdn_label_id = Some(value);
    }

    /// Gets the value of CdnLabelId
    pub fn get_cdn_label_id(&self) -> Option<&u32> {
        self.cdn_label_id.as_ref()
    }

    /// Sets the value of CdnLabelString
    pub fn set_cdn_label_string(&mut self, value: String) {
        self.cdn_label_string = Some(value);
    }

    /// Gets the value of CdnLabelString
    pub fn get_cdn_label_string(&self) -> Option<&String> {
        self.cdn_label_string.as_ref()
    }

    /// Sets the value of NetCfgInstanceId
    pub fn set_net_cfg_instance_id(&mut self, value: String) {
        self.net_cfg_instance_id = Some(value);
    }

    /// Gets the value of NetCfgInstanceId
    pub fn get_net_cfg_instance_id(&self) -> Option<&String> {
        self.net_cfg_instance_id.as_ref()
    }

    /// Sets the value of PciBusNumber
    pub fn set_pci_bus_number(&mut self, value: u32) {
        self.pci_bus_number = Some(value);
    }

    /// Gets the value of PciBusNumber
    pub fn get_pci_bus_number(&self) -> Option<&u32> {
        self.pci_bus_number.as_ref()
    }

    /// Sets the value of PciDeviceNumber
    pub fn set_pci_device_number(&mut self, value: u32) {
        self.pci_device_number = Some(value);
    }

    /// Gets the value of PciDeviceNumber
    pub fn get_pci_device_number(&self) -> Option<&u32> {
        self.pci_device_number.as_ref()
    }

    /// Sets the value of PciFunctionNumber
    pub fn set_pci_function_number(&mut self, value: u32) {
        self.pci_function_number = Some(value);
    }

    /// Gets the value of PciFunctionNumber
    pub fn get_pci_function_number(&self) -> Option<&u32> {
        self.pci_function_number.as_ref()
    }

    /// Sets the value of PciSegmentNumber
    pub fn set_pci_segment_number(&mut self, value: u32) {
        self.pci_segment_number = Some(value);
    }

    /// Gets the value of PciSegmentNumber
    pub fn get_pci_segment_number(&self) -> Option<&u32> {
        self.pci_segment_number.as_ref()
    }

    /// Sets the value of ProfileData
    pub fn set_profile_data(&mut self, value: u32) {
        self.profile_data = Some(value);
    }

    /// Gets the value of ProfileData
    pub fn get_profile_data(&self) -> Option<&u32> {
        self.profile_data.as_ref()
    }

    /// Sets the value of ProfileId
    pub fn set_profile_id(&mut self, value: String) {
        self.profile_id = Some(value);
    }

    /// Gets the value of ProfileId
    pub fn get_profile_id(&self) -> Option<&String> {
        self.profile_id.as_ref()
    }

    /// Sets the value of ProfileName
    pub fn set_profile_name(&mut self, value: String) {
        self.profile_name = Some(value);
    }

    /// Gets the value of ProfileName
    pub fn get_profile_name(&self) -> Option<&String> {
        self.profile_name.as_ref()
    }

    /// Sets the value of VendorId
    pub fn set_vendor_id(&mut self, value: String) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorId
    pub fn get_vendor_id(&self) -> Option<&String> {
        self.vendor_id.as_ref()
    }

    /// Sets the value of VendorName
    pub fn set_vendor_name(&mut self, value: String) {
        self.vendor_name = Some(value);
    }

    /// Gets the value of VendorName
    pub fn get_vendor_name(&self) -> Option<&String> {
        self.vendor_name.as_ref()
    }
}

impl Msvm_EthernetSwitchPortProfileSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}


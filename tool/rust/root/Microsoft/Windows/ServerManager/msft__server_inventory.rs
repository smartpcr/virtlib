// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerInventory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerInventory {

/// 
    #[serde(rename = "ActivationStatus")]
    pub activation_status: Option<u8>,

/// 
    #[serde(rename = "CbsTimestamp")]
    pub cbs_timestamp: Option<String>,

/// 
    #[serde(rename = "CeipSetting")]
    pub ceip_setting: Option<u8>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "Fqdn")]
    pub fqdn: Option<String>,

/// 
    #[serde(rename = "IsDomainJoined")]
    pub is_domain_joined: Option<bool>,

/// 
    #[serde(rename = "IsWerSettingEnforcedByGP")]
    pub is_wer_setting_enforced_by_gp: Option<bool>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProcessorNames")]
    pub processor_names: Vec<String>,

/// 
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

/// 
    #[serde(rename = "TotalPhysicalMemory")]
    pub total_physical_memory: Option<u64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,

/// 
    #[serde(rename = "WerSetting")]
    pub wer_setting: Option<u8>,
}

impl MSFT_ServerInventory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            activation_status: None,
            cbs_timestamp: None,
            ceip_setting: None,
            description: None,
            domain: None,
            fqdn: None,
            is_domain_joined: None,
            is_wer_setting_enforced_by_gp: None,
            manufacturer: None,
            name: None,
            processor_names: Vec::new(),
            product_id: None,
            total_physical_memory: None,
            type: None,
            wer_setting: None,
        }
    }


    /// Sets the value of ActivationStatus
    pub fn set_activation_status(&mut self, value: u8) {
        self.activation_status = Some(value);
    }

    /// Gets the value of ActivationStatus
    pub fn get_activation_status(&self) -> Option<&u8> {
        self.activation_status.as_ref()
    }

    /// Sets the value of CbsTimestamp
    pub fn set_cbs_timestamp(&mut self, value: String) {
        self.cbs_timestamp = Some(value);
    }

    /// Gets the value of CbsTimestamp
    pub fn get_cbs_timestamp(&self) -> Option<&String> {
        self.cbs_timestamp.as_ref()
    }

    /// Sets the value of CeipSetting
    pub fn set_ceip_setting(&mut self, value: u8) {
        self.ceip_setting = Some(value);
    }

    /// Gets the value of CeipSetting
    pub fn get_ceip_setting(&self) -> Option<&u8> {
        self.ceip_setting.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of Fqdn
    pub fn set_fqdn(&mut self, value: String) {
        self.fqdn = Some(value);
    }

    /// Gets the value of Fqdn
    pub fn get_fqdn(&self) -> Option<&String> {
        self.fqdn.as_ref()
    }

    /// Sets the value of IsDomainJoined
    pub fn set_is_domain_joined(&mut self, value: bool) {
        self.is_domain_joined = Some(value);
    }

    /// Gets the value of IsDomainJoined
    pub fn get_is_domain_joined(&self) -> Option<&bool> {
        self.is_domain_joined.as_ref()
    }

    /// Sets the value of IsWerSettingEnforcedByGP
    pub fn set_is_wer_setting_enforced_by_gp(&mut self, value: bool) {
        self.is_wer_setting_enforced_by_gp = Some(value);
    }

    /// Gets the value of IsWerSettingEnforcedByGP
    pub fn get_is_wer_setting_enforced_by_gp(&self) -> Option<&bool> {
        self.is_wer_setting_enforced_by_gp.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProcessorNames
    pub fn set_processor_names(&mut self, value: Vec<String>) {
        self.processor_names = value;
    }

    /// Gets the value of ProcessorNames
    pub fn get_processor_names(&self) -> &Vec<String> {
        &self.processor_names
    }

    /// Sets the value of ProductId
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductId
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of TotalPhysicalMemory
    pub fn set_total_physical_memory(&mut self, value: u64) {
        self.total_physical_memory = Some(value);
    }

    /// Gets the value of TotalPhysicalMemory
    pub fn get_total_physical_memory(&self) -> Option<&u64> {
        self.total_physical_memory.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }

    /// Sets the value of WerSetting
    pub fn set_wer_setting(&mut self, value: u8) {
        self.wer_setting = Some(value);
    }

    /// Gets the value of WerSetting
    pub fn get_wer_setting(&self) -> Option<&u8> {
        self.wer_setting.as_ref()
    }
}


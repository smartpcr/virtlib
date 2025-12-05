// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemMigrationNetworkSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemMigrationNetworkSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "Metric")]
    pub metric: Option<u32>,

/// 
    #[serde(rename = "PrefixLength")]
    pub prefix_length: Option<u8>,

/// 
    #[serde(rename = "SubnetNumber")]
    pub subnet_number: Option<String>,

/// 
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
}

impl Msvm_VirtualSystemMigrationNetworkSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            metric: None,
            prefix_length: None,
            subnet_number: None,
            tags: Vec::new(),
        }
    }


    /// Sets the value of Metric
    pub fn set_metric(&mut self, value: u32) {
        self.metric = Some(value);
    }

    /// Gets the value of Metric
    pub fn get_metric(&self) -> Option<&u32> {
        self.metric.as_ref()
    }

    /// Sets the value of PrefixLength
    pub fn set_prefix_length(&mut self, value: u8) {
        self.prefix_length = Some(value);
    }

    /// Gets the value of PrefixLength
    pub fn get_prefix_length(&self) -> Option<&u8> {
        self.prefix_length.as_ref()
    }

    /// Sets the value of SubnetNumber
    pub fn set_subnet_number(&mut self, value: String) {
        self.subnet_number = Some(value);
    }

    /// Gets the value of SubnetNumber
    pub fn get_subnet_number(&self) -> Option<&String> {
        self.subnet_number.as_ref()
    }

    /// Sets the value of Tags
    pub fn set_tags(&mut self, value: Vec<String>) {
        self.tags = value;
    }

    /// Gets the value of Tags
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl Msvm_VirtualSystemMigrationNetworkSettingData {
    /// Gets the related Msvm_VirtualSystemMigrationServiceSettingData object(s)
    pub fn get_related__virtual_system_migration_service_setting_data(&self) -> Result<Msvm_VirtualSystemMigrationServiceSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemMigrationServiceSettingData")
    }

}


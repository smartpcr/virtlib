// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_AbstractResourcePoolSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_AbstractResourcePoolSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "LoadBalancingBehavior")]
    pub load_balancing_behavior: Option<u16>,

/// 
    #[serde(rename = "MappingBehavior")]
    pub mapping_behavior: Option<u16>,

/// 
    #[serde(rename = "MappingOrder")]
    pub mapping_order: Vec<String>,

/// End-user supplied notes that are related to this ResourcePool.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

/// 
    #[serde(rename = "OtherResourceType")]
    pub other_resource_type: Option<String>,

/// 
    #[serde(rename = "PoolID")]
    pub pool_id: Option<String>,

/// 
    #[serde(rename = "ResourceSubType")]
    pub resource_sub_type: Option<String>,

/// 
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<u16>,
}

impl Msvm_AbstractResourcePoolSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            load_balancing_behavior: None,
            mapping_behavior: None,
            mapping_order: Vec::new(),
            notes: None,
            other_resource_type: None,
            pool_id: None,
            resource_sub_type: None,
            resource_type: None,
        }
    }


    /// Sets the value of LoadBalancingBehavior
    pub fn set_load_balancing_behavior(&mut self, value: u16) {
        self.load_balancing_behavior = Some(value);
    }

    /// Gets the value of LoadBalancingBehavior
    pub fn get_load_balancing_behavior(&self) -> Option<&u16> {
        self.load_balancing_behavior.as_ref()
    }

    /// Sets the value of MappingBehavior
    pub fn set_mapping_behavior(&mut self, value: u16) {
        self.mapping_behavior = Some(value);
    }

    /// Gets the value of MappingBehavior
    pub fn get_mapping_behavior(&self) -> Option<&u16> {
        self.mapping_behavior.as_ref()
    }

    /// Sets the value of MappingOrder
    pub fn set_mapping_order(&mut self, value: Vec<String>) {
        self.mapping_order = value;
    }

    /// Gets the value of MappingOrder
    pub fn get_mapping_order(&self) -> &Vec<String> {
        &self.mapping_order
    }

    /// Sets the value of Notes
    pub fn set_notes(&mut self, value: String) {
        self.notes = Some(value);
    }

    /// Gets the value of Notes
    pub fn get_notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    /// Sets the value of OtherResourceType
    pub fn set_other_resource_type(&mut self, value: String) {
        self.other_resource_type = Some(value);
    }

    /// Gets the value of OtherResourceType
    pub fn get_other_resource_type(&self) -> Option<&String> {
        self.other_resource_type.as_ref()
    }

    /// Sets the value of PoolID
    pub fn set_pool_id(&mut self, value: String) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolID
    pub fn get_pool_id(&self) -> Option<&String> {
        self.pool_id.as_ref()
    }

    /// Sets the value of ResourceSubType
    pub fn set_resource_sub_type(&mut self, value: String) {
        self.resource_sub_type = Some(value);
    }

    /// Gets the value of ResourceSubType
    pub fn get_resource_sub_type(&self) -> Option<&String> {
        self.resource_sub_type.as_ref()
    }

    /// Sets the value of ResourceType
    pub fn set_resource_type(&mut self, value: u16) {
        self.resource_type = Some(value);
    }

    /// Gets the value of ResourceType
    pub fn get_resource_type(&self) -> Option<&u16> {
        self.resource_type.as_ref()
    }
}

impl Msvm_AbstractResourcePoolSettingData {
    /// Gets the related Msvm_ResourcePool object(s)
    pub fn get_related__resource_pool(&self) -> Result<Msvm_ResourcePool, WmiError> {
        self.get_related("Msvm_ResourcePool")
    }

}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DevDetail_Microsoft02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DevDetail_Microsoft02 {

/// 
    #[serde(rename = "CommercializationOperator")]
    pub commercialization_operator: Option<String>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LocalTime")]
    pub local_time: Option<String>,

/// 
    #[serde(rename = "MobileID")]
    pub mobile_id: Option<String>,

/// 
    #[serde(rename = "OSPlatform")]
    pub osplatform: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ProcessorArchitecture")]
    pub processor_architecture: Option<i32>,

/// 
    #[serde(rename = "ProcessorType")]
    pub processor_type: Option<i32>,

/// 
    #[serde(rename = "RadioSwV")]
    pub radio_sw_v: Option<String>,

/// 
    #[serde(rename = "Resolution")]
    pub resolution: Option<String>,

/// 
    #[serde(rename = "SystemSKU")]
    pub system_sku: Option<String>,
}

impl MDM_DevDetail_Microsoft02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            commercialization_operator: None,
            device_name: None,
            instance_id: None,
            local_time: None,
            mobile_id: None,
            osplatform: None,
            parent_id: None,
            processor_architecture: None,
            processor_type: None,
            radio_sw_v: None,
            resolution: None,
            system_sku: None,
        }
    }


    /// Sets the value of CommercializationOperator
    pub fn set_commercialization_operator(&mut self, value: String) {
        self.commercialization_operator = Some(value);
    }

    /// Gets the value of CommercializationOperator
    pub fn get_commercialization_operator(&self) -> Option<&String> {
        self.commercialization_operator.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LocalTime
    pub fn set_local_time(&mut self, value: String) {
        self.local_time = Some(value);
    }

    /// Gets the value of LocalTime
    pub fn get_local_time(&self) -> Option<&String> {
        self.local_time.as_ref()
    }

    /// Sets the value of MobileID
    pub fn set_mobile_id(&mut self, value: String) {
        self.mobile_id = Some(value);
    }

    /// Gets the value of MobileID
    pub fn get_mobile_id(&self) -> Option<&String> {
        self.mobile_id.as_ref()
    }

    /// Sets the value of OSPlatform
    pub fn set_osplatform(&mut self, value: String) {
        self.osplatform = Some(value);
    }

    /// Gets the value of OSPlatform
    pub fn get_osplatform(&self) -> Option<&String> {
        self.osplatform.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ProcessorArchitecture
    pub fn set_processor_architecture(&mut self, value: i32) {
        self.processor_architecture = Some(value);
    }

    /// Gets the value of ProcessorArchitecture
    pub fn get_processor_architecture(&self) -> Option<&i32> {
        self.processor_architecture.as_ref()
    }

    /// Sets the value of ProcessorType
    pub fn set_processor_type(&mut self, value: i32) {
        self.processor_type = Some(value);
    }

    /// Gets the value of ProcessorType
    pub fn get_processor_type(&self) -> Option<&i32> {
        self.processor_type.as_ref()
    }

    /// Sets the value of RadioSwV
    pub fn set_radio_sw_v(&mut self, value: String) {
        self.radio_sw_v = Some(value);
    }

    /// Gets the value of RadioSwV
    pub fn get_radio_sw_v(&self) -> Option<&String> {
        self.radio_sw_v.as_ref()
    }

    /// Sets the value of Resolution
    pub fn set_resolution(&mut self, value: String) {
        self.resolution = Some(value);
    }

    /// Gets the value of Resolution
    pub fn get_resolution(&self) -> Option<&String> {
        self.resolution.as_ref()
    }

    /// Sets the value of SystemSKU
    pub fn set_system_sku(&mut self, value: String) {
        self.system_sku = Some(value);
    }

    /// Gets the value of SystemSKU
    pub fn get_system_sku(&self) -> Option<&String> {
        self.system_sku.as_ref()
    }
}


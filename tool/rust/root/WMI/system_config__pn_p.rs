// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_PnP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_PnP {
    #[serde(flatten)]
    pub base: SystemConfig,

/// 
    #[serde(rename = "ClassGuid")]
    pub class_guid: Option<serde_json::Value>,

/// 
    #[serde(rename = "DeviceDescription")]
    pub device_description: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "DevProblem")]
    pub dev_problem: Option<u32>,

/// 
    #[serde(rename = "DevStatus")]
    pub dev_status: Option<u32>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "LowerFilters")]
    pub lower_filters: Vec<String>,

/// 
    #[serde(rename = "LowerFiltersCount")]
    pub lower_filters_count: Option<u32>,

/// 
    #[serde(rename = "PdoName")]
    pub pdo_name: Option<String>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// 
    #[serde(rename = "UpperFilters")]
    pub upper_filters: Vec<String>,

/// 
    #[serde(rename = "UpperFiltersCount")]
    pub upper_filters_count: Option<u32>,
}

impl SystemConfig_PnP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig::new(),
            class_guid: None,
            device_description: None,
            device_id: None,
            dev_problem: None,
            dev_status: None,
            friendly_name: None,
            lower_filters: Vec::new(),
            lower_filters_count: None,
            pdo_name: None,
            service_name: None,
            upper_filters: Vec::new(),
            upper_filters_count: None,
        }
    }


    /// Sets the value of ClassGuid
    pub fn set_class_guid(&mut self, value: serde_json::Value) {
        self.class_guid = Some(value);
    }

    /// Gets the value of ClassGuid
    pub fn get_class_guid(&self) -> Option<&serde_json::Value> {
        self.class_guid.as_ref()
    }

    /// Sets the value of DeviceDescription
    pub fn set_device_description(&mut self, value: String) {
        self.device_description = Some(value);
    }

    /// Gets the value of DeviceDescription
    pub fn get_device_description(&self) -> Option<&String> {
        self.device_description.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of DevProblem
    pub fn set_dev_problem(&mut self, value: u32) {
        self.dev_problem = Some(value);
    }

    /// Gets the value of DevProblem
    pub fn get_dev_problem(&self) -> Option<&u32> {
        self.dev_problem.as_ref()
    }

    /// Sets the value of DevStatus
    pub fn set_dev_status(&mut self, value: u32) {
        self.dev_status = Some(value);
    }

    /// Gets the value of DevStatus
    pub fn get_dev_status(&self) -> Option<&u32> {
        self.dev_status.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of LowerFilters
    pub fn set_lower_filters(&mut self, value: Vec<String>) {
        self.lower_filters = value;
    }

    /// Gets the value of LowerFilters
    pub fn get_lower_filters(&self) -> &Vec<String> {
        &self.lower_filters
    }

    /// Sets the value of LowerFiltersCount
    pub fn set_lower_filters_count(&mut self, value: u32) {
        self.lower_filters_count = Some(value);
    }

    /// Gets the value of LowerFiltersCount
    pub fn get_lower_filters_count(&self) -> Option<&u32> {
        self.lower_filters_count.as_ref()
    }

    /// Sets the value of PdoName
    pub fn set_pdo_name(&mut self, value: String) {
        self.pdo_name = Some(value);
    }

    /// Gets the value of PdoName
    pub fn get_pdo_name(&self) -> Option<&String> {
        self.pdo_name.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of UpperFilters
    pub fn set_upper_filters(&mut self, value: Vec<String>) {
        self.upper_filters = value;
    }

    /// Gets the value of UpperFilters
    pub fn get_upper_filters(&self) -> &Vec<String> {
        &self.upper_filters
    }

    /// Sets the value of UpperFiltersCount
    pub fn set_upper_filters_count(&mut self, value: u32) {
        self.upper_filters_count = Some(value);
    }

    /// Gets the value of UpperFiltersCount
    pub fn get_upper_filters_count(&self) -> Option<&u32> {
        self.upper_filters_count.as_ref()
    }
}


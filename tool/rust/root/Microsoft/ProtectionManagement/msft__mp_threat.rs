// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpThreat struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpThreat {
    #[serde(flatten)]
    pub base: BaseStatus,

/// 
    #[serde(rename = "CategoryID")]
    pub category_id: Option<u8>,

/// 
    #[serde(rename = "DidThreatExecute")]
    pub did_threat_execute: Option<bool>,

/// 
    #[serde(rename = "IsActive")]
    pub is_active: Option<bool>,

/// 
    #[serde(rename = "Resources")]
    pub resources: Vec<String>,

/// 
    #[serde(rename = "RollupStatus")]
    pub rollup_status: Option<u32>,

/// 
    #[serde(rename = "SchemaVersion")]
    pub schema_version: Option<String>,

/// 
    #[serde(rename = "SeverityID")]
    pub severity_id: Option<u8>,

/// 
    #[serde(rename = "ThreatID")]
    pub threat_id: Option<i64>,

/// 
    #[serde(rename = "ThreatName")]
    pub threat_name: Option<String>,

/// 
    #[serde(rename = "TypeID")]
    pub type_id: Option<u8>,
}

impl MSFT_MpThreat {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BaseStatus::new(),
            category_id: None,
            did_threat_execute: None,
            is_active: None,
            resources: Vec::new(),
            rollup_status: None,
            schema_version: None,
            severity_id: None,
            threat_id: None,
            threat_name: None,
            type_id: None,
        }
    }


    /// Sets the value of CategoryID
    pub fn set_category_id(&mut self, value: u8) {
        self.category_id = Some(value);
    }

    /// Gets the value of CategoryID
    pub fn get_category_id(&self) -> Option<&u8> {
        self.category_id.as_ref()
    }

    /// Sets the value of DidThreatExecute
    pub fn set_did_threat_execute(&mut self, value: bool) {
        self.did_threat_execute = Some(value);
    }

    /// Gets the value of DidThreatExecute
    pub fn get_did_threat_execute(&self) -> Option<&bool> {
        self.did_threat_execute.as_ref()
    }

    /// Sets the value of IsActive
    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = Some(value);
    }

    /// Gets the value of IsActive
    pub fn get_is_active(&self) -> Option<&bool> {
        self.is_active.as_ref()
    }

    /// Sets the value of Resources
    pub fn set_resources(&mut self, value: Vec<String>) {
        self.resources = value;
    }

    /// Gets the value of Resources
    pub fn get_resources(&self) -> &Vec<String> {
        &self.resources
    }

    /// Sets the value of RollupStatus
    pub fn set_rollup_status(&mut self, value: u32) {
        self.rollup_status = Some(value);
    }

    /// Gets the value of RollupStatus
    pub fn get_rollup_status(&self) -> Option<&u32> {
        self.rollup_status.as_ref()
    }

    /// Sets the value of SchemaVersion
    pub fn set_schema_version(&mut self, value: String) {
        self.schema_version = Some(value);
    }

    /// Gets the value of SchemaVersion
    pub fn get_schema_version(&self) -> Option<&String> {
        self.schema_version.as_ref()
    }

    /// Sets the value of SeverityID
    pub fn set_severity_id(&mut self, value: u8) {
        self.severity_id = Some(value);
    }

    /// Gets the value of SeverityID
    pub fn get_severity_id(&self) -> Option<&u8> {
        self.severity_id.as_ref()
    }

    /// Sets the value of ThreatID
    pub fn set_threat_id(&mut self, value: i64) {
        self.threat_id = Some(value);
    }

    /// Gets the value of ThreatID
    pub fn get_threat_id(&self) -> Option<&i64> {
        self.threat_id.as_ref()
    }

    /// Sets the value of ThreatName
    pub fn set_threat_name(&mut self, value: String) {
        self.threat_name = Some(value);
    }

    /// Gets the value of ThreatName
    pub fn get_threat_name(&self) -> Option<&String> {
        self.threat_name.as_ref()
    }

    /// Sets the value of TypeID
    pub fn set_type_id(&mut self, value: u8) {
        self.type_id = Some(value);
    }

    /// Gets the value of TypeID
    pub fn get_type_id(&self) -> Option<&u8> {
        self.type_id.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn remove(&self) -> Result<(), WmiError> {
        self.invoke_method("Remove", &[])

    }

}


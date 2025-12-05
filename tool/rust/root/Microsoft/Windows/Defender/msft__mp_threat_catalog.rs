// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Defender
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpThreatCatalog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpThreatCatalog {
    #[serde(flatten)]
    pub base: BaseStatus,

/// 
    #[serde(rename = "CategoryID")]
    pub category_id: Option<u8>,

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

impl MSFT_MpThreatCatalog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BaseStatus::new(),
            category_id: None,
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
}


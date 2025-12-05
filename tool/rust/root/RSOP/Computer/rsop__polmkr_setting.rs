// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PolmkrSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PolmkrSetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "polmkrBaseCseGuid")]
    pub polmkr_base_cse_guid: Option<String>,

/// 
    #[serde(rename = "polmkrBaseGpeGuid")]
    pub polmkr_base_gpe_guid: Option<String>,

/// 
    #[serde(rename = "polmkrBaseGpoDisplayName")]
    pub polmkr_base_gpo_display_name: Option<String>,

/// 
    #[serde(rename = "polmkrBaseGpoGuid")]
    pub polmkr_base_gpo_guid: Option<String>,

/// 
    #[serde(rename = "polmkrBaseHash")]
    pub polmkr_base_hash: Option<String>,

/// 
    #[serde(rename = "polmkrBaseInstanceXml")]
    pub polmkr_base_instance_xml: Option<String>,

/// 
    #[serde(rename = "polmkrBaseKeyValues")]
    pub polmkr_base_key_values: Vec<String>,
}

impl RSOP_PolmkrSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            polmkr_base_cse_guid: None,
            polmkr_base_gpe_guid: None,
            polmkr_base_gpo_display_name: None,
            polmkr_base_gpo_guid: None,
            polmkr_base_hash: None,
            polmkr_base_instance_xml: None,
            polmkr_base_key_values: Vec::new(),
        }
    }


    /// Sets the value of polmkrBaseCseGuid
    pub fn set_polmkr_base_cse_guid(&mut self, value: String) {
        self.polmkr_base_cse_guid = Some(value);
    }

    /// Gets the value of polmkrBaseCseGuid
    pub fn get_polmkr_base_cse_guid(&self) -> Option<&String> {
        self.polmkr_base_cse_guid.as_ref()
    }

    /// Sets the value of polmkrBaseGpeGuid
    pub fn set_polmkr_base_gpe_guid(&mut self, value: String) {
        self.polmkr_base_gpe_guid = Some(value);
    }

    /// Gets the value of polmkrBaseGpeGuid
    pub fn get_polmkr_base_gpe_guid(&self) -> Option<&String> {
        self.polmkr_base_gpe_guid.as_ref()
    }

    /// Sets the value of polmkrBaseGpoDisplayName
    pub fn set_polmkr_base_gpo_display_name(&mut self, value: String) {
        self.polmkr_base_gpo_display_name = Some(value);
    }

    /// Gets the value of polmkrBaseGpoDisplayName
    pub fn get_polmkr_base_gpo_display_name(&self) -> Option<&String> {
        self.polmkr_base_gpo_display_name.as_ref()
    }

    /// Sets the value of polmkrBaseGpoGuid
    pub fn set_polmkr_base_gpo_guid(&mut self, value: String) {
        self.polmkr_base_gpo_guid = Some(value);
    }

    /// Gets the value of polmkrBaseGpoGuid
    pub fn get_polmkr_base_gpo_guid(&self) -> Option<&String> {
        self.polmkr_base_gpo_guid.as_ref()
    }

    /// Sets the value of polmkrBaseHash
    pub fn set_polmkr_base_hash(&mut self, value: String) {
        self.polmkr_base_hash = Some(value);
    }

    /// Gets the value of polmkrBaseHash
    pub fn get_polmkr_base_hash(&self) -> Option<&String> {
        self.polmkr_base_hash.as_ref()
    }

    /// Sets the value of polmkrBaseInstanceXml
    pub fn set_polmkr_base_instance_xml(&mut self, value: String) {
        self.polmkr_base_instance_xml = Some(value);
    }

    /// Gets the value of polmkrBaseInstanceXml
    pub fn get_polmkr_base_instance_xml(&self) -> Option<&String> {
        self.polmkr_base_instance_xml.as_ref()
    }

    /// Sets the value of polmkrBaseKeyValues
    pub fn set_polmkr_base_key_values(&mut self, value: Vec<String>) {
        self.polmkr_base_key_values = value;
    }

    /// Gets the value of polmkrBaseKeyValues
    pub fn get_polmkr_base_key_values(&self) -> &Vec<String> {
        &self.polmkr_base_key_values
    }
}


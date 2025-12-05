// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEEE80211PolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEEE80211PolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "ClassName")]
    pub class_name: Option<String>,

/// 
    #[serde(rename = "description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "msieee80211Data")]
    pub msieee80211_data: Vec<u8>,

/// 
    #[serde(rename = "msieee80211DataType")]
    pub msieee80211_data_type: Option<u32>,

/// 
    #[serde(rename = "msieee80211ID")]
    pub msieee80211_id: Option<String>,

/// 
    #[serde(rename = "msieee80211Name")]
    pub msieee80211_name: Option<String>,

/// 
    #[serde(rename = "whenChanged")]
    pub when_changed: Option<u32>,
}

impl RSOP_IEEE80211PolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            class_name: None,
            description: None,
            msieee80211_data: Vec::new(),
            msieee80211_data_type: None,
            msieee80211_id: None,
            msieee80211_name: None,
            when_changed: None,
        }
    }


    /// Sets the value of ClassName
    pub fn set_class_name(&mut self, value: String) {
        self.class_name = Some(value);
    }

    /// Gets the value of ClassName
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
    }

    /// Sets the value of description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of msieee80211Data
    pub fn set_msieee80211_data(&mut self, value: Vec<u8>) {
        self.msieee80211_data = value;
    }

    /// Gets the value of msieee80211Data
    pub fn get_msieee80211_data(&self) -> &Vec<u8> {
        &self.msieee80211_data
    }

    /// Sets the value of msieee80211DataType
    pub fn set_msieee80211_data_type(&mut self, value: u32) {
        self.msieee80211_data_type = Some(value);
    }

    /// Gets the value of msieee80211DataType
    pub fn get_msieee80211_data_type(&self) -> Option<&u32> {
        self.msieee80211_data_type.as_ref()
    }

    /// Sets the value of msieee80211ID
    pub fn set_msieee80211_id(&mut self, value: String) {
        self.msieee80211_id = Some(value);
    }

    /// Gets the value of msieee80211ID
    pub fn get_msieee80211_id(&self) -> Option<&String> {
        self.msieee80211_id.as_ref()
    }

    /// Sets the value of msieee80211Name
    pub fn set_msieee80211_name(&mut self, value: String) {
        self.msieee80211_name = Some(value);
    }

    /// Gets the value of msieee80211Name
    pub fn get_msieee80211_name(&self) -> Option<&String> {
        self.msieee80211_name.as_ref()
    }

    /// Sets the value of whenChanged
    pub fn set_when_changed(&mut self, value: u32) {
        self.when_changed = Some(value);
    }

    /// Gets the value of whenChanged
    pub fn get_when_changed(&self) -> Option<&u32> {
        self.when_changed.as_ref()
    }
}


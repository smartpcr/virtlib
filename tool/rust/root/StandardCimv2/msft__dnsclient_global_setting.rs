// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DNSClientGlobalSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DNSClientGlobalSetting {
    #[serde(flatten)]
    pub base: CIM_DNSGeneralSettingData,

/// 705
    #[serde(rename = "DevolutionLevel")]
    pub devolution_level: Option<u32>,

/// 704
    #[serde(rename = "SuffixSearchList")]
    pub suffix_search_list: Vec<String>,

/// 703
    #[serde(rename = "UseDevolution")]
    pub use_devolution: Option<bool>,

/// 702
    #[serde(rename = "UseSuffixSearchList")]
    pub use_suffix_search_list: Option<bool>,
}

impl MSFT_DNSClientGlobalSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DNSGeneralSettingData::new(),
            devolution_level: None,
            suffix_search_list: Vec::new(),
            use_devolution: None,
            use_suffix_search_list: None,
        }
    }


    /// Sets the value of DevolutionLevel
    pub fn set_devolution_level(&mut self, value: u32) {
        self.devolution_level = Some(value);
    }

    /// Gets the value of DevolutionLevel
    pub fn get_devolution_level(&self) -> Option<&u32> {
        self.devolution_level.as_ref()
    }

    /// Sets the value of SuffixSearchList
    pub fn set_suffix_search_list(&mut self, value: Vec<String>) {
        self.suffix_search_list = value;
    }

    /// Gets the value of SuffixSearchList
    pub fn get_suffix_search_list(&self) -> &Vec<String> {
        &self.suffix_search_list
    }

    /// Sets the value of UseDevolution
    pub fn set_use_devolution(&mut self, value: bool) {
        self.use_devolution = Some(value);
    }

    /// Gets the value of UseDevolution
    pub fn get_use_devolution(&self) -> Option<&bool> {
        self.use_devolution.as_ref()
    }

    /// Sets the value of UseSuffixSearchList
    pub fn set_use_suffix_search_list(&mut self, value: bool) {
        self.use_suffix_search_list = Some(value);
    }

    /// Gets the value of UseSuffixSearchList
    pub fn get_use_suffix_search_list(&self) -> Option<&bool> {
        self.use_suffix_search_list.as_ref()
    }
}


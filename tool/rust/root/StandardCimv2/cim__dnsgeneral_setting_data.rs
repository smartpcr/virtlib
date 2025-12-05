// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DNSGeneralSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DNSGeneralSettingData {
    #[serde(flatten)]
    pub base: CIM_IPAssignmentSettingData,

/// 698
    #[serde(rename = "AppendParentSuffixes")]
    pub append_parent_suffixes: Option<bool>,

/// 697
    #[serde(rename = "AppendPrimarySuffixes")]
    pub append_primary_suffixes: Option<bool>,

/// 699
    #[serde(rename = "DNSSuffixesToAppend")]
    pub dnssuffixes_to_append: Vec<String>,
}

impl CIM_DNSGeneralSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IPAssignmentSettingData::new(),
            append_parent_suffixes: None,
            append_primary_suffixes: None,
            dnssuffixes_to_append: Vec::new(),
        }
    }


    /// Sets the value of AppendParentSuffixes
    pub fn set_append_parent_suffixes(&mut self, value: bool) {
        self.append_parent_suffixes = Some(value);
    }

    /// Gets the value of AppendParentSuffixes
    pub fn get_append_parent_suffixes(&self) -> Option<&bool> {
        self.append_parent_suffixes.as_ref()
    }

    /// Sets the value of AppendPrimarySuffixes
    pub fn set_append_primary_suffixes(&mut self, value: bool) {
        self.append_primary_suffixes = Some(value);
    }

    /// Gets the value of AppendPrimarySuffixes
    pub fn get_append_primary_suffixes(&self) -> Option<&bool> {
        self.append_primary_suffixes.as_ref()
    }

    /// Sets the value of DNSSuffixesToAppend
    pub fn set_dnssuffixes_to_append(&mut self, value: Vec<String>) {
        self.dnssuffixes_to_append = value;
    }

    /// Gets the value of DNSSuffixesToAppend
    pub fn get_dnssuffixes_to_append(&self) -> &Vec<String> {
        &self.dnssuffixes_to_append
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_HbaApiVersion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_HbaApiVersion {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "HbaApiVersion")]
    pub hba_api_version: Option<u32>,

/// 
    #[serde(rename = "WmiHbaApiVersion")]
    pub wmi_hba_api_version: Option<u32>,
}

impl MS_SM_HbaApiVersion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            hba_api_version: None,
            wmi_hba_api_version: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of HbaApiVersion
    pub fn set_hba_api_version(&mut self, value: u32) {
        self.hba_api_version = Some(value);
    }

    /// Gets the value of HbaApiVersion
    pub fn get_hba_api_version(&self) -> Option<&u32> {
        self.hba_api_version.as_ref()
    }

    /// Sets the value of WmiHbaApiVersion
    pub fn set_wmi_hba_api_version(&mut self, value: u32) {
        self.wmi_hba_api_version = Some(value);
    }

    /// Gets the value of WmiHbaApiVersion
    pub fn get_wmi_hba_api_version(&self) -> Option<&u32> {
        self.wmi_hba_api_version.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_ScanUpdateInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_ScanUpdateInfo {

/// 
    #[serde(rename = "UpdateDesc")]
    pub update_desc: Option<String>,

/// 
    #[serde(rename = "UpdateId")]
    pub update_id: Option<String>,

/// 
    #[serde(rename = "UpdateTitle")]
    pub update_title: Option<String>,
}

impl MSFT_CAU_ScanUpdateInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            update_desc: None,
            update_id: None,
            update_title: None,
        }
    }


    /// Sets the value of UpdateDesc
    pub fn set_update_desc(&mut self, value: String) {
        self.update_desc = Some(value);
    }

    /// Gets the value of UpdateDesc
    pub fn get_update_desc(&self) -> Option<&String> {
        self.update_desc.as_ref()
    }

    /// Sets the value of UpdateId
    pub fn set_update_id(&mut self, value: String) {
        self.update_id = Some(value);
    }

    /// Gets the value of UpdateId
    pub fn get_update_id(&self) -> Option<&String> {
        self.update_id.as_ref()
    }

    /// Sets the value of UpdateTitle
    pub fn set_update_title(&mut self, value: String) {
        self.update_title = Some(value);
    }

    /// Gets the value of UpdateTitle
    pub fn get_update_title(&self) -> Option<&String> {
        self.update_title.as_ref()
    }
}


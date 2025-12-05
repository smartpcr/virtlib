// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_WindowsUpdate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_WindowsUpdate {
    #[serde(flatten)]
    pub base: MsftSil_Data,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,
}

impl MsftSil_WindowsUpdate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsftSil_Data::new(),
            id: None,
            install_date: None,
        }
    }


    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterNfcTag struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterNfcTag {

/// 
    #[serde(rename = "Locked")]
    pub locked: Option<bool>,

/// 
    #[serde(rename = "SharePath")]
    pub share_path: Vec<String>,

/// 
    #[serde(rename = "WsdAddress")]
    pub wsd_address: Vec<String>,
}

impl MSFT_PrinterNfcTag {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            locked: None,
            share_path: Vec::new(),
            wsd_address: Vec::new(),
        }
    }


    /// Sets the value of Locked
    pub fn set_locked(&mut self, value: bool) {
        self.locked = Some(value);
    }

    /// Gets the value of Locked
    pub fn get_locked(&self) -> Option<&bool> {
        self.locked.as_ref()
    }

    /// Sets the value of SharePath
    pub fn set_share_path(&mut self, value: Vec<String>) {
        self.share_path = value;
    }

    /// Gets the value of SharePath
    pub fn get_share_path(&self) -> &Vec<String> {
        &self.share_path
    }

    /// Sets the value of WsdAddress
    pub fn set_wsd_address(&mut self, value: Vec<String>) {
        self.wsd_address = value;
    }

    /// Gets the value of WsdAddress
    pub fn get_wsd_address(&self) -> &Vec<String> {
        &self.wsd_address
    }
}


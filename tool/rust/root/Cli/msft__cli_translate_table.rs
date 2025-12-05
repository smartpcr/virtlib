// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliTranslateTable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliTranslateTable {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Tbl")]
    pub tbl: Vec<MSFT_CliTranslateTableEntry>,
}

impl MSFT_CliTranslateTable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            tbl: Vec::new(),
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Tbl
    pub fn set_tbl(&mut self, value: Vec<MSFT_CliTranslateTableEntry>) {
        self.tbl = value;
    }

    /// Gets the value of Tbl
    pub fn get_tbl(&self) -> &Vec<MSFT_CliTranslateTableEntry> {
        &self.tbl
    }
}


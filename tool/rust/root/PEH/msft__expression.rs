// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Expression struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Expression {

/// 
    #[serde(rename = "SourceInfo")]
    pub source_info: Option<String>,

/// 
    #[serde(rename = "SourceLines")]
    pub source_lines: Vec<String>,
}

impl MSFT_Expression {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            source_info: None,
            source_lines: Vec::new(),
        }
    }


    /// Sets the value of SourceInfo
    pub fn set_source_info(&mut self, value: String) {
        self.source_info = Some(value);
    }

    /// Gets the value of SourceInfo
    pub fn get_source_info(&self) -> Option<&String> {
        self.source_info.as_ref()
    }

    /// Sets the value of SourceLines
    pub fn set_source_lines(&mut self, value: Vec<String>) {
        self.source_lines = value;
    }

    /// Gets the value of SourceLines
    pub fn get_source_lines(&self) -> &Vec<String> {
        &self.source_lines
    }
}


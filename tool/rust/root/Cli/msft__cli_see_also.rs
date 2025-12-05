// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliSeeAlso struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliSeeAlso {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Original")]
    pub original: Option<MSFT_CliAlias>,

/// 
    #[serde(rename = "Related")]
    pub related: Option<MSFT_CliAlias>,
}

impl MSFT_CliSeeAlso {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            original: None,
            related: None,
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

    /// Sets the value of Original
    pub fn set_original(&mut self, value: MSFT_CliAlias) {
        self.original = Some(value);
    }

    /// Gets the value of Original
    pub fn get_original(&self) -> Option<&MSFT_CliAlias> {
        self.original.as_ref()
    }

    /// Sets the value of Related
    pub fn set_related(&mut self, value: MSFT_CliAlias) {
        self.related = Some(value);
    }

    /// Gets the value of Related
    pub fn get_related(&self) -> Option<&MSFT_CliAlias> {
        self.related.as_ref()
    }
}


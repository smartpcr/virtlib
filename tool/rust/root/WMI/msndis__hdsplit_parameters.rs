// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_HDSplitParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_HDSplitParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "HDSplitCombineFlags")]
    pub hdsplit_combine_flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,
}

impl MSNdis_HDSplitParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            hdsplit_combine_flags: None,
            header: None,
        }
    }


    /// Sets the value of HDSplitCombineFlags
    pub fn set_hdsplit_combine_flags(&mut self, value: u32) {
        self.hdsplit_combine_flags = Some(value);
    }

    /// Gets the value of HDSplitCombineFlags
    pub fn get_hdsplit_combine_flags(&self) -> Option<&u32> {
        self.hdsplit_combine_flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }
}


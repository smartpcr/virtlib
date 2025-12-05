// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ObjectHeader struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ObjectHeader {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Revision")]
    pub revision: Option<u8>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u16>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,
}

impl MSNdis_ObjectHeader {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            revision: None,
            size: None,
            type: None,
        }
    }


    /// Sets the value of Revision
    pub fn set_revision(&mut self, value: u8) {
        self.revision = Some(value);
    }

    /// Gets the value of Revision
    pub fn get_revision(&self) -> Option<&u8> {
        self.revision.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u16) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u16> {
        self.size.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }
}


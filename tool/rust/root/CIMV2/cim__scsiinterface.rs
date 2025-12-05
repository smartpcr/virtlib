// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SCSIInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SCSIInterface {
    #[serde(flatten)]
    pub base: CIM_ControlledBy,

/// 
    #[serde(rename = "SCSIRetries")]
    pub scsiretries: Option<u32>,

/// 
    #[serde(rename = "SCSITimeouts")]
    pub scsitimeouts: Option<u32>,
}

impl CIM_SCSIInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ControlledBy::new(),
            scsiretries: None,
            scsitimeouts: None,
        }
    }


    /// Sets the value of SCSIRetries
    pub fn set_scsiretries(&mut self, value: u32) {
        self.scsiretries = Some(value);
    }

    /// Gets the value of SCSIRetries
    pub fn get_scsiretries(&self) -> Option<&u32> {
        self.scsiretries.as_ref()
    }

    /// Sets the value of SCSITimeouts
    pub fn set_scsitimeouts(&mut self, value: u32) {
        self.scsitimeouts = Some(value);
    }

    /// Gets the value of SCSITimeouts
    pub fn get_scsitimeouts(&self) -> Option<&u32> {
        self.scsitimeouts.as_ref()
    }
}


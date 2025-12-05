// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HBAFCPBindingEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HBAFCPBindingEntry {

/// 
    #[serde(rename = "FCPId")]
    pub fcpid: Option<HBAFCPID>,

/// 
    #[serde(rename = "ScsiId")]
    pub scsi_id: Option<HBAScsiID>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl HBAFCPBindingEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fcpid: None,
            scsi_id: None,
            type: None,
        }
    }


    /// Sets the value of FCPId
    pub fn set_fcpid(&mut self, value: HBAFCPID) {
        self.fcpid = Some(value);
    }

    /// Gets the value of FCPId
    pub fn get_fcpid(&self) -> Option<&HBAFCPID> {
        self.fcpid.as_ref()
    }

    /// Sets the value of ScsiId
    pub fn set_scsi_id(&mut self, value: HBAScsiID) {
        self.scsi_id = Some(value);
    }

    /// Gets the value of ScsiId
    pub fn get_scsi_id(&self) -> Option<&HBAScsiID> {
        self.scsi_id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}


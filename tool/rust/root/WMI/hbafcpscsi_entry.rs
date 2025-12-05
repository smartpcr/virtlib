// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HBAFCPScsiEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HBAFCPScsiEntry {

/// 
    #[serde(rename = "FCPId")]
    pub fcpid: Option<HBAFCPID>,

/// 
    #[serde(rename = "Luid")]
    pub luid: Vec<u8>,

/// 
    #[serde(rename = "ScsiId")]
    pub scsi_id: Option<HBAScsiID>,
}

impl HBAFCPScsiEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fcpid: None,
            luid: Vec::new(),
            scsi_id: None,
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

    /// Sets the value of Luid
    pub fn set_luid(&mut self, value: Vec<u8>) {
        self.luid = value;
    }

    /// Gets the value of Luid
    pub fn get_luid(&self) -> &Vec<u8> {
        &self.luid
    }

    /// Sets the value of ScsiId
    pub fn set_scsi_id(&mut self, value: HBAScsiID) {
        self.scsi_id = Some(value);
    }

    /// Gets the value of ScsiId
    pub fn get_scsi_id(&self) -> Option<&HBAScsiID> {
        self.scsi_id.as_ref()
    }
}


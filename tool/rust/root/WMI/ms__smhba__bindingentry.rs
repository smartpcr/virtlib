// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_BINDINGENTRY struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_BINDINGENTRY {

/// 
    #[serde(rename = "LUID")]
    pub luid: Vec<u8>,

/// 
    #[serde(rename = "PortLun")]
    pub port_lun: Option<MS_SMHBA_PORTLUN>,

/// 
    #[serde(rename = "ScsiId")]
    pub scsi_id: Option<HBAScsiID>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "type")]
    pub type: Option<u32>,
}

impl MS_SMHBA_BINDINGENTRY {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            luid: Vec::new(),
            port_lun: None,
            scsi_id: None,
            status: None,
            type: None,
        }
    }


    /// Sets the value of LUID
    pub fn set_luid(&mut self, value: Vec<u8>) {
        self.luid = value;
    }

    /// Gets the value of LUID
    pub fn get_luid(&self) -> &Vec<u8> {
        &self.luid
    }

    /// Sets the value of PortLun
    pub fn set_port_lun(&mut self, value: MS_SMHBA_PORTLUN) {
        self.port_lun = Some(value);
    }

    /// Gets the value of PortLun
    pub fn get_port_lun(&self) -> Option<&MS_SMHBA_PORTLUN> {
        self.port_lun.as_ref()
    }

    /// Sets the value of ScsiId
    pub fn set_scsi_id(&mut self, value: HBAScsiID) {
        self.scsi_id = Some(value);
    }

    /// Gets the value of ScsiId
    pub fn get_scsi_id(&self) -> Option<&HBAScsiID> {
        self.scsi_id.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}


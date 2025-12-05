// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_VirtualFibrePortAttributes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_VirtualFibrePortAttributes {

/// 
    #[serde(rename = "FabricWWN")]
    pub fabric_wwn: Vec<u8>,

/// 
    #[serde(rename = "FCId")]
    pub fcid: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<VirtualFibrePortAttributes_Status>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Vec<u8>,

/// 
    #[serde(rename = "VirtualName")]
    pub virtual_name: Vec<u16>,

/// 
    #[serde(rename = "WWNN")]
    pub wwnn: Vec<u8>,

/// 
    #[serde(rename = "WWPN")]
    pub wwpn: Vec<u8>,
}

impl MSFC_VirtualFibrePortAttributes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fabric_wwn: Vec::new(),
            fcid: None,
            status: None,
            tag: Vec::new(),
            virtual_name: Vec::new(),
            wwnn: Vec::new(),
            wwpn: Vec::new(),
        }
    }


    /// Sets the value of FabricWWN
    pub fn set_fabric_wwn(&mut self, value: Vec<u8>) {
        self.fabric_wwn = value;
    }

    /// Gets the value of FabricWWN
    pub fn get_fabric_wwn(&self) -> &Vec<u8> {
        &self.fabric_wwn
    }

    /// Sets the value of FCId
    pub fn set_fcid(&mut self, value: u32) {
        self.fcid = Some(value);
    }

    /// Gets the value of FCId
    pub fn get_fcid(&self) -> Option<&u32> {
        self.fcid.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: VirtualFibrePortAttributes_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&VirtualFibrePortAttributes_Status> {
        self.status.as_ref()
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: Vec<u8>) {
        self.tag = value;
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> &Vec<u8> {
        &self.tag
    }

    /// Sets the value of VirtualName
    pub fn set_virtual_name(&mut self, value: Vec<u16>) {
        self.virtual_name = value;
    }

    /// Gets the value of VirtualName
    pub fn get_virtual_name(&self) -> &Vec<u16> {
        &self.virtual_name
    }

    /// Sets the value of WWNN
    pub fn set_wwnn(&mut self, value: Vec<u8>) {
        self.wwnn = value;
    }

    /// Gets the value of WWNN
    pub fn get_wwnn(&self) -> &Vec<u8> {
        &self.wwnn
    }

    /// Sets the value of WWPN
    pub fn set_wwpn(&mut self, value: Vec<u8>) {
        self.wwpn = value;
    }

    /// Gets the value of WWPN
    pub fn get_wwpn(&self) -> &Vec<u8> {
        &self.wwpn
    }
}


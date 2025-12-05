// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DevDetail struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DevDetail {

/// 
    #[serde(rename = "DevTyp")]
    pub dev_typ: Option<String>,

/// 
    #[serde(rename = "FwV")]
    pub fw_v: Option<String>,

/// 
    #[serde(rename = "HwV")]
    pub hw_v: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LrgObj")]
    pub lrg_obj: Option<bool>,

/// 
    #[serde(rename = "OEM")]
    pub oem: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SwV")]
    pub sw_v: Option<String>,
}

impl MDM_DevDetail {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dev_typ: None,
            fw_v: None,
            hw_v: None,
            instance_id: None,
            lrg_obj: None,
            oem: None,
            parent_id: None,
            sw_v: None,
        }
    }


    /// Sets the value of DevTyp
    pub fn set_dev_typ(&mut self, value: String) {
        self.dev_typ = Some(value);
    }

    /// Gets the value of DevTyp
    pub fn get_dev_typ(&self) -> Option<&String> {
        self.dev_typ.as_ref()
    }

    /// Sets the value of FwV
    pub fn set_fw_v(&mut self, value: String) {
        self.fw_v = Some(value);
    }

    /// Gets the value of FwV
    pub fn get_fw_v(&self) -> Option<&String> {
        self.fw_v.as_ref()
    }

    /// Sets the value of HwV
    pub fn set_hw_v(&mut self, value: String) {
        self.hw_v = Some(value);
    }

    /// Gets the value of HwV
    pub fn get_hw_v(&self) -> Option<&String> {
        self.hw_v.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LrgObj
    pub fn set_lrg_obj(&mut self, value: bool) {
        self.lrg_obj = Some(value);
    }

    /// Gets the value of LrgObj
    pub fn get_lrg_obj(&self) -> Option<&bool> {
        self.lrg_obj.as_ref()
    }

    /// Sets the value of OEM
    pub fn set_oem(&mut self, value: String) {
        self.oem = Some(value);
    }

    /// Gets the value of OEM
    pub fn get_oem(&self) -> Option<&String> {
        self.oem.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SwV
    pub fn set_sw_v(&mut self, value: String) {
        self.sw_v = Some(value);
    }

    /// Gets the value of SwV
    pub fn get_sw_v(&self) -> Option<&String> {
        self.sw_v.as_ref()
    }
}


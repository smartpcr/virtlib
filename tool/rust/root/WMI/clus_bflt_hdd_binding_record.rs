// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltHddBindingRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltHddBindingRecord {

/// Id.
    #[serde(rename = "BindingId")]
    pub binding_id: Option<String>,

/// cDirtyPages.
    #[serde(rename = "cDirtyPages")]
    pub c_dirty_pages: Option<u32>,

/// cDirtySlots.
    #[serde(rename = "cDirtySlots")]
    pub c_dirty_slots: Option<u32>,

/// cPages.
    #[serde(rename = "cPages")]
    pub c_pages: Option<u32>,

/// cPagesL2.
    #[serde(rename = "cPagesL2")]
    pub c_pages_l2: Option<u32>,

/// cRefs.
    #[serde(rename = "cRefs")]
    pub c_refs: Option<u32>,

/// Device Guid.
    #[serde(rename = "DeviceGuid")]
    pub device_guid: Option<String>,

/// DeviceSize.
    #[serde(rename = "DeviceSize")]
    pub device_size: Option<u64>,

/// Flags.
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,
}

impl ClusBfltHddBindingRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            binding_id: None,
            c_dirty_pages: None,
            c_dirty_slots: None,
            c_pages: None,
            c_pages_l2: None,
            c_refs: None,
            device_guid: None,
            device_size: None,
            flags: None,
        }
    }


    /// Sets the value of BindingId
    pub fn set_binding_id(&mut self, value: String) {
        self.binding_id = Some(value);
    }

    /// Gets the value of BindingId
    pub fn get_binding_id(&self) -> Option<&String> {
        self.binding_id.as_ref()
    }

    /// Sets the value of cDirtyPages
    pub fn set_c_dirty_pages(&mut self, value: u32) {
        self.c_dirty_pages = Some(value);
    }

    /// Gets the value of cDirtyPages
    pub fn get_c_dirty_pages(&self) -> Option<&u32> {
        self.c_dirty_pages.as_ref()
    }

    /// Sets the value of cDirtySlots
    pub fn set_c_dirty_slots(&mut self, value: u32) {
        self.c_dirty_slots = Some(value);
    }

    /// Gets the value of cDirtySlots
    pub fn get_c_dirty_slots(&self) -> Option<&u32> {
        self.c_dirty_slots.as_ref()
    }

    /// Sets the value of cPages
    pub fn set_c_pages(&mut self, value: u32) {
        self.c_pages = Some(value);
    }

    /// Gets the value of cPages
    pub fn get_c_pages(&self) -> Option<&u32> {
        self.c_pages.as_ref()
    }

    /// Sets the value of cPagesL2
    pub fn set_c_pages_l2(&mut self, value: u32) {
        self.c_pages_l2 = Some(value);
    }

    /// Gets the value of cPagesL2
    pub fn get_c_pages_l2(&self) -> Option<&u32> {
        self.c_pages_l2.as_ref()
    }

    /// Sets the value of cRefs
    pub fn set_c_refs(&mut self, value: u32) {
        self.c_refs = Some(value);
    }

    /// Gets the value of cRefs
    pub fn get_c_refs(&self) -> Option<&u32> {
        self.c_refs.as_ref()
    }

    /// Sets the value of DeviceGuid
    pub fn set_device_guid(&mut self, value: String) {
        self.device_guid = Some(value);
    }

    /// Gets the value of DeviceGuid
    pub fn get_device_guid(&self) -> Option<&String> {
        self.device_guid.as_ref()
    }

    /// Sets the value of DeviceSize
    pub fn set_device_size(&mut self, value: u64) {
        self.device_size = Some(value);
    }

    /// Gets the value of DeviceSize
    pub fn get_device_size(&self) -> Option<&u64> {
        self.device_size.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }
}


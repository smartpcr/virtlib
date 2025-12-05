// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_ScsiInfoExceptions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_ScsiInfoExceptions {
    #[serde(flatten)]
    pub base: MSStorageDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "IntervalTimer")]
    pub interval_timer: Option<u32>,

/// 
    #[serde(rename = "MRIE")]
    pub mrie: Option<u8>,

/// 
    #[serde(rename = "Padding")]
    pub padding: Option<u8>,

/// 
    #[serde(rename = "PageSavable")]
    pub page_savable: Option<bool>,

/// 
    #[serde(rename = "ReportCount")]
    pub report_count: Option<u32>,
}

impl MSStorageDriver_ScsiInfoExceptions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSStorageDriver::new(),
            active: None,
            flags: None,
            instance_name: None,
            interval_timer: None,
            mrie: None,
            padding: None,
            page_savable: None,
            report_count: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u8) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u8> {
        self.flags.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IntervalTimer
    pub fn set_interval_timer(&mut self, value: u32) {
        self.interval_timer = Some(value);
    }

    /// Gets the value of IntervalTimer
    pub fn get_interval_timer(&self) -> Option<&u32> {
        self.interval_timer.as_ref()
    }

    /// Sets the value of MRIE
    pub fn set_mrie(&mut self, value: u8) {
        self.mrie = Some(value);
    }

    /// Gets the value of MRIE
    pub fn get_mrie(&self) -> Option<&u8> {
        self.mrie.as_ref()
    }

    /// Sets the value of Padding
    pub fn set_padding(&mut self, value: u8) {
        self.padding = Some(value);
    }

    /// Gets the value of Padding
    pub fn get_padding(&self) -> Option<&u8> {
        self.padding.as_ref()
    }

    /// Sets the value of PageSavable
    pub fn set_page_savable(&mut self, value: bool) {
        self.page_savable = Some(value);
    }

    /// Gets the value of PageSavable
    pub fn get_page_savable(&self) -> Option<&bool> {
        self.page_savable.as_ref()
    }

    /// Sets the value of ReportCount
    pub fn set_report_count(&mut self, value: u32) {
        self.report_count = Some(value);
    }

    /// Gets the value of ReportCount
    pub fn get_report_count(&self) -> Option<&u32> {
        self.report_count.as_ref()
    }
}


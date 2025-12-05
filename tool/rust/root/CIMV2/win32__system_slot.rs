// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SystemSlot struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SystemSlot {
    #[serde(flatten)]
    pub base: CIM_Slot,

/// 
    #[serde(rename = "BusNumber")]
    pub bus_number: Option<u32>,

/// 
    #[serde(rename = "CurrentUsage")]
    pub current_usage: Option<u16>,

/// 
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// 
    #[serde(rename = "FunctionNumber")]
    pub function_number: Option<u32>,

/// 
    #[serde(rename = "PMESignal")]
    pub pmesignal: Option<bool>,

/// 
    #[serde(rename = "SegmentGroupNumber")]
    pub segment_group_number: Option<u32>,

/// 
    #[serde(rename = "Shared")]
    pub shared: Option<bool>,

/// 
    #[serde(rename = "SlotDesignation")]
    pub slot_designation: Option<String>,
}

impl Win32_SystemSlot {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Slot::new(),
            bus_number: None,
            current_usage: None,
            device_number: None,
            function_number: None,
            pmesignal: None,
            segment_group_number: None,
            shared: None,
            slot_designation: None,
        }
    }


    /// Sets the value of BusNumber
    pub fn set_bus_number(&mut self, value: u32) {
        self.bus_number = Some(value);
    }

    /// Gets the value of BusNumber
    pub fn get_bus_number(&self) -> Option<&u32> {
        self.bus_number.as_ref()
    }

    /// Sets the value of CurrentUsage
    pub fn set_current_usage(&mut self, value: u16) {
        self.current_usage = Some(value);
    }

    /// Gets the value of CurrentUsage
    pub fn get_current_usage(&self) -> Option<&u16> {
        self.current_usage.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of FunctionNumber
    pub fn set_function_number(&mut self, value: u32) {
        self.function_number = Some(value);
    }

    /// Gets the value of FunctionNumber
    pub fn get_function_number(&self) -> Option<&u32> {
        self.function_number.as_ref()
    }

    /// Sets the value of PMESignal
    pub fn set_pmesignal(&mut self, value: bool) {
        self.pmesignal = Some(value);
    }

    /// Gets the value of PMESignal
    pub fn get_pmesignal(&self) -> Option<&bool> {
        self.pmesignal.as_ref()
    }

    /// Sets the value of SegmentGroupNumber
    pub fn set_segment_group_number(&mut self, value: u32) {
        self.segment_group_number = Some(value);
    }

    /// Gets the value of SegmentGroupNumber
    pub fn get_segment_group_number(&self) -> Option<&u32> {
        self.segment_group_number.as_ref()
    }

    /// Sets the value of Shared
    pub fn set_shared(&mut self, value: bool) {
        self.shared = Some(value);
    }

    /// Gets the value of Shared
    pub fn get_shared(&self) -> Option<&bool> {
        self.shared.as_ref()
    }

    /// Sets the value of SlotDesignation
    pub fn set_slot_designation(&mut self, value: String) {
        self.slot_designation = Some(value);
    }

    /// Gets the value of SlotDesignation
    pub fn get_slot_designation(&self) -> Option<&String> {
        self.slot_designation.as_ref()
    }
}


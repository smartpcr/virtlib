// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSerial_PerformanceInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSerial_PerformanceInformation {
    #[serde(flatten)]
    pub base: MSSerial,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BufferOverrunErrorCount")]
    pub buffer_overrun_error_count: Option<u32>,

/// 
    #[serde(rename = "FrameErrorCount")]
    pub frame_error_count: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "ParityErrorCount")]
    pub parity_error_count: Option<u32>,

/// 
    #[serde(rename = "ReceivedCount")]
    pub received_count: Option<u32>,

/// 
    #[serde(rename = "SerialOverrunErrorCount")]
    pub serial_overrun_error_count: Option<u32>,

/// 
    #[serde(rename = "TransmittedCount")]
    pub transmitted_count: Option<u32>,
}

impl MSSerial_PerformanceInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSerial::new(),
            active: None,
            buffer_overrun_error_count: None,
            frame_error_count: None,
            instance_name: None,
            parity_error_count: None,
            received_count: None,
            serial_overrun_error_count: None,
            transmitted_count: None,
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

    /// Sets the value of BufferOverrunErrorCount
    pub fn set_buffer_overrun_error_count(&mut self, value: u32) {
        self.buffer_overrun_error_count = Some(value);
    }

    /// Gets the value of BufferOverrunErrorCount
    pub fn get_buffer_overrun_error_count(&self) -> Option<&u32> {
        self.buffer_overrun_error_count.as_ref()
    }

    /// Sets the value of FrameErrorCount
    pub fn set_frame_error_count(&mut self, value: u32) {
        self.frame_error_count = Some(value);
    }

    /// Gets the value of FrameErrorCount
    pub fn get_frame_error_count(&self) -> Option<&u32> {
        self.frame_error_count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of ParityErrorCount
    pub fn set_parity_error_count(&mut self, value: u32) {
        self.parity_error_count = Some(value);
    }

    /// Gets the value of ParityErrorCount
    pub fn get_parity_error_count(&self) -> Option<&u32> {
        self.parity_error_count.as_ref()
    }

    /// Sets the value of ReceivedCount
    pub fn set_received_count(&mut self, value: u32) {
        self.received_count = Some(value);
    }

    /// Gets the value of ReceivedCount
    pub fn get_received_count(&self) -> Option<&u32> {
        self.received_count.as_ref()
    }

    /// Sets the value of SerialOverrunErrorCount
    pub fn set_serial_overrun_error_count(&mut self, value: u32) {
        self.serial_overrun_error_count = Some(value);
    }

    /// Gets the value of SerialOverrunErrorCount
    pub fn get_serial_overrun_error_count(&self) -> Option<&u32> {
        self.serial_overrun_error_count.as_ref()
    }

    /// Sets the value of TransmittedCount
    pub fn set_transmitted_count(&mut self, value: u32) {
        self.transmitted_count = Some(value);
    }

    /// Gets the value of TransmittedCount
    pub fn get_transmitted_count(&self) -> Option<&u32> {
        self.transmitted_count.as_ref()
    }
}


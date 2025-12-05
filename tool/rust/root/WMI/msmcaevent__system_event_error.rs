// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMCAEvent_SystemEventError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMCAEvent_SystemEventError {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AdditionalErrors")]
    pub additional_errors: Option<u32>,

/// 
    #[serde(rename = "Cpu")]
    pub cpu: Option<u32>,

/// 
    #[serde(rename = "ErrorSeverity")]
    pub error_severity: Option<SystemEventError_ErrorSeverity>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LogToEventlog")]
    pub log_to_eventlog: Option<u32>,

/// 
    #[serde(rename = "RawRecord")]
    pub raw_record: Vec<u8>,

/// 
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,

/// 
    #[serde(rename = "SEL_DATA1")]
    pub sel__data1: Option<u8>,

/// 
    #[serde(rename = "SEL_DATA2")]
    pub sel__data2: Option<u8>,

/// 
    #[serde(rename = "SEL_DATA3")]
    pub sel__data3: Option<u8>,

/// 
    #[serde(rename = "SEL_EVENT_DIR_TYPE")]
    pub sel__event__dir__type: Option<u8>,

/// 
    #[serde(rename = "SEL_EVM_REV")]
    pub sel__evm__rev: Option<u8>,

/// 
    #[serde(rename = "SEL_GENERATOR_ID")]
    pub sel__generator__id: Option<u16>,

/// 
    #[serde(rename = "SEL_RECORD_ID")]
    pub sel__record__id: Option<u16>,

/// 
    #[serde(rename = "SEL_RECORD_TYPE")]
    pub sel__record__type: Option<u8>,

/// 
    #[serde(rename = "SEL_SENSOR_NUM")]
    pub sel__sensor__num: Option<u8>,

/// 
    #[serde(rename = "SEL_SENSOR_TYPE")]
    pub sel__sensor__type: Option<u8>,

/// 
    #[serde(rename = "SEL_TIME_STAMP")]
    pub sel__time__stamp: Option<u64>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<SystemEventError_Type>,

/// 
    #[serde(rename = "VALIDATION_BITS")]
    pub validation__bits: Option<u64>,
}

impl MSMCAEvent_SystemEventError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            additional_errors: None,
            cpu: None,
            error_severity: None,
            instance_name: None,
            log_to_eventlog: None,
            raw_record: Vec::new(),
            record_id: None,
            sel__data1: None,
            sel__data2: None,
            sel__data3: None,
            sel__event__dir__type: None,
            sel__evm__rev: None,
            sel__generator__id: None,
            sel__record__id: None,
            sel__record__type: None,
            sel__sensor__num: None,
            sel__sensor__type: None,
            sel__time__stamp: None,
            size: None,
            type: None,
            validation__bits: None,
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

    /// Sets the value of AdditionalErrors
    pub fn set_additional_errors(&mut self, value: u32) {
        self.additional_errors = Some(value);
    }

    /// Gets the value of AdditionalErrors
    pub fn get_additional_errors(&self) -> Option<&u32> {
        self.additional_errors.as_ref()
    }

    /// Sets the value of Cpu
    pub fn set_cpu(&mut self, value: u32) {
        self.cpu = Some(value);
    }

    /// Gets the value of Cpu
    pub fn get_cpu(&self) -> Option<&u32> {
        self.cpu.as_ref()
    }

    /// Sets the value of ErrorSeverity
    pub fn set_error_severity(&mut self, value: SystemEventError_ErrorSeverity) {
        self.error_severity = Some(value);
    }

    /// Gets the value of ErrorSeverity
    pub fn get_error_severity(&self) -> Option<&SystemEventError_ErrorSeverity> {
        self.error_severity.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LogToEventlog
    pub fn set_log_to_eventlog(&mut self, value: u32) {
        self.log_to_eventlog = Some(value);
    }

    /// Gets the value of LogToEventlog
    pub fn get_log_to_eventlog(&self) -> Option<&u32> {
        self.log_to_eventlog.as_ref()
    }

    /// Sets the value of RawRecord
    pub fn set_raw_record(&mut self, value: Vec<u8>) {
        self.raw_record = value;
    }

    /// Gets the value of RawRecord
    pub fn get_raw_record(&self) -> &Vec<u8> {
        &self.raw_record
    }

    /// Sets the value of RecordId
    pub fn set_record_id(&mut self, value: u64) {
        self.record_id = Some(value);
    }

    /// Gets the value of RecordId
    pub fn get_record_id(&self) -> Option<&u64> {
        self.record_id.as_ref()
    }

    /// Sets the value of SEL_DATA1
    pub fn set_sel__data1(&mut self, value: u8) {
        self.sel__data1 = Some(value);
    }

    /// Gets the value of SEL_DATA1
    pub fn get_sel__data1(&self) -> Option<&u8> {
        self.sel__data1.as_ref()
    }

    /// Sets the value of SEL_DATA2
    pub fn set_sel__data2(&mut self, value: u8) {
        self.sel__data2 = Some(value);
    }

    /// Gets the value of SEL_DATA2
    pub fn get_sel__data2(&self) -> Option<&u8> {
        self.sel__data2.as_ref()
    }

    /// Sets the value of SEL_DATA3
    pub fn set_sel__data3(&mut self, value: u8) {
        self.sel__data3 = Some(value);
    }

    /// Gets the value of SEL_DATA3
    pub fn get_sel__data3(&self) -> Option<&u8> {
        self.sel__data3.as_ref()
    }

    /// Sets the value of SEL_EVENT_DIR_TYPE
    pub fn set_sel__event__dir__type(&mut self, value: u8) {
        self.sel__event__dir__type = Some(value);
    }

    /// Gets the value of SEL_EVENT_DIR_TYPE
    pub fn get_sel__event__dir__type(&self) -> Option<&u8> {
        self.sel__event__dir__type.as_ref()
    }

    /// Sets the value of SEL_EVM_REV
    pub fn set_sel__evm__rev(&mut self, value: u8) {
        self.sel__evm__rev = Some(value);
    }

    /// Gets the value of SEL_EVM_REV
    pub fn get_sel__evm__rev(&self) -> Option<&u8> {
        self.sel__evm__rev.as_ref()
    }

    /// Sets the value of SEL_GENERATOR_ID
    pub fn set_sel__generator__id(&mut self, value: u16) {
        self.sel__generator__id = Some(value);
    }

    /// Gets the value of SEL_GENERATOR_ID
    pub fn get_sel__generator__id(&self) -> Option<&u16> {
        self.sel__generator__id.as_ref()
    }

    /// Sets the value of SEL_RECORD_ID
    pub fn set_sel__record__id(&mut self, value: u16) {
        self.sel__record__id = Some(value);
    }

    /// Gets the value of SEL_RECORD_ID
    pub fn get_sel__record__id(&self) -> Option<&u16> {
        self.sel__record__id.as_ref()
    }

    /// Sets the value of SEL_RECORD_TYPE
    pub fn set_sel__record__type(&mut self, value: u8) {
        self.sel__record__type = Some(value);
    }

    /// Gets the value of SEL_RECORD_TYPE
    pub fn get_sel__record__type(&self) -> Option<&u8> {
        self.sel__record__type.as_ref()
    }

    /// Sets the value of SEL_SENSOR_NUM
    pub fn set_sel__sensor__num(&mut self, value: u8) {
        self.sel__sensor__num = Some(value);
    }

    /// Gets the value of SEL_SENSOR_NUM
    pub fn get_sel__sensor__num(&self) -> Option<&u8> {
        self.sel__sensor__num.as_ref()
    }

    /// Sets the value of SEL_SENSOR_TYPE
    pub fn set_sel__sensor__type(&mut self, value: u8) {
        self.sel__sensor__type = Some(value);
    }

    /// Gets the value of SEL_SENSOR_TYPE
    pub fn get_sel__sensor__type(&self) -> Option<&u8> {
        self.sel__sensor__type.as_ref()
    }

    /// Sets the value of SEL_TIME_STAMP
    pub fn set_sel__time__stamp(&mut self, value: u64) {
        self.sel__time__stamp = Some(value);
    }

    /// Gets the value of SEL_TIME_STAMP
    pub fn get_sel__time__stamp(&self) -> Option<&u64> {
        self.sel__time__stamp.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: SystemEventError_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&SystemEventError_Type> {
        self.type.as_ref()
    }

    /// Sets the value of VALIDATION_BITS
    pub fn set_validation__bits(&mut self, value: u64) {
        self.validation__bits = Some(value);
    }

    /// Gets the value of VALIDATION_BITS
    pub fn get_validation__bits(&self) -> Option<&u64> {
        self.validation__bits.as_ref()
    }
}


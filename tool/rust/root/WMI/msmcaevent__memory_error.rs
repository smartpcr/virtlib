// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMCAEvent_MemoryError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMCAEvent_MemoryError {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AdditionalErrors")]
    pub additional_errors: Option<u32>,

/// 
    #[serde(rename = "BUS_SPECIFIC_DATA")]
    pub bus__specific__data: Option<u64>,

/// 
    #[serde(rename = "Cpu")]
    pub cpu: Option<u32>,

/// 
    #[serde(rename = "ErrorSeverity")]
    pub error_severity: Option<MemoryError_ErrorSeverity>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LogToEventlog")]
    pub log_to_eventlog: Option<u32>,

/// 
    #[serde(rename = "MEM_BANK")]
    pub mem__bank: Option<u16>,

/// 
    #[serde(rename = "MEM_BIT_POSITION")]
    pub mem__bit__position: Option<u16>,

/// 
    #[serde(rename = "MEM_CARD")]
    pub mem__card: Option<u16>,

/// 
    #[serde(rename = "MEM_COLUMN")]
    pub mem__column: Option<u16>,

/// 
    #[serde(rename = "MEM_ERROR_STATUS")]
    pub mem__error__status: Option<u64>,

/// 
    #[serde(rename = "MEM_MODULE")]
    pub mem__module: Option<u16>,

/// 
    #[serde(rename = "MEM_NODE")]
    pub mem__node: Option<u16>,

/// 
    #[serde(rename = "MEM_PHYSICAL_ADDR")]
    pub mem__physical__addr: Option<u64>,

/// 
    #[serde(rename = "MEM_PHYSICAL_MASK")]
    pub mem__physical__mask: Option<u64>,

/// 
    #[serde(rename = "MEM_ROW")]
    pub mem__row: Option<u16>,

/// 
    #[serde(rename = "RawRecord")]
    pub raw_record: Vec<u8>,

/// 
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,

/// 
    #[serde(rename = "REQUESTOR_ID")]
    pub requestor__id: Option<u64>,

/// 
    #[serde(rename = "RESPONDER_ID")]
    pub responder__id: Option<u64>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "TARGET_ID")]
    pub target__id: Option<u64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<MemoryError_Type>,

/// 
    #[serde(rename = "VALIDATION_BITS")]
    pub validation__bits: Option<u64>,

/// 
    #[serde(rename = "xMEM_DEVICE")]
    pub x_mem__device: Option<u16>,
}

impl MSMCAEvent_MemoryError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            additional_errors: None,
            bus__specific__data: None,
            cpu: None,
            error_severity: None,
            instance_name: None,
            log_to_eventlog: None,
            mem__bank: None,
            mem__bit__position: None,
            mem__card: None,
            mem__column: None,
            mem__error__status: None,
            mem__module: None,
            mem__node: None,
            mem__physical__addr: None,
            mem__physical__mask: None,
            mem__row: None,
            raw_record: Vec::new(),
            record_id: None,
            requestor__id: None,
            responder__id: None,
            size: None,
            target__id: None,
            type: None,
            validation__bits: None,
            x_mem__device: None,
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

    /// Sets the value of BUS_SPECIFIC_DATA
    pub fn set_bus__specific__data(&mut self, value: u64) {
        self.bus__specific__data = Some(value);
    }

    /// Gets the value of BUS_SPECIFIC_DATA
    pub fn get_bus__specific__data(&self) -> Option<&u64> {
        self.bus__specific__data.as_ref()
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
    pub fn set_error_severity(&mut self, value: MemoryError_ErrorSeverity) {
        self.error_severity = Some(value);
    }

    /// Gets the value of ErrorSeverity
    pub fn get_error_severity(&self) -> Option<&MemoryError_ErrorSeverity> {
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

    /// Sets the value of MEM_BANK
    pub fn set_mem__bank(&mut self, value: u16) {
        self.mem__bank = Some(value);
    }

    /// Gets the value of MEM_BANK
    pub fn get_mem__bank(&self) -> Option<&u16> {
        self.mem__bank.as_ref()
    }

    /// Sets the value of MEM_BIT_POSITION
    pub fn set_mem__bit__position(&mut self, value: u16) {
        self.mem__bit__position = Some(value);
    }

    /// Gets the value of MEM_BIT_POSITION
    pub fn get_mem__bit__position(&self) -> Option<&u16> {
        self.mem__bit__position.as_ref()
    }

    /// Sets the value of MEM_CARD
    pub fn set_mem__card(&mut self, value: u16) {
        self.mem__card = Some(value);
    }

    /// Gets the value of MEM_CARD
    pub fn get_mem__card(&self) -> Option<&u16> {
        self.mem__card.as_ref()
    }

    /// Sets the value of MEM_COLUMN
    pub fn set_mem__column(&mut self, value: u16) {
        self.mem__column = Some(value);
    }

    /// Gets the value of MEM_COLUMN
    pub fn get_mem__column(&self) -> Option<&u16> {
        self.mem__column.as_ref()
    }

    /// Sets the value of MEM_ERROR_STATUS
    pub fn set_mem__error__status(&mut self, value: u64) {
        self.mem__error__status = Some(value);
    }

    /// Gets the value of MEM_ERROR_STATUS
    pub fn get_mem__error__status(&self) -> Option<&u64> {
        self.mem__error__status.as_ref()
    }

    /// Sets the value of MEM_MODULE
    pub fn set_mem__module(&mut self, value: u16) {
        self.mem__module = Some(value);
    }

    /// Gets the value of MEM_MODULE
    pub fn get_mem__module(&self) -> Option<&u16> {
        self.mem__module.as_ref()
    }

    /// Sets the value of MEM_NODE
    pub fn set_mem__node(&mut self, value: u16) {
        self.mem__node = Some(value);
    }

    /// Gets the value of MEM_NODE
    pub fn get_mem__node(&self) -> Option<&u16> {
        self.mem__node.as_ref()
    }

    /// Sets the value of MEM_PHYSICAL_ADDR
    pub fn set_mem__physical__addr(&mut self, value: u64) {
        self.mem__physical__addr = Some(value);
    }

    /// Gets the value of MEM_PHYSICAL_ADDR
    pub fn get_mem__physical__addr(&self) -> Option<&u64> {
        self.mem__physical__addr.as_ref()
    }

    /// Sets the value of MEM_PHYSICAL_MASK
    pub fn set_mem__physical__mask(&mut self, value: u64) {
        self.mem__physical__mask = Some(value);
    }

    /// Gets the value of MEM_PHYSICAL_MASK
    pub fn get_mem__physical__mask(&self) -> Option<&u64> {
        self.mem__physical__mask.as_ref()
    }

    /// Sets the value of MEM_ROW
    pub fn set_mem__row(&mut self, value: u16) {
        self.mem__row = Some(value);
    }

    /// Gets the value of MEM_ROW
    pub fn get_mem__row(&self) -> Option<&u16> {
        self.mem__row.as_ref()
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

    /// Sets the value of REQUESTOR_ID
    pub fn set_requestor__id(&mut self, value: u64) {
        self.requestor__id = Some(value);
    }

    /// Gets the value of REQUESTOR_ID
    pub fn get_requestor__id(&self) -> Option<&u64> {
        self.requestor__id.as_ref()
    }

    /// Sets the value of RESPONDER_ID
    pub fn set_responder__id(&mut self, value: u64) {
        self.responder__id = Some(value);
    }

    /// Gets the value of RESPONDER_ID
    pub fn get_responder__id(&self) -> Option<&u64> {
        self.responder__id.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of TARGET_ID
    pub fn set_target__id(&mut self, value: u64) {
        self.target__id = Some(value);
    }

    /// Gets the value of TARGET_ID
    pub fn get_target__id(&self) -> Option<&u64> {
        self.target__id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: MemoryError_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&MemoryError_Type> {
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

    /// Sets the value of xMEM_DEVICE
    pub fn set_x_mem__device(&mut self, value: u16) {
        self.x_mem__device = Some(value);
    }

    /// Gets the value of xMEM_DEVICE
    pub fn get_x_mem__device(&self) -> Option<&u16> {
        self.x_mem__device.as_ref()
    }
}


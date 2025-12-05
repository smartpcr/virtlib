// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMCAEvent_CPUError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMCAEvent_CPUError {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AdditionalErrors")]
    pub additional_errors: Option<u32>,

/// 
    #[serde(rename = "BusSev")]
    pub bus_sev: Option<u32>,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<u32>,

/// 
    #[serde(rename = "CacheMesi")]
    pub cache_mesi: Option<u32>,

/// 
    #[serde(rename = "CacheOp")]
    pub cache_op: Option<u32>,

/// 
    #[serde(rename = "Cpu")]
    pub cpu: Option<u32>,

/// 
    #[serde(rename = "ErrorSeverity")]
    pub error_severity: Option<CPUError_ErrorSeverity>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u32>,

/// 
    #[serde(rename = "LogToEventlog")]
    pub log_to_eventlog: Option<u32>,

/// 
    #[serde(rename = "MajorErrorType")]
    pub major_error_type: Option<CPUError_MajorErrorType>,

/// 
    #[serde(rename = "MSArrayId")]
    pub msarray_id: Option<u32>,

/// 
    #[serde(rename = "MSIndex")]
    pub msindex: Option<u32>,

/// 
    #[serde(rename = "MSOp")]
    pub msop: Option<u32>,

/// 
    #[serde(rename = "MSSid")]
    pub mssid: Option<u32>,

/// 
    #[serde(rename = "RawRecord")]
    pub raw_record: Vec<u8>,

/// 
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,

/// 
    #[serde(rename = "RegFileId")]
    pub reg_file_id: Option<u32>,

/// 
    #[serde(rename = "RegFileOp")]
    pub reg_file_op: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "TLBOp")]
    pub tlbop: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<CPUError_Type>,
}

impl MSMCAEvent_CPUError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            additional_errors: None,
            bus_sev: None,
            bus_type: None,
            cache_mesi: None,
            cache_op: None,
            cpu: None,
            error_severity: None,
            instance_name: None,
            level: None,
            log_to_eventlog: None,
            major_error_type: None,
            msarray_id: None,
            msindex: None,
            msop: None,
            mssid: None,
            raw_record: Vec::new(),
            record_id: None,
            reg_file_id: None,
            reg_file_op: None,
            size: None,
            tlbop: None,
            type: None,
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

    /// Sets the value of BusSev
    pub fn set_bus_sev(&mut self, value: u32) {
        self.bus_sev = Some(value);
    }

    /// Gets the value of BusSev
    pub fn get_bus_sev(&self) -> Option<&u32> {
        self.bus_sev.as_ref()
    }

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: u32) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u32> {
        self.bus_type.as_ref()
    }

    /// Sets the value of CacheMesi
    pub fn set_cache_mesi(&mut self, value: u32) {
        self.cache_mesi = Some(value);
    }

    /// Gets the value of CacheMesi
    pub fn get_cache_mesi(&self) -> Option<&u32> {
        self.cache_mesi.as_ref()
    }

    /// Sets the value of CacheOp
    pub fn set_cache_op(&mut self, value: u32) {
        self.cache_op = Some(value);
    }

    /// Gets the value of CacheOp
    pub fn get_cache_op(&self) -> Option<&u32> {
        self.cache_op.as_ref()
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
    pub fn set_error_severity(&mut self, value: CPUError_ErrorSeverity) {
        self.error_severity = Some(value);
    }

    /// Gets the value of ErrorSeverity
    pub fn get_error_severity(&self) -> Option<&CPUError_ErrorSeverity> {
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

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u32) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u32> {
        self.level.as_ref()
    }

    /// Sets the value of LogToEventlog
    pub fn set_log_to_eventlog(&mut self, value: u32) {
        self.log_to_eventlog = Some(value);
    }

    /// Gets the value of LogToEventlog
    pub fn get_log_to_eventlog(&self) -> Option<&u32> {
        self.log_to_eventlog.as_ref()
    }

    /// Sets the value of MajorErrorType
    pub fn set_major_error_type(&mut self, value: CPUError_MajorErrorType) {
        self.major_error_type = Some(value);
    }

    /// Gets the value of MajorErrorType
    pub fn get_major_error_type(&self) -> Option<&CPUError_MajorErrorType> {
        self.major_error_type.as_ref()
    }

    /// Sets the value of MSArrayId
    pub fn set_msarray_id(&mut self, value: u32) {
        self.msarray_id = Some(value);
    }

    /// Gets the value of MSArrayId
    pub fn get_msarray_id(&self) -> Option<&u32> {
        self.msarray_id.as_ref()
    }

    /// Sets the value of MSIndex
    pub fn set_msindex(&mut self, value: u32) {
        self.msindex = Some(value);
    }

    /// Gets the value of MSIndex
    pub fn get_msindex(&self) -> Option<&u32> {
        self.msindex.as_ref()
    }

    /// Sets the value of MSOp
    pub fn set_msop(&mut self, value: u32) {
        self.msop = Some(value);
    }

    /// Gets the value of MSOp
    pub fn get_msop(&self) -> Option<&u32> {
        self.msop.as_ref()
    }

    /// Sets the value of MSSid
    pub fn set_mssid(&mut self, value: u32) {
        self.mssid = Some(value);
    }

    /// Gets the value of MSSid
    pub fn get_mssid(&self) -> Option<&u32> {
        self.mssid.as_ref()
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

    /// Sets the value of RegFileId
    pub fn set_reg_file_id(&mut self, value: u32) {
        self.reg_file_id = Some(value);
    }

    /// Gets the value of RegFileId
    pub fn get_reg_file_id(&self) -> Option<&u32> {
        self.reg_file_id.as_ref()
    }

    /// Sets the value of RegFileOp
    pub fn set_reg_file_op(&mut self, value: u32) {
        self.reg_file_op = Some(value);
    }

    /// Gets the value of RegFileOp
    pub fn get_reg_file_op(&self) -> Option<&u32> {
        self.reg_file_op.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of TLBOp
    pub fn set_tlbop(&mut self, value: u32) {
        self.tlbop = Some(value);
    }

    /// Gets the value of TLBOp
    pub fn get_tlbop(&self) -> Option<&u32> {
        self.tlbop.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: CPUError_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&CPUError_Type> {
        self.type.as_ref()
    }
}


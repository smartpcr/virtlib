// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSMCAEvent_PlatformSpecificError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSMCAEvent_PlatformSpecificError {
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
    pub error_severity: Option<PlatformSpecificError_ErrorSeverity>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LogToEventlog")]
    pub log_to_eventlog: Option<u32>,

/// 
    #[serde(rename = "OEM_COMPONENT_ID")]
    pub oem__component__id: Vec<u8>,

/// 
    #[serde(rename = "PLATFORM_BUS_SPECIFIC_DATA")]
    pub platform__bus__specific__data: Option<u64>,

/// 
    #[serde(rename = "PLATFORM_ERROR_STATUS")]
    pub platform__error__status: Option<u64>,

/// 
    #[serde(rename = "PLATFORM_REQUESTOR_ID")]
    pub platform__requestor__id: Option<u64>,

/// 
    #[serde(rename = "PLATFORM_RESPONDER_ID")]
    pub platform__responder__id: Option<u64>,

/// 
    #[serde(rename = "PLATFORM_TARGET_ID")]
    pub platform__target__id: Option<u64>,

/// 
    #[serde(rename = "RawRecord")]
    pub raw_record: Vec<u8>,

/// 
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<PlatformSpecificError_Type>,

/// 
    #[serde(rename = "VALIDATION_BITS")]
    pub validation__bits: Option<u64>,
}

impl MSMCAEvent_PlatformSpecificError {
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
            oem__component__id: Vec::new(),
            platform__bus__specific__data: None,
            platform__error__status: None,
            platform__requestor__id: None,
            platform__responder__id: None,
            platform__target__id: None,
            raw_record: Vec::new(),
            record_id: None,
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
    pub fn set_error_severity(&mut self, value: PlatformSpecificError_ErrorSeverity) {
        self.error_severity = Some(value);
    }

    /// Gets the value of ErrorSeverity
    pub fn get_error_severity(&self) -> Option<&PlatformSpecificError_ErrorSeverity> {
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

    /// Sets the value of OEM_COMPONENT_ID
    pub fn set_oem__component__id(&mut self, value: Vec<u8>) {
        self.oem__component__id = value;
    }

    /// Gets the value of OEM_COMPONENT_ID
    pub fn get_oem__component__id(&self) -> &Vec<u8> {
        &self.oem__component__id
    }

    /// Sets the value of PLATFORM_BUS_SPECIFIC_DATA
    pub fn set_platform__bus__specific__data(&mut self, value: u64) {
        self.platform__bus__specific__data = Some(value);
    }

    /// Gets the value of PLATFORM_BUS_SPECIFIC_DATA
    pub fn get_platform__bus__specific__data(&self) -> Option<&u64> {
        self.platform__bus__specific__data.as_ref()
    }

    /// Sets the value of PLATFORM_ERROR_STATUS
    pub fn set_platform__error__status(&mut self, value: u64) {
        self.platform__error__status = Some(value);
    }

    /// Gets the value of PLATFORM_ERROR_STATUS
    pub fn get_platform__error__status(&self) -> Option<&u64> {
        self.platform__error__status.as_ref()
    }

    /// Sets the value of PLATFORM_REQUESTOR_ID
    pub fn set_platform__requestor__id(&mut self, value: u64) {
        self.platform__requestor__id = Some(value);
    }

    /// Gets the value of PLATFORM_REQUESTOR_ID
    pub fn get_platform__requestor__id(&self) -> Option<&u64> {
        self.platform__requestor__id.as_ref()
    }

    /// Sets the value of PLATFORM_RESPONDER_ID
    pub fn set_platform__responder__id(&mut self, value: u64) {
        self.platform__responder__id = Some(value);
    }

    /// Gets the value of PLATFORM_RESPONDER_ID
    pub fn get_platform__responder__id(&self) -> Option<&u64> {
        self.platform__responder__id.as_ref()
    }

    /// Sets the value of PLATFORM_TARGET_ID
    pub fn set_platform__target__id(&mut self, value: u64) {
        self.platform__target__id = Some(value);
    }

    /// Gets the value of PLATFORM_TARGET_ID
    pub fn get_platform__target__id(&self) -> Option<&u64> {
        self.platform__target__id.as_ref()
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

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: PlatformSpecificError_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&PlatformSpecificError_Type> {
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


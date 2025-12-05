// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_InitiatorInstanceStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_InitiatorInstanceStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "SessionConnectionTimeoutErrorCount")]
    pub session_connection_timeout_error_count: Option<u32>,

/// 
    #[serde(rename = "SessionDigestErrorCount")]
    pub session_digest_error_count: Option<u32>,

/// 
    #[serde(rename = "SessionFailureCount")]
    pub session_failure_count: Option<u32>,

/// 
    #[serde(rename = "SessionFormatErrorCount")]
    pub session_format_error_count: Option<u32>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,
}

impl MSiSCSI_InitiatorInstanceStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            instance_name: None,
            session_connection_timeout_error_count: None,
            session_digest_error_count: None,
            session_failure_count: None,
            session_format_error_count: None,
            unique_adapter_id: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of SessionConnectionTimeoutErrorCount
    pub fn set_session_connection_timeout_error_count(&mut self, value: u32) {
        self.session_connection_timeout_error_count = Some(value);
    }

    /// Gets the value of SessionConnectionTimeoutErrorCount
    pub fn get_session_connection_timeout_error_count(&self) -> Option<&u32> {
        self.session_connection_timeout_error_count.as_ref()
    }

    /// Sets the value of SessionDigestErrorCount
    pub fn set_session_digest_error_count(&mut self, value: u32) {
        self.session_digest_error_count = Some(value);
    }

    /// Gets the value of SessionDigestErrorCount
    pub fn get_session_digest_error_count(&self) -> Option<&u32> {
        self.session_digest_error_count.as_ref()
    }

    /// Sets the value of SessionFailureCount
    pub fn set_session_failure_count(&mut self, value: u32) {
        self.session_failure_count = Some(value);
    }

    /// Gets the value of SessionFailureCount
    pub fn get_session_failure_count(&self) -> Option<&u32> {
        self.session_failure_count.as_ref()
    }

    /// Sets the value of SessionFormatErrorCount
    pub fn set_session_format_error_count(&mut self, value: u32) {
        self.session_format_error_count = Some(value);
    }

    /// Gets the value of SessionFormatErrorCount
    pub fn get_session_format_error_count(&self) -> Option<&u32> {
        self.session_format_error_count.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }
}


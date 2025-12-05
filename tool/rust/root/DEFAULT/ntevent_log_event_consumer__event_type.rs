// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NTEventLogEventConsumer_EventType
//////////////////////////////////////////////

/// NTEventLogEventConsumer_EventType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NTEventLogEventConsumer_EventType {
    /// Success
    #[serde(rename = "Success")]
    Success = 0,
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Information
    #[serde(rename = "Information")]
    Information = 4,
    /// Audit_Success
    #[serde(rename = "Audit_Success")]
    AuditSuccess = 8,
    /// Audit_Failure
    #[serde(rename = "Audit_Failure")]
    AuditFailure = 16,
}

impl Default for NTEventLogEventConsumer_EventType {
    fn default() -> Self {
        Self::Success
    }
}


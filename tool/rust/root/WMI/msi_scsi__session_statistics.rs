// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_SessionStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_SessionStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesSent")]
    pub bytes_sent: Option<u64>,

/// 
    #[serde(rename = "ConnectionTimeoutErrors")]
    pub connection_timeout_errors: Option<u64>,

/// 
    #[serde(rename = "DigestErrors")]
    pub digest_errors: Option<u64>,

/// 
    #[serde(rename = "FormatErrors")]
    pub format_errors: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Name of the iSCSI Target
    #[serde(rename = "iSCSIName")]
    pub i_scsiname: Option<String>,

/// 
    #[serde(rename = "PDUCommandsSent")]
    pub pducommands_sent: Option<u64>,

/// 
    #[serde(rename = "PDUResponsesReceived")]
    pub pduresponses_received: Option<u64>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,

/// 
    #[serde(rename = "USID")]
    pub usid: Option<u64>,
}

impl MSiSCSI_SessionStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            bytes_received: None,
            bytes_sent: None,
            connection_timeout_errors: None,
            digest_errors: None,
            format_errors: None,
            instance_name: None,
            i_scsiname: None,
            pducommands_sent: None,
            pduresponses_received: None,
            unique_adapter_id: None,
            usid: None,
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

    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesSent
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.bytes_sent = Some(value);
    }

    /// Gets the value of BytesSent
    pub fn get_bytes_sent(&self) -> Option<&u64> {
        self.bytes_sent.as_ref()
    }

    /// Sets the value of ConnectionTimeoutErrors
    pub fn set_connection_timeout_errors(&mut self, value: u64) {
        self.connection_timeout_errors = Some(value);
    }

    /// Gets the value of ConnectionTimeoutErrors
    pub fn get_connection_timeout_errors(&self) -> Option<&u64> {
        self.connection_timeout_errors.as_ref()
    }

    /// Sets the value of DigestErrors
    pub fn set_digest_errors(&mut self, value: u64) {
        self.digest_errors = Some(value);
    }

    /// Gets the value of DigestErrors
    pub fn get_digest_errors(&self) -> Option<&u64> {
        self.digest_errors.as_ref()
    }

    /// Sets the value of FormatErrors
    pub fn set_format_errors(&mut self, value: u64) {
        self.format_errors = Some(value);
    }

    /// Gets the value of FormatErrors
    pub fn get_format_errors(&self) -> Option<&u64> {
        self.format_errors.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of iSCSIName
    pub fn set_i_scsiname(&mut self, value: String) {
        self.i_scsiname = Some(value);
    }

    /// Gets the value of iSCSIName
    pub fn get_i_scsiname(&self) -> Option<&String> {
        self.i_scsiname.as_ref()
    }

    /// Sets the value of PDUCommandsSent
    pub fn set_pducommands_sent(&mut self, value: u64) {
        self.pducommands_sent = Some(value);
    }

    /// Gets the value of PDUCommandsSent
    pub fn get_pducommands_sent(&self) -> Option<&u64> {
        self.pducommands_sent.as_ref()
    }

    /// Sets the value of PDUResponsesReceived
    pub fn set_pduresponses_received(&mut self, value: u64) {
        self.pduresponses_received = Some(value);
    }

    /// Gets the value of PDUResponsesReceived
    pub fn get_pduresponses_received(&self) -> Option<&u64> {
        self.pduresponses_received.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }

    /// Sets the value of USID
    pub fn set_usid(&mut self, value: u64) {
        self.usid = Some(value);
    }

    /// Gets the value of USID
    pub fn get_usid(&self) -> Option<&u64> {
        self.usid.as_ref()
    }
}


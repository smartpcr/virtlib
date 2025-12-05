// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_NetworkConnectionDiagnosticSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_NetworkConnectionDiagnosticSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "IsolationId")]
    pub isolation_id: Option<u32>,

/// 
    #[serde(rename = "IsSender")]
    pub is_sender: Option<bool>,

/// 
    #[serde(rename = "PayloadSize")]
    pub payload_size: Option<u32>,

/// 
    #[serde(rename = "ReceiverIP")]
    pub receiver_ip: Option<String>,

/// 
    #[serde(rename = "ReceiverMac")]
    pub receiver_mac: Option<String>,

/// 
    #[serde(rename = "SenderIP")]
    pub sender_ip: Option<String>,

/// 
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<u32>,
}

impl Msvm_NetworkConnectionDiagnosticSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            isolation_id: None,
            is_sender: None,
            payload_size: None,
            receiver_ip: None,
            receiver_mac: None,
            sender_ip: None,
            sequence_number: None,
        }
    }


    /// Sets the value of IsolationId
    pub fn set_isolation_id(&mut self, value: u32) {
        self.isolation_id = Some(value);
    }

    /// Gets the value of IsolationId
    pub fn get_isolation_id(&self) -> Option<&u32> {
        self.isolation_id.as_ref()
    }

    /// Sets the value of IsSender
    pub fn set_is_sender(&mut self, value: bool) {
        self.is_sender = Some(value);
    }

    /// Gets the value of IsSender
    pub fn get_is_sender(&self) -> Option<&bool> {
        self.is_sender.as_ref()
    }

    /// Sets the value of PayloadSize
    pub fn set_payload_size(&mut self, value: u32) {
        self.payload_size = Some(value);
    }

    /// Gets the value of PayloadSize
    pub fn get_payload_size(&self) -> Option<&u32> {
        self.payload_size.as_ref()
    }

    /// Sets the value of ReceiverIP
    pub fn set_receiver_ip(&mut self, value: String) {
        self.receiver_ip = Some(value);
    }

    /// Gets the value of ReceiverIP
    pub fn get_receiver_ip(&self) -> Option<&String> {
        self.receiver_ip.as_ref()
    }

    /// Sets the value of ReceiverMac
    pub fn set_receiver_mac(&mut self, value: String) {
        self.receiver_mac = Some(value);
    }

    /// Gets the value of ReceiverMac
    pub fn get_receiver_mac(&self) -> Option<&String> {
        self.receiver_mac.as_ref()
    }

    /// Sets the value of SenderIP
    pub fn set_sender_ip(&mut self, value: String) {
        self.sender_ip = Some(value);
    }

    /// Gets the value of SenderIP
    pub fn get_sender_ip(&self) -> Option<&String> {
        self.sender_ip.as_ref()
    }

    /// Sets the value of SequenceNumber
    pub fn set_sequence_number(&mut self, value: u32) {
        self.sequence_number = Some(value);
    }

    /// Gets the value of SequenceNumber
    pub fn get_sequence_number(&self) -> Option<&u32> {
        self.sequence_number.as_ref()
    }
}


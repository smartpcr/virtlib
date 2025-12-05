// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_tapisrv_Telephony struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_tapisrv_Telephony {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ActiveLines")]
    pub active_lines: Option<u32>,

/// 
    #[serde(rename = "ActiveTelephones")]
    pub active_telephones: Option<u32>,

/// 
    #[serde(rename = "ClientApps")]
    pub client_apps: Option<u32>,

/// 
    #[serde(rename = "CurrentIncomingCalls")]
    pub current_incoming_calls: Option<u32>,

/// 
    #[serde(rename = "CurrentOutgoingCalls")]
    pub current_outgoing_calls: Option<u32>,

/// 
    #[serde(rename = "IncomingCallsPersec")]
    pub incoming_calls_persec: Option<u32>,

/// 
    #[serde(rename = "Lines")]
    pub lines: Option<u32>,

/// 
    #[serde(rename = "OutgoingCallsPersec")]
    pub outgoing_calls_persec: Option<u32>,

/// 
    #[serde(rename = "TelephoneDevices")]
    pub telephone_devices: Option<u32>,
}

impl Win32_PerfRawData_tapisrv_Telephony {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active_lines: None,
            active_telephones: None,
            client_apps: None,
            current_incoming_calls: None,
            current_outgoing_calls: None,
            incoming_calls_persec: None,
            lines: None,
            outgoing_calls_persec: None,
            telephone_devices: None,
        }
    }


    /// Sets the value of ActiveLines
    pub fn set_active_lines(&mut self, value: u32) {
        self.active_lines = Some(value);
    }

    /// Gets the value of ActiveLines
    pub fn get_active_lines(&self) -> Option<&u32> {
        self.active_lines.as_ref()
    }

    /// Sets the value of ActiveTelephones
    pub fn set_active_telephones(&mut self, value: u32) {
        self.active_telephones = Some(value);
    }

    /// Gets the value of ActiveTelephones
    pub fn get_active_telephones(&self) -> Option<&u32> {
        self.active_telephones.as_ref()
    }

    /// Sets the value of ClientApps
    pub fn set_client_apps(&mut self, value: u32) {
        self.client_apps = Some(value);
    }

    /// Gets the value of ClientApps
    pub fn get_client_apps(&self) -> Option<&u32> {
        self.client_apps.as_ref()
    }

    /// Sets the value of CurrentIncomingCalls
    pub fn set_current_incoming_calls(&mut self, value: u32) {
        self.current_incoming_calls = Some(value);
    }

    /// Gets the value of CurrentIncomingCalls
    pub fn get_current_incoming_calls(&self) -> Option<&u32> {
        self.current_incoming_calls.as_ref()
    }

    /// Sets the value of CurrentOutgoingCalls
    pub fn set_current_outgoing_calls(&mut self, value: u32) {
        self.current_outgoing_calls = Some(value);
    }

    /// Gets the value of CurrentOutgoingCalls
    pub fn get_current_outgoing_calls(&self) -> Option<&u32> {
        self.current_outgoing_calls.as_ref()
    }

    /// Sets the value of IncomingCallsPersec
    pub fn set_incoming_calls_persec(&mut self, value: u32) {
        self.incoming_calls_persec = Some(value);
    }

    /// Gets the value of IncomingCallsPersec
    pub fn get_incoming_calls_persec(&self) -> Option<&u32> {
        self.incoming_calls_persec.as_ref()
    }

    /// Sets the value of Lines
    pub fn set_lines(&mut self, value: u32) {
        self.lines = Some(value);
    }

    /// Gets the value of Lines
    pub fn get_lines(&self) -> Option<&u32> {
        self.lines.as_ref()
    }

    /// Sets the value of OutgoingCallsPersec
    pub fn set_outgoing_calls_persec(&mut self, value: u32) {
        self.outgoing_calls_persec = Some(value);
    }

    /// Gets the value of OutgoingCallsPersec
    pub fn get_outgoing_calls_persec(&self) -> Option<&u32> {
        self.outgoing_calls_persec.as_ref()
    }

    /// Sets the value of TelephoneDevices
    pub fn set_telephone_devices(&mut self, value: u32) {
        self.telephone_devices = Some(value);
    }

    /// Gets the value of TelephoneDevices
    pub fn get_telephone_devices(&self) -> Option<&u32> {
        self.telephone_devices.as_ref()
    }
}


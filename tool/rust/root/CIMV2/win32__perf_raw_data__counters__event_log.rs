// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_EventLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_EventLog {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ActiveChannels")]
    pub active_channels: Option<u32>,

/// 
    #[serde(rename = "Activesubscriptions")]
    pub activesubscriptions: Option<u32>,

/// 
    #[serde(rename = "ELFRPCcallsPersec")]
    pub elfrpccalls_persec: Option<u64>,

/// 
    #[serde(rename = "EnabledChannels")]
    pub enabled_channels: Option<u32>,

/// 
    #[serde(rename = "EventfilteroperationsPersec")]
    pub eventfilteroperations_persec: Option<u64>,

/// 
    #[serde(rename = "EventsPersec")]
    pub events_persec: Option<u64>,

/// 
    #[serde(rename = "WEVTRPCcallsPersec")]
    pub wevtrpccalls_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_EventLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active_channels: None,
            activesubscriptions: None,
            elfrpccalls_persec: None,
            enabled_channels: None,
            eventfilteroperations_persec: None,
            events_persec: None,
            wevtrpccalls_persec: None,
        }
    }


    /// Sets the value of ActiveChannels
    pub fn set_active_channels(&mut self, value: u32) {
        self.active_channels = Some(value);
    }

    /// Gets the value of ActiveChannels
    pub fn get_active_channels(&self) -> Option<&u32> {
        self.active_channels.as_ref()
    }

    /// Sets the value of Activesubscriptions
    pub fn set_activesubscriptions(&mut self, value: u32) {
        self.activesubscriptions = Some(value);
    }

    /// Gets the value of Activesubscriptions
    pub fn get_activesubscriptions(&self) -> Option<&u32> {
        self.activesubscriptions.as_ref()
    }

    /// Sets the value of ELFRPCcallsPersec
    pub fn set_elfrpccalls_persec(&mut self, value: u64) {
        self.elfrpccalls_persec = Some(value);
    }

    /// Gets the value of ELFRPCcallsPersec
    pub fn get_elfrpccalls_persec(&self) -> Option<&u64> {
        self.elfrpccalls_persec.as_ref()
    }

    /// Sets the value of EnabledChannels
    pub fn set_enabled_channels(&mut self, value: u32) {
        self.enabled_channels = Some(value);
    }

    /// Gets the value of EnabledChannels
    pub fn get_enabled_channels(&self) -> Option<&u32> {
        self.enabled_channels.as_ref()
    }

    /// Sets the value of EventfilteroperationsPersec
    pub fn set_eventfilteroperations_persec(&mut self, value: u64) {
        self.eventfilteroperations_persec = Some(value);
    }

    /// Gets the value of EventfilteroperationsPersec
    pub fn get_eventfilteroperations_persec(&self) -> Option<&u64> {
        self.eventfilteroperations_persec.as_ref()
    }

    /// Sets the value of EventsPersec
    pub fn set_events_persec(&mut self, value: u64) {
        self.events_persec = Some(value);
    }

    /// Gets the value of EventsPersec
    pub fn get_events_persec(&self) -> Option<&u64> {
        self.events_persec.as_ref()
    }

    /// Sets the value of WEVTRPCcallsPersec
    pub fn set_wevtrpccalls_persec(&mut self, value: u64) {
        self.wevtrpccalls_persec = Some(value);
    }

    /// Gets the value of WEVTRPCcallsPersec
    pub fn get_wevtrpccalls_persec(&self) -> Option<&u64> {
        self.wevtrpccalls_persec.as_ref()
    }
}


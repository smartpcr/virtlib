// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_HyperVVirtualMachineBus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_HyperVVirtualMachineBus {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "InterruptsReceivedPersec")]
    pub interrupts_received_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptsSentPersec")]
    pub interrupts_sent_persec: Option<u64>,

/// 
    #[serde(rename = "ThrottleEvents")]
    pub throttle_events: Option<u64>,
}

impl Win32_PerfRawData_Counters_HyperVVirtualMachineBus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            interrupts_received_persec: None,
            interrupts_sent_persec: None,
            throttle_events: None,
        }
    }


    /// Sets the value of InterruptsReceivedPersec
    pub fn set_interrupts_received_persec(&mut self, value: u64) {
        self.interrupts_received_persec = Some(value);
    }

    /// Gets the value of InterruptsReceivedPersec
    pub fn get_interrupts_received_persec(&self) -> Option<&u64> {
        self.interrupts_received_persec.as_ref()
    }

    /// Sets the value of InterruptsSentPersec
    pub fn set_interrupts_sent_persec(&mut self, value: u64) {
        self.interrupts_sent_persec = Some(value);
    }

    /// Gets the value of InterruptsSentPersec
    pub fn get_interrupts_sent_persec(&self) -> Option<&u64> {
        self.interrupts_sent_persec.as_ref()
    }

    /// Sets the value of ThrottleEvents
    pub fn set_throttle_events(&mut self, value: u64) {
        self.throttle_events = Some(value);
    }

    /// Gets the value of ThrottleEvents
    pub fn get_throttle_events(&self) -> Option<&u64> {
        self.throttle_events.as_ref()
    }
}


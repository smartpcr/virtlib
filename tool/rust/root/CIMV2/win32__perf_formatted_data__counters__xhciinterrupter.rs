// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_XHCIInterrupter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_XHCIInterrupter {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DpcRequeueCount")]
    pub dpc_requeue_count: Option<u32>,

/// 
    #[serde(rename = "DPCsPersec")]
    pub dpcs_persec: Option<u32>,

/// 
    #[serde(rename = "EventRingFullCount")]
    pub event_ring_full_count: Option<u32>,

/// 
    #[serde(rename = "EventsprocessedDPC")]
    pub eventsprocessed_dpc: Option<u64>,

/// 
    #[serde(rename = "InterruptsPersec")]
    pub interrupts_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_XHCIInterrupter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            dpc_requeue_count: None,
            dpcs_persec: None,
            event_ring_full_count: None,
            eventsprocessed_dpc: None,
            interrupts_persec: None,
        }
    }


    /// Sets the value of DpcRequeueCount
    pub fn set_dpc_requeue_count(&mut self, value: u32) {
        self.dpc_requeue_count = Some(value);
    }

    /// Gets the value of DpcRequeueCount
    pub fn get_dpc_requeue_count(&self) -> Option<&u32> {
        self.dpc_requeue_count.as_ref()
    }

    /// Sets the value of DPCsPersec
    pub fn set_dpcs_persec(&mut self, value: u32) {
        self.dpcs_persec = Some(value);
    }

    /// Gets the value of DPCsPersec
    pub fn get_dpcs_persec(&self) -> Option<&u32> {
        self.dpcs_persec.as_ref()
    }

    /// Sets the value of EventRingFullCount
    pub fn set_event_ring_full_count(&mut self, value: u32) {
        self.event_ring_full_count = Some(value);
    }

    /// Gets the value of EventRingFullCount
    pub fn get_event_ring_full_count(&self) -> Option<&u32> {
        self.event_ring_full_count.as_ref()
    }

    /// Sets the value of EventsprocessedDPC
    pub fn set_eventsprocessed_dpc(&mut self, value: u64) {
        self.eventsprocessed_dpc = Some(value);
    }

    /// Gets the value of EventsprocessedDPC
    pub fn get_eventsprocessed_dpc(&self) -> Option<&u64> {
        self.eventsprocessed_dpc.as_ref()
    }

    /// Sets the value of InterruptsPersec
    pub fn set_interrupts_persec(&mut self, value: u32) {
        self.interrupts_persec = Some(value);
    }

    /// Gets the value of InterruptsPersec
    pub fn get_interrupts_persec(&self) -> Option<&u32> {
        self.interrupts_persec.as_ref()
    }
}


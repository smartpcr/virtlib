// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfOS_Processor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfOS_Processor {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "C1TransitionsPersec")]
    pub c1_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "C2TransitionsPersec")]
    pub c2_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "C3TransitionsPersec")]
    pub c3_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "DPCRate")]
    pub dpcrate: Option<u32>,

/// 
    #[serde(rename = "DPCsQueuedPersec")]
    pub dpcs_queued_persec: Option<u32>,

/// 
    #[serde(rename = "InterruptsPersec")]
    pub interrupts_persec: Option<u32>,

/// 
    #[serde(rename = "PercentC1Time")]
    pub percent_c1_time: Option<u64>,

/// 
    #[serde(rename = "PercentC2Time")]
    pub percent_c2_time: Option<u64>,

/// 
    #[serde(rename = "PercentC3Time")]
    pub percent_c3_time: Option<u64>,

/// 
    #[serde(rename = "PercentDPCTime")]
    pub percent_dpctime: Option<u64>,

/// 
    #[serde(rename = "PercentIdleTime")]
    pub percent_idle_time: Option<u64>,

/// 
    #[serde(rename = "PercentInterruptTime")]
    pub percent_interrupt_time: Option<u64>,

/// 
    #[serde(rename = "PercentPrivilegedTime")]
    pub percent_privileged_time: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorTime")]
    pub percent_processor_time: Option<u64>,

/// 
    #[serde(rename = "PercentUserTime")]
    pub percent_user_time: Option<u64>,
}

impl Win32_PerfRawData_PerfOS_Processor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            c1_transitions_persec: None,
            c2_transitions_persec: None,
            c3_transitions_persec: None,
            dpcrate: None,
            dpcs_queued_persec: None,
            interrupts_persec: None,
            percent_c1_time: None,
            percent_c2_time: None,
            percent_c3_time: None,
            percent_dpctime: None,
            percent_idle_time: None,
            percent_interrupt_time: None,
            percent_privileged_time: None,
            percent_processor_time: None,
            percent_user_time: None,
        }
    }


    /// Sets the value of C1TransitionsPersec
    pub fn set_c1_transitions_persec(&mut self, value: u64) {
        self.c1_transitions_persec = Some(value);
    }

    /// Gets the value of C1TransitionsPersec
    pub fn get_c1_transitions_persec(&self) -> Option<&u64> {
        self.c1_transitions_persec.as_ref()
    }

    /// Sets the value of C2TransitionsPersec
    pub fn set_c2_transitions_persec(&mut self, value: u64) {
        self.c2_transitions_persec = Some(value);
    }

    /// Gets the value of C2TransitionsPersec
    pub fn get_c2_transitions_persec(&self) -> Option<&u64> {
        self.c2_transitions_persec.as_ref()
    }

    /// Sets the value of C3TransitionsPersec
    pub fn set_c3_transitions_persec(&mut self, value: u64) {
        self.c3_transitions_persec = Some(value);
    }

    /// Gets the value of C3TransitionsPersec
    pub fn get_c3_transitions_persec(&self) -> Option<&u64> {
        self.c3_transitions_persec.as_ref()
    }

    /// Sets the value of DPCRate
    pub fn set_dpcrate(&mut self, value: u32) {
        self.dpcrate = Some(value);
    }

    /// Gets the value of DPCRate
    pub fn get_dpcrate(&self) -> Option<&u32> {
        self.dpcrate.as_ref()
    }

    /// Sets the value of DPCsQueuedPersec
    pub fn set_dpcs_queued_persec(&mut self, value: u32) {
        self.dpcs_queued_persec = Some(value);
    }

    /// Gets the value of DPCsQueuedPersec
    pub fn get_dpcs_queued_persec(&self) -> Option<&u32> {
        self.dpcs_queued_persec.as_ref()
    }

    /// Sets the value of InterruptsPersec
    pub fn set_interrupts_persec(&mut self, value: u32) {
        self.interrupts_persec = Some(value);
    }

    /// Gets the value of InterruptsPersec
    pub fn get_interrupts_persec(&self) -> Option<&u32> {
        self.interrupts_persec.as_ref()
    }

    /// Sets the value of PercentC1Time
    pub fn set_percent_c1_time(&mut self, value: u64) {
        self.percent_c1_time = Some(value);
    }

    /// Gets the value of PercentC1Time
    pub fn get_percent_c1_time(&self) -> Option<&u64> {
        self.percent_c1_time.as_ref()
    }

    /// Sets the value of PercentC2Time
    pub fn set_percent_c2_time(&mut self, value: u64) {
        self.percent_c2_time = Some(value);
    }

    /// Gets the value of PercentC2Time
    pub fn get_percent_c2_time(&self) -> Option<&u64> {
        self.percent_c2_time.as_ref()
    }

    /// Sets the value of PercentC3Time
    pub fn set_percent_c3_time(&mut self, value: u64) {
        self.percent_c3_time = Some(value);
    }

    /// Gets the value of PercentC3Time
    pub fn get_percent_c3_time(&self) -> Option<&u64> {
        self.percent_c3_time.as_ref()
    }

    /// Sets the value of PercentDPCTime
    pub fn set_percent_dpctime(&mut self, value: u64) {
        self.percent_dpctime = Some(value);
    }

    /// Gets the value of PercentDPCTime
    pub fn get_percent_dpctime(&self) -> Option<&u64> {
        self.percent_dpctime.as_ref()
    }

    /// Sets the value of PercentIdleTime
    pub fn set_percent_idle_time(&mut self, value: u64) {
        self.percent_idle_time = Some(value);
    }

    /// Gets the value of PercentIdleTime
    pub fn get_percent_idle_time(&self) -> Option<&u64> {
        self.percent_idle_time.as_ref()
    }

    /// Sets the value of PercentInterruptTime
    pub fn set_percent_interrupt_time(&mut self, value: u64) {
        self.percent_interrupt_time = Some(value);
    }

    /// Gets the value of PercentInterruptTime
    pub fn get_percent_interrupt_time(&self) -> Option<&u64> {
        self.percent_interrupt_time.as_ref()
    }

    /// Sets the value of PercentPrivilegedTime
    pub fn set_percent_privileged_time(&mut self, value: u64) {
        self.percent_privileged_time = Some(value);
    }

    /// Gets the value of PercentPrivilegedTime
    pub fn get_percent_privileged_time(&self) -> Option<&u64> {
        self.percent_privileged_time.as_ref()
    }

    /// Sets the value of PercentProcessorTime
    pub fn set_percent_processor_time(&mut self, value: u64) {
        self.percent_processor_time = Some(value);
    }

    /// Gets the value of PercentProcessorTime
    pub fn get_percent_processor_time(&self) -> Option<&u64> {
        self.percent_processor_time.as_ref()
    }

    /// Sets the value of PercentUserTime
    pub fn set_percent_user_time(&mut self, value: u64) {
        self.percent_user_time = Some(value);
    }

    /// Gets the value of PercentUserTime
    pub fn get_percent_user_time(&self) -> Option<&u64> {
        self.percent_user_time.as_ref()
    }
}


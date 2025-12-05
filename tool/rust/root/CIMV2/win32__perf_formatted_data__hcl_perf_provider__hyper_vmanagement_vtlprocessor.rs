// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_HclPerfProvider_HyperVManagementVTLProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_HclPerfProvider_HyperVManagementVTLProcessor {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BackingTimerProgrammedPersec")]
    pub backing_timer_programmed_persec: Option<u64>,

/// 
    #[serde(rename = "EarlyTimerPersec")]
    pub early_timer_persec: Option<u64>,

/// 
    #[serde(rename = "InterProcessorInterruptsPersec")]
    pub inter_processor_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "InterProcessorInterruptsSentPersec")]
    pub inter_processor_interrupts_sent_persec: Option<u64>,

/// 
    #[serde(rename = "TimerInterruptsPersec")]
    pub timer_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "TotalInterruptsPersec")]
    pub total_interrupts_persec: Option<u64>,
}

impl Win32_PerfFormattedData_HclPerfProvider_HyperVManagementVTLProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            backing_timer_programmed_persec: None,
            early_timer_persec: None,
            inter_processor_interrupts_persec: None,
            inter_processor_interrupts_sent_persec: None,
            timer_interrupts_persec: None,
            total_interrupts_persec: None,
        }
    }


    /// Sets the value of BackingTimerProgrammedPersec
    pub fn set_backing_timer_programmed_persec(&mut self, value: u64) {
        self.backing_timer_programmed_persec = Some(value);
    }

    /// Gets the value of BackingTimerProgrammedPersec
    pub fn get_backing_timer_programmed_persec(&self) -> Option<&u64> {
        self.backing_timer_programmed_persec.as_ref()
    }

    /// Sets the value of EarlyTimerPersec
    pub fn set_early_timer_persec(&mut self, value: u64) {
        self.early_timer_persec = Some(value);
    }

    /// Gets the value of EarlyTimerPersec
    pub fn get_early_timer_persec(&self) -> Option<&u64> {
        self.early_timer_persec.as_ref()
    }

    /// Sets the value of InterProcessorInterruptsPersec
    pub fn set_inter_processor_interrupts_persec(&mut self, value: u64) {
        self.inter_processor_interrupts_persec = Some(value);
    }

    /// Gets the value of InterProcessorInterruptsPersec
    pub fn get_inter_processor_interrupts_persec(&self) -> Option<&u64> {
        self.inter_processor_interrupts_persec.as_ref()
    }

    /// Sets the value of InterProcessorInterruptsSentPersec
    pub fn set_inter_processor_interrupts_sent_persec(&mut self, value: u64) {
        self.inter_processor_interrupts_sent_persec = Some(value);
    }

    /// Gets the value of InterProcessorInterruptsSentPersec
    pub fn get_inter_processor_interrupts_sent_persec(&self) -> Option<&u64> {
        self.inter_processor_interrupts_sent_persec.as_ref()
    }

    /// Sets the value of TimerInterruptsPersec
    pub fn set_timer_interrupts_persec(&mut self, value: u64) {
        self.timer_interrupts_persec = Some(value);
    }

    /// Gets the value of TimerInterruptsPersec
    pub fn get_timer_interrupts_persec(&self) -> Option<&u64> {
        self.timer_interrupts_persec.as_ref()
    }

    /// Sets the value of TotalInterruptsPersec
    pub fn set_total_interrupts_persec(&mut self, value: u64) {
        self.total_interrupts_persec = Some(value);
    }

    /// Gets the value of TotalInterruptsPersec
    pub fn get_total_interrupts_persec(&self) -> Option<&u64> {
        self.total_interrupts_persec.as_ref()
    }
}


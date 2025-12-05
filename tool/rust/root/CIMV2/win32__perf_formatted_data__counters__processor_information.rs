// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_ProcessorInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_ProcessorInformation {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActualFrequency")]
    pub actual_frequency: Option<u64>,

/// 
    #[serde(rename = "AverageIdleTime")]
    pub average_idle_time: Option<u64>,

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
    #[serde(rename = "ClockInterruptsPersec")]
    pub clock_interrupts_persec: Option<u32>,

/// 
    #[serde(rename = "DPCRate")]
    pub dpcrate: Option<u32>,

/// 
    #[serde(rename = "DPCsQueuedPersec")]
    pub dpcs_queued_persec: Option<u32>,

/// 
    #[serde(rename = "IdleBreakEventsPersec")]
    pub idle_break_events_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptsPersec")]
    pub interrupts_persec: Option<u32>,

/// 
    #[serde(rename = "ParkingStatus")]
    pub parking_status: Option<u32>,

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
    #[serde(rename = "PercentofMaximumFrequency")]
    pub percentof_maximum_frequency: Option<u32>,

/// 
    #[serde(rename = "PercentPerformanceLimit")]
    pub percent_performance_limit: Option<u32>,

/// 
    #[serde(rename = "PercentPriorityTime")]
    pub percent_priority_time: Option<u64>,

/// 
    #[serde(rename = "PercentPrivilegedTime")]
    pub percent_privileged_time: Option<u64>,

/// 
    #[serde(rename = "PercentPrivilegedUtility")]
    pub percent_privileged_utility: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorPerformance")]
    pub percent_processor_performance: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorTime")]
    pub percent_processor_time: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorUtility")]
    pub percent_processor_utility: Option<u64>,

/// 
    #[serde(rename = "PercentUserTime")]
    pub percent_user_time: Option<u64>,

/// 
    #[serde(rename = "PerformanceLimitFlags")]
    pub performance_limit_flags: Option<u32>,

/// 
    #[serde(rename = "ProcessorFrequency")]
    pub processor_frequency: Option<u32>,

/// 
    #[serde(rename = "ProcessorStateFlags")]
    pub processor_state_flags: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_ProcessorInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            actual_frequency: None,
            average_idle_time: None,
            c1_transitions_persec: None,
            c2_transitions_persec: None,
            c3_transitions_persec: None,
            clock_interrupts_persec: None,
            dpcrate: None,
            dpcs_queued_persec: None,
            idle_break_events_persec: None,
            interrupts_persec: None,
            parking_status: None,
            percent_c1_time: None,
            percent_c2_time: None,
            percent_c3_time: None,
            percent_dpctime: None,
            percent_idle_time: None,
            percent_interrupt_time: None,
            percentof_maximum_frequency: None,
            percent_performance_limit: None,
            percent_priority_time: None,
            percent_privileged_time: None,
            percent_privileged_utility: None,
            percent_processor_performance: None,
            percent_processor_time: None,
            percent_processor_utility: None,
            percent_user_time: None,
            performance_limit_flags: None,
            processor_frequency: None,
            processor_state_flags: None,
        }
    }


    /// Sets the value of ActualFrequency
    pub fn set_actual_frequency(&mut self, value: u64) {
        self.actual_frequency = Some(value);
    }

    /// Gets the value of ActualFrequency
    pub fn get_actual_frequency(&self) -> Option<&u64> {
        self.actual_frequency.as_ref()
    }

    /// Sets the value of AverageIdleTime
    pub fn set_average_idle_time(&mut self, value: u64) {
        self.average_idle_time = Some(value);
    }

    /// Gets the value of AverageIdleTime
    pub fn get_average_idle_time(&self) -> Option<&u64> {
        self.average_idle_time.as_ref()
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

    /// Sets the value of ClockInterruptsPersec
    pub fn set_clock_interrupts_persec(&mut self, value: u32) {
        self.clock_interrupts_persec = Some(value);
    }

    /// Gets the value of ClockInterruptsPersec
    pub fn get_clock_interrupts_persec(&self) -> Option<&u32> {
        self.clock_interrupts_persec.as_ref()
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

    /// Sets the value of IdleBreakEventsPersec
    pub fn set_idle_break_events_persec(&mut self, value: u64) {
        self.idle_break_events_persec = Some(value);
    }

    /// Gets the value of IdleBreakEventsPersec
    pub fn get_idle_break_events_persec(&self) -> Option<&u64> {
        self.idle_break_events_persec.as_ref()
    }

    /// Sets the value of InterruptsPersec
    pub fn set_interrupts_persec(&mut self, value: u32) {
        self.interrupts_persec = Some(value);
    }

    /// Gets the value of InterruptsPersec
    pub fn get_interrupts_persec(&self) -> Option<&u32> {
        self.interrupts_persec.as_ref()
    }

    /// Sets the value of ParkingStatus
    pub fn set_parking_status(&mut self, value: u32) {
        self.parking_status = Some(value);
    }

    /// Gets the value of ParkingStatus
    pub fn get_parking_status(&self) -> Option<&u32> {
        self.parking_status.as_ref()
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

    /// Sets the value of PercentofMaximumFrequency
    pub fn set_percentof_maximum_frequency(&mut self, value: u32) {
        self.percentof_maximum_frequency = Some(value);
    }

    /// Gets the value of PercentofMaximumFrequency
    pub fn get_percentof_maximum_frequency(&self) -> Option<&u32> {
        self.percentof_maximum_frequency.as_ref()
    }

    /// Sets the value of PercentPerformanceLimit
    pub fn set_percent_performance_limit(&mut self, value: u32) {
        self.percent_performance_limit = Some(value);
    }

    /// Gets the value of PercentPerformanceLimit
    pub fn get_percent_performance_limit(&self) -> Option<&u32> {
        self.percent_performance_limit.as_ref()
    }

    /// Sets the value of PercentPriorityTime
    pub fn set_percent_priority_time(&mut self, value: u64) {
        self.percent_priority_time = Some(value);
    }

    /// Gets the value of PercentPriorityTime
    pub fn get_percent_priority_time(&self) -> Option<&u64> {
        self.percent_priority_time.as_ref()
    }

    /// Sets the value of PercentPrivilegedTime
    pub fn set_percent_privileged_time(&mut self, value: u64) {
        self.percent_privileged_time = Some(value);
    }

    /// Gets the value of PercentPrivilegedTime
    pub fn get_percent_privileged_time(&self) -> Option<&u64> {
        self.percent_privileged_time.as_ref()
    }

    /// Sets the value of PercentPrivilegedUtility
    pub fn set_percent_privileged_utility(&mut self, value: u64) {
        self.percent_privileged_utility = Some(value);
    }

    /// Gets the value of PercentPrivilegedUtility
    pub fn get_percent_privileged_utility(&self) -> Option<&u64> {
        self.percent_privileged_utility.as_ref()
    }

    /// Sets the value of PercentProcessorPerformance
    pub fn set_percent_processor_performance(&mut self, value: u64) {
        self.percent_processor_performance = Some(value);
    }

    /// Gets the value of PercentProcessorPerformance
    pub fn get_percent_processor_performance(&self) -> Option<&u64> {
        self.percent_processor_performance.as_ref()
    }

    /// Sets the value of PercentProcessorTime
    pub fn set_percent_processor_time(&mut self, value: u64) {
        self.percent_processor_time = Some(value);
    }

    /// Gets the value of PercentProcessorTime
    pub fn get_percent_processor_time(&self) -> Option<&u64> {
        self.percent_processor_time.as_ref()
    }

    /// Sets the value of PercentProcessorUtility
    pub fn set_percent_processor_utility(&mut self, value: u64) {
        self.percent_processor_utility = Some(value);
    }

    /// Gets the value of PercentProcessorUtility
    pub fn get_percent_processor_utility(&self) -> Option<&u64> {
        self.percent_processor_utility.as_ref()
    }

    /// Sets the value of PercentUserTime
    pub fn set_percent_user_time(&mut self, value: u64) {
        self.percent_user_time = Some(value);
    }

    /// Gets the value of PercentUserTime
    pub fn get_percent_user_time(&self) -> Option<&u64> {
        self.percent_user_time.as_ref()
    }

    /// Sets the value of PerformanceLimitFlags
    pub fn set_performance_limit_flags(&mut self, value: u32) {
        self.performance_limit_flags = Some(value);
    }

    /// Gets the value of PerformanceLimitFlags
    pub fn get_performance_limit_flags(&self) -> Option<&u32> {
        self.performance_limit_flags.as_ref()
    }

    /// Sets the value of ProcessorFrequency
    pub fn set_processor_frequency(&mut self, value: u32) {
        self.processor_frequency = Some(value);
    }

    /// Gets the value of ProcessorFrequency
    pub fn get_processor_frequency(&self) -> Option<&u32> {
        self.processor_frequency.as_ref()
    }

    /// Sets the value of ProcessorStateFlags
    pub fn set_processor_state_flags(&mut self, value: u32) {
        self.processor_state_flags = Some(value);
    }

    /// Gets the value of ProcessorStateFlags
    pub fn get_processor_state_flags(&self) -> Option<&u32> {
        self.processor_state_flags.as_ref()
    }
}


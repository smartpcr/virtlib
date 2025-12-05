// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PacketDirectECUtilization struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PacketDirectECUtilization {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BusyWaitIterationsPersec")]
    pub busy_wait_iterations_persec: Option<u32>,

/// 
    #[serde(rename = "IterationsPersec")]
    pub iterations_persec: Option<u32>,

/// 
    #[serde(rename = "PercentBusyWaitingTime")]
    pub percent_busy_waiting_time: Option<u64>,

/// 
    #[serde(rename = "PercentBusyWaitIterations")]
    pub percent_busy_wait_iterations: Option<u32>,

/// 
    #[serde(rename = "PercentIdleTime")]
    pub percent_idle_time: Option<u64>,

/// 
    #[serde(rename = "PercentProcessingTime")]
    pub percent_processing_time: Option<u64>,

/// 
    #[serde(rename = "ProcessorNumber")]
    pub processor_number: Option<u32>,

/// 
    #[serde(rename = "RXQueueCount")]
    pub rxqueue_count: Option<u32>,

/// 
    #[serde(rename = "TotalBusyWaitIterations")]
    pub total_busy_wait_iterations: Option<u64>,

/// 
    #[serde(rename = "TotalIterations")]
    pub total_iterations: Option<u64>,

/// 
    #[serde(rename = "TXQueueCount")]
    pub txqueue_count: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_PacketDirectECUtilization {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            busy_wait_iterations_persec: None,
            iterations_persec: None,
            percent_busy_waiting_time: None,
            percent_busy_wait_iterations: None,
            percent_idle_time: None,
            percent_processing_time: None,
            processor_number: None,
            rxqueue_count: None,
            total_busy_wait_iterations: None,
            total_iterations: None,
            txqueue_count: None,
        }
    }


    /// Sets the value of BusyWaitIterationsPersec
    pub fn set_busy_wait_iterations_persec(&mut self, value: u32) {
        self.busy_wait_iterations_persec = Some(value);
    }

    /// Gets the value of BusyWaitIterationsPersec
    pub fn get_busy_wait_iterations_persec(&self) -> Option<&u32> {
        self.busy_wait_iterations_persec.as_ref()
    }

    /// Sets the value of IterationsPersec
    pub fn set_iterations_persec(&mut self, value: u32) {
        self.iterations_persec = Some(value);
    }

    /// Gets the value of IterationsPersec
    pub fn get_iterations_persec(&self) -> Option<&u32> {
        self.iterations_persec.as_ref()
    }

    /// Sets the value of PercentBusyWaitingTime
    pub fn set_percent_busy_waiting_time(&mut self, value: u64) {
        self.percent_busy_waiting_time = Some(value);
    }

    /// Gets the value of PercentBusyWaitingTime
    pub fn get_percent_busy_waiting_time(&self) -> Option<&u64> {
        self.percent_busy_waiting_time.as_ref()
    }

    /// Sets the value of PercentBusyWaitIterations
    pub fn set_percent_busy_wait_iterations(&mut self, value: u32) {
        self.percent_busy_wait_iterations = Some(value);
    }

    /// Gets the value of PercentBusyWaitIterations
    pub fn get_percent_busy_wait_iterations(&self) -> Option<&u32> {
        self.percent_busy_wait_iterations.as_ref()
    }

    /// Sets the value of PercentIdleTime
    pub fn set_percent_idle_time(&mut self, value: u64) {
        self.percent_idle_time = Some(value);
    }

    /// Gets the value of PercentIdleTime
    pub fn get_percent_idle_time(&self) -> Option<&u64> {
        self.percent_idle_time.as_ref()
    }

    /// Sets the value of PercentProcessingTime
    pub fn set_percent_processing_time(&mut self, value: u64) {
        self.percent_processing_time = Some(value);
    }

    /// Gets the value of PercentProcessingTime
    pub fn get_percent_processing_time(&self) -> Option<&u64> {
        self.percent_processing_time.as_ref()
    }

    /// Sets the value of ProcessorNumber
    pub fn set_processor_number(&mut self, value: u32) {
        self.processor_number = Some(value);
    }

    /// Gets the value of ProcessorNumber
    pub fn get_processor_number(&self) -> Option<&u32> {
        self.processor_number.as_ref()
    }

    /// Sets the value of RXQueueCount
    pub fn set_rxqueue_count(&mut self, value: u32) {
        self.rxqueue_count = Some(value);
    }

    /// Gets the value of RXQueueCount
    pub fn get_rxqueue_count(&self) -> Option<&u32> {
        self.rxqueue_count.as_ref()
    }

    /// Sets the value of TotalBusyWaitIterations
    pub fn set_total_busy_wait_iterations(&mut self, value: u64) {
        self.total_busy_wait_iterations = Some(value);
    }

    /// Gets the value of TotalBusyWaitIterations
    pub fn get_total_busy_wait_iterations(&self) -> Option<&u64> {
        self.total_busy_wait_iterations.as_ref()
    }

    /// Sets the value of TotalIterations
    pub fn set_total_iterations(&mut self, value: u64) {
        self.total_iterations = Some(value);
    }

    /// Gets the value of TotalIterations
    pub fn get_total_iterations(&self) -> Option<&u64> {
        self.total_iterations.as_ref()
    }

    /// Sets the value of TXQueueCount
    pub fn set_txqueue_count(&mut self, value: u32) {
        self.txqueue_count = Some(value);
    }

    /// Gets the value of TXQueueCount
    pub fn get_txqueue_count(&self) -> Option<&u32> {
        self.txqueue_count.as_ref()
    }
}


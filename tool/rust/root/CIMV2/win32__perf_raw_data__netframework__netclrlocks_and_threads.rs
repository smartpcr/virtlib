// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETFramework_NETCLRLocksAndThreads struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETFramework_NETCLRLocksAndThreads {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ContentionRatePersec")]
    pub contention_rate_persec: Option<u32>,

/// 
    #[serde(rename = "CurrentQueueLength")]
    pub current_queue_length: Option<u32>,

/// 
    #[serde(rename = "NumberofcurrentlogicalThreads")]
    pub numberofcurrentlogical_threads: Option<u32>,

/// 
    #[serde(rename = "NumberofcurrentphysicalThreads")]
    pub numberofcurrentphysical_threads: Option<u32>,

/// 
    #[serde(rename = "Numberofcurrentrecognizedthreads")]
    pub numberofcurrentrecognizedthreads: Option<u32>,

/// 
    #[serde(rename = "Numberoftotalrecognizedthreads")]
    pub numberoftotalrecognizedthreads: Option<u32>,

/// 
    #[serde(rename = "QueueLengthPeak")]
    pub queue_length_peak: Option<u32>,

/// 
    #[serde(rename = "QueueLengthPersec")]
    pub queue_length_persec: Option<u32>,

/// 
    #[serde(rename = "rateofrecognizedthreadsPersec")]
    pub rateofrecognizedthreads_persec: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofContentions")]
    pub total_numberof_contentions: Option<u32>,
}

impl Win32_PerfRawData_NETFramework_NETCLRLocksAndThreads {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            contention_rate_persec: None,
            current_queue_length: None,
            numberofcurrentlogical_threads: None,
            numberofcurrentphysical_threads: None,
            numberofcurrentrecognizedthreads: None,
            numberoftotalrecognizedthreads: None,
            queue_length_peak: None,
            queue_length_persec: None,
            rateofrecognizedthreads_persec: None,
            total_numberof_contentions: None,
        }
    }


    /// Sets the value of ContentionRatePersec
    pub fn set_contention_rate_persec(&mut self, value: u32) {
        self.contention_rate_persec = Some(value);
    }

    /// Gets the value of ContentionRatePersec
    pub fn get_contention_rate_persec(&self) -> Option<&u32> {
        self.contention_rate_persec.as_ref()
    }

    /// Sets the value of CurrentQueueLength
    pub fn set_current_queue_length(&mut self, value: u32) {
        self.current_queue_length = Some(value);
    }

    /// Gets the value of CurrentQueueLength
    pub fn get_current_queue_length(&self) -> Option<&u32> {
        self.current_queue_length.as_ref()
    }

    /// Sets the value of NumberofcurrentlogicalThreads
    pub fn set_numberofcurrentlogical_threads(&mut self, value: u32) {
        self.numberofcurrentlogical_threads = Some(value);
    }

    /// Gets the value of NumberofcurrentlogicalThreads
    pub fn get_numberofcurrentlogical_threads(&self) -> Option<&u32> {
        self.numberofcurrentlogical_threads.as_ref()
    }

    /// Sets the value of NumberofcurrentphysicalThreads
    pub fn set_numberofcurrentphysical_threads(&mut self, value: u32) {
        self.numberofcurrentphysical_threads = Some(value);
    }

    /// Gets the value of NumberofcurrentphysicalThreads
    pub fn get_numberofcurrentphysical_threads(&self) -> Option<&u32> {
        self.numberofcurrentphysical_threads.as_ref()
    }

    /// Sets the value of Numberofcurrentrecognizedthreads
    pub fn set_numberofcurrentrecognizedthreads(&mut self, value: u32) {
        self.numberofcurrentrecognizedthreads = Some(value);
    }

    /// Gets the value of Numberofcurrentrecognizedthreads
    pub fn get_numberofcurrentrecognizedthreads(&self) -> Option<&u32> {
        self.numberofcurrentrecognizedthreads.as_ref()
    }

    /// Sets the value of Numberoftotalrecognizedthreads
    pub fn set_numberoftotalrecognizedthreads(&mut self, value: u32) {
        self.numberoftotalrecognizedthreads = Some(value);
    }

    /// Gets the value of Numberoftotalrecognizedthreads
    pub fn get_numberoftotalrecognizedthreads(&self) -> Option<&u32> {
        self.numberoftotalrecognizedthreads.as_ref()
    }

    /// Sets the value of QueueLengthPeak
    pub fn set_queue_length_peak(&mut self, value: u32) {
        self.queue_length_peak = Some(value);
    }

    /// Gets the value of QueueLengthPeak
    pub fn get_queue_length_peak(&self) -> Option<&u32> {
        self.queue_length_peak.as_ref()
    }

    /// Sets the value of QueueLengthPersec
    pub fn set_queue_length_persec(&mut self, value: u32) {
        self.queue_length_persec = Some(value);
    }

    /// Gets the value of QueueLengthPersec
    pub fn get_queue_length_persec(&self) -> Option<&u32> {
        self.queue_length_persec.as_ref()
    }

    /// Sets the value of rateofrecognizedthreadsPersec
    pub fn set_rateofrecognizedthreads_persec(&mut self, value: u32) {
        self.rateofrecognizedthreads_persec = Some(value);
    }

    /// Gets the value of rateofrecognizedthreadsPersec
    pub fn get_rateofrecognizedthreads_persec(&self) -> Option<&u32> {
        self.rateofrecognizedthreads_persec.as_ref()
    }

    /// Sets the value of TotalNumberofContentions
    pub fn set_total_numberof_contentions(&mut self, value: u32) {
        self.total_numberof_contentions = Some(value);
    }

    /// Gets the value of TotalNumberofContentions
    pub fn get_total_numberof_contentions(&self) -> Option<&u32> {
        self.total_numberof_contentions.as_ref()
    }
}


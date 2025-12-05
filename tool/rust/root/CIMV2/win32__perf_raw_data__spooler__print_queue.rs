// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Spooler_PrintQueue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Spooler_PrintQueue {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AddNetworkPrinterCalls")]
    pub add_network_printer_calls: Option<u32>,

/// 
    #[serde(rename = "BytesPrintedPersec")]
    pub bytes_printed_persec: Option<u64>,

/// 
    #[serde(rename = "EnumerateNetworkPrinterCalls")]
    pub enumerate_network_printer_calls: Option<u32>,

/// 
    #[serde(rename = "JobErrors")]
    pub job_errors: Option<u32>,

/// 
    #[serde(rename = "Jobs")]
    pub jobs: Option<u32>,

/// 
    #[serde(rename = "JobsSpooling")]
    pub jobs_spooling: Option<u32>,

/// 
    #[serde(rename = "MaxJobsSpooling")]
    pub max_jobs_spooling: Option<u32>,

/// 
    #[serde(rename = "MaxReferences")]
    pub max_references: Option<u32>,

/// 
    #[serde(rename = "NotReadyErrors")]
    pub not_ready_errors: Option<u32>,

/// 
    #[serde(rename = "OutofPaperErrors")]
    pub outof_paper_errors: Option<u32>,

/// 
    #[serde(rename = "References")]
    pub references: Option<u32>,

/// 
    #[serde(rename = "TotalJobsPrinted")]
    pub total_jobs_printed: Option<u32>,

/// 
    #[serde(rename = "TotalPagesPrinted")]
    pub total_pages_printed: Option<u32>,
}

impl Win32_PerfRawData_Spooler_PrintQueue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            add_network_printer_calls: None,
            bytes_printed_persec: None,
            enumerate_network_printer_calls: None,
            job_errors: None,
            jobs: None,
            jobs_spooling: None,
            max_jobs_spooling: None,
            max_references: None,
            not_ready_errors: None,
            outof_paper_errors: None,
            references: None,
            total_jobs_printed: None,
            total_pages_printed: None,
        }
    }


    /// Sets the value of AddNetworkPrinterCalls
    pub fn set_add_network_printer_calls(&mut self, value: u32) {
        self.add_network_printer_calls = Some(value);
    }

    /// Gets the value of AddNetworkPrinterCalls
    pub fn get_add_network_printer_calls(&self) -> Option<&u32> {
        self.add_network_printer_calls.as_ref()
    }

    /// Sets the value of BytesPrintedPersec
    pub fn set_bytes_printed_persec(&mut self, value: u64) {
        self.bytes_printed_persec = Some(value);
    }

    /// Gets the value of BytesPrintedPersec
    pub fn get_bytes_printed_persec(&self) -> Option<&u64> {
        self.bytes_printed_persec.as_ref()
    }

    /// Sets the value of EnumerateNetworkPrinterCalls
    pub fn set_enumerate_network_printer_calls(&mut self, value: u32) {
        self.enumerate_network_printer_calls = Some(value);
    }

    /// Gets the value of EnumerateNetworkPrinterCalls
    pub fn get_enumerate_network_printer_calls(&self) -> Option<&u32> {
        self.enumerate_network_printer_calls.as_ref()
    }

    /// Sets the value of JobErrors
    pub fn set_job_errors(&mut self, value: u32) {
        self.job_errors = Some(value);
    }

    /// Gets the value of JobErrors
    pub fn get_job_errors(&self) -> Option<&u32> {
        self.job_errors.as_ref()
    }

    /// Sets the value of Jobs
    pub fn set_jobs(&mut self, value: u32) {
        self.jobs = Some(value);
    }

    /// Gets the value of Jobs
    pub fn get_jobs(&self) -> Option<&u32> {
        self.jobs.as_ref()
    }

    /// Sets the value of JobsSpooling
    pub fn set_jobs_spooling(&mut self, value: u32) {
        self.jobs_spooling = Some(value);
    }

    /// Gets the value of JobsSpooling
    pub fn get_jobs_spooling(&self) -> Option<&u32> {
        self.jobs_spooling.as_ref()
    }

    /// Sets the value of MaxJobsSpooling
    pub fn set_max_jobs_spooling(&mut self, value: u32) {
        self.max_jobs_spooling = Some(value);
    }

    /// Gets the value of MaxJobsSpooling
    pub fn get_max_jobs_spooling(&self) -> Option<&u32> {
        self.max_jobs_spooling.as_ref()
    }

    /// Sets the value of MaxReferences
    pub fn set_max_references(&mut self, value: u32) {
        self.max_references = Some(value);
    }

    /// Gets the value of MaxReferences
    pub fn get_max_references(&self) -> Option<&u32> {
        self.max_references.as_ref()
    }

    /// Sets the value of NotReadyErrors
    pub fn set_not_ready_errors(&mut self, value: u32) {
        self.not_ready_errors = Some(value);
    }

    /// Gets the value of NotReadyErrors
    pub fn get_not_ready_errors(&self) -> Option<&u32> {
        self.not_ready_errors.as_ref()
    }

    /// Sets the value of OutofPaperErrors
    pub fn set_outof_paper_errors(&mut self, value: u32) {
        self.outof_paper_errors = Some(value);
    }

    /// Gets the value of OutofPaperErrors
    pub fn get_outof_paper_errors(&self) -> Option<&u32> {
        self.outof_paper_errors.as_ref()
    }

    /// Sets the value of References
    pub fn set_references(&mut self, value: u32) {
        self.references = Some(value);
    }

    /// Gets the value of References
    pub fn get_references(&self) -> Option<&u32> {
        self.references.as_ref()
    }

    /// Sets the value of TotalJobsPrinted
    pub fn set_total_jobs_printed(&mut self, value: u32) {
        self.total_jobs_printed = Some(value);
    }

    /// Gets the value of TotalJobsPrinted
    pub fn get_total_jobs_printed(&self) -> Option<&u32> {
        self.total_jobs_printed.as_ref()
    }

    /// Sets the value of TotalPagesPrinted
    pub fn set_total_pages_printed(&mut self, value: u32) {
        self.total_pages_printed = Some(value);
    }

    /// Gets the value of TotalPagesPrinted
    pub fn get_total_pages_printed(&self) -> Option<&u32> {
        self.total_pages_printed.as_ref()
    }
}


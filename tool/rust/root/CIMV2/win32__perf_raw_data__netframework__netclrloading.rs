// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETFramework_NETCLRLoading struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETFramework_NETCLRLoading {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AssemblySearchLength")]
    pub assembly_search_length: Option<u32>,

/// 
    #[serde(rename = "BytesinLoaderHeap")]
    pub bytesin_loader_heap: Option<u64>,

/// 
    #[serde(rename = "Currentappdomains")]
    pub currentappdomains: Option<u32>,

/// 
    #[serde(rename = "CurrentAssemblies")]
    pub current_assemblies: Option<u32>,

/// 
    #[serde(rename = "CurrentClassesLoaded")]
    pub current_classes_loaded: Option<u32>,

/// 
    #[serde(rename = "PercentTimeLoading")]
    pub percent_time_loading: Option<u64>,

/// 
    #[serde(rename = "Rateofappdomains")]
    pub rateofappdomains: Option<u32>,

/// 
    #[serde(rename = "Rateofappdomainsunloaded")]
    pub rateofappdomainsunloaded: Option<u32>,

/// 
    #[serde(rename = "RateofAssemblies")]
    pub rateof_assemblies: Option<u32>,

/// 
    #[serde(rename = "RateofClassesLoaded")]
    pub rateof_classes_loaded: Option<u32>,

/// 
    #[serde(rename = "RateofLoadFailures")]
    pub rateof_load_failures: Option<u32>,

/// 
    #[serde(rename = "TotalAppdomains")]
    pub total_appdomains: Option<u32>,

/// 
    #[serde(rename = "Totalappdomainsunloaded")]
    pub totalappdomainsunloaded: Option<u32>,

/// 
    #[serde(rename = "TotalAssemblies")]
    pub total_assemblies: Option<u32>,

/// 
    #[serde(rename = "TotalClassesLoaded")]
    pub total_classes_loaded: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofLoadFailures")]
    pub total_numberof_load_failures: Option<u32>,
}

impl Win32_PerfRawData_NETFramework_NETCLRLoading {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            assembly_search_length: None,
            bytesin_loader_heap: None,
            currentappdomains: None,
            current_assemblies: None,
            current_classes_loaded: None,
            percent_time_loading: None,
            rateofappdomains: None,
            rateofappdomainsunloaded: None,
            rateof_assemblies: None,
            rateof_classes_loaded: None,
            rateof_load_failures: None,
            total_appdomains: None,
            totalappdomainsunloaded: None,
            total_assemblies: None,
            total_classes_loaded: None,
            total_numberof_load_failures: None,
        }
    }


    /// Sets the value of AssemblySearchLength
    pub fn set_assembly_search_length(&mut self, value: u32) {
        self.assembly_search_length = Some(value);
    }

    /// Gets the value of AssemblySearchLength
    pub fn get_assembly_search_length(&self) -> Option<&u32> {
        self.assembly_search_length.as_ref()
    }

    /// Sets the value of BytesinLoaderHeap
    pub fn set_bytesin_loader_heap(&mut self, value: u64) {
        self.bytesin_loader_heap = Some(value);
    }

    /// Gets the value of BytesinLoaderHeap
    pub fn get_bytesin_loader_heap(&self) -> Option<&u64> {
        self.bytesin_loader_heap.as_ref()
    }

    /// Sets the value of Currentappdomains
    pub fn set_currentappdomains(&mut self, value: u32) {
        self.currentappdomains = Some(value);
    }

    /// Gets the value of Currentappdomains
    pub fn get_currentappdomains(&self) -> Option<&u32> {
        self.currentappdomains.as_ref()
    }

    /// Sets the value of CurrentAssemblies
    pub fn set_current_assemblies(&mut self, value: u32) {
        self.current_assemblies = Some(value);
    }

    /// Gets the value of CurrentAssemblies
    pub fn get_current_assemblies(&self) -> Option<&u32> {
        self.current_assemblies.as_ref()
    }

    /// Sets the value of CurrentClassesLoaded
    pub fn set_current_classes_loaded(&mut self, value: u32) {
        self.current_classes_loaded = Some(value);
    }

    /// Gets the value of CurrentClassesLoaded
    pub fn get_current_classes_loaded(&self) -> Option<&u32> {
        self.current_classes_loaded.as_ref()
    }

    /// Sets the value of PercentTimeLoading
    pub fn set_percent_time_loading(&mut self, value: u64) {
        self.percent_time_loading = Some(value);
    }

    /// Gets the value of PercentTimeLoading
    pub fn get_percent_time_loading(&self) -> Option<&u64> {
        self.percent_time_loading.as_ref()
    }

    /// Sets the value of Rateofappdomains
    pub fn set_rateofappdomains(&mut self, value: u32) {
        self.rateofappdomains = Some(value);
    }

    /// Gets the value of Rateofappdomains
    pub fn get_rateofappdomains(&self) -> Option<&u32> {
        self.rateofappdomains.as_ref()
    }

    /// Sets the value of Rateofappdomainsunloaded
    pub fn set_rateofappdomainsunloaded(&mut self, value: u32) {
        self.rateofappdomainsunloaded = Some(value);
    }

    /// Gets the value of Rateofappdomainsunloaded
    pub fn get_rateofappdomainsunloaded(&self) -> Option<&u32> {
        self.rateofappdomainsunloaded.as_ref()
    }

    /// Sets the value of RateofAssemblies
    pub fn set_rateof_assemblies(&mut self, value: u32) {
        self.rateof_assemblies = Some(value);
    }

    /// Gets the value of RateofAssemblies
    pub fn get_rateof_assemblies(&self) -> Option<&u32> {
        self.rateof_assemblies.as_ref()
    }

    /// Sets the value of RateofClassesLoaded
    pub fn set_rateof_classes_loaded(&mut self, value: u32) {
        self.rateof_classes_loaded = Some(value);
    }

    /// Gets the value of RateofClassesLoaded
    pub fn get_rateof_classes_loaded(&self) -> Option<&u32> {
        self.rateof_classes_loaded.as_ref()
    }

    /// Sets the value of RateofLoadFailures
    pub fn set_rateof_load_failures(&mut self, value: u32) {
        self.rateof_load_failures = Some(value);
    }

    /// Gets the value of RateofLoadFailures
    pub fn get_rateof_load_failures(&self) -> Option<&u32> {
        self.rateof_load_failures.as_ref()
    }

    /// Sets the value of TotalAppdomains
    pub fn set_total_appdomains(&mut self, value: u32) {
        self.total_appdomains = Some(value);
    }

    /// Gets the value of TotalAppdomains
    pub fn get_total_appdomains(&self) -> Option<&u32> {
        self.total_appdomains.as_ref()
    }

    /// Sets the value of Totalappdomainsunloaded
    pub fn set_totalappdomainsunloaded(&mut self, value: u32) {
        self.totalappdomainsunloaded = Some(value);
    }

    /// Gets the value of Totalappdomainsunloaded
    pub fn get_totalappdomainsunloaded(&self) -> Option<&u32> {
        self.totalappdomainsunloaded.as_ref()
    }

    /// Sets the value of TotalAssemblies
    pub fn set_total_assemblies(&mut self, value: u32) {
        self.total_assemblies = Some(value);
    }

    /// Gets the value of TotalAssemblies
    pub fn get_total_assemblies(&self) -> Option<&u32> {
        self.total_assemblies.as_ref()
    }

    /// Sets the value of TotalClassesLoaded
    pub fn set_total_classes_loaded(&mut self, value: u32) {
        self.total_classes_loaded = Some(value);
    }

    /// Gets the value of TotalClassesLoaded
    pub fn get_total_classes_loaded(&self) -> Option<&u32> {
        self.total_classes_loaded.as_ref()
    }

    /// Sets the value of TotalNumberofLoadFailures
    pub fn set_total_numberof_load_failures(&mut self, value: u32) {
        self.total_numberof_load_failures = Some(value);
    }

    /// Gets the value of TotalNumberofLoadFailures
    pub fn get_total_numberof_load_failures(&self) -> Option<&u32> {
        self.total_numberof_load_failures.as_ref()
    }
}


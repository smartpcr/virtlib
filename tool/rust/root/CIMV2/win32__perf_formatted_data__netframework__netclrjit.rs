// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETFramework_NETCLRJit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETFramework_NETCLRJit {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ILBytesJittedPersec")]
    pub ilbytes_jitted_persec: Option<u32>,

/// 
    #[serde(rename = "NumberofILBytesJitted")]
    pub numberof_ilbytes_jitted: Option<u32>,

/// 
    #[serde(rename = "NumberofMethodsJitted")]
    pub numberof_methods_jitted: Option<u32>,

/// 
    #[serde(rename = "PercentTimeinJit")]
    pub percent_timein_jit: Option<u32>,

/// 
    #[serde(rename = "StandardJitFailures")]
    pub standard_jit_failures: Option<u32>,

/// 
    #[serde(rename = "TotalNumberofILBytesJitted")]
    pub total_numberof_ilbytes_jitted: Option<u32>,
}

impl Win32_PerfFormattedData_NETFramework_NETCLRJit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            ilbytes_jitted_persec: None,
            numberof_ilbytes_jitted: None,
            numberof_methods_jitted: None,
            percent_timein_jit: None,
            standard_jit_failures: None,
            total_numberof_ilbytes_jitted: None,
        }
    }


    /// Sets the value of ILBytesJittedPersec
    pub fn set_ilbytes_jitted_persec(&mut self, value: u32) {
        self.ilbytes_jitted_persec = Some(value);
    }

    /// Gets the value of ILBytesJittedPersec
    pub fn get_ilbytes_jitted_persec(&self) -> Option<&u32> {
        self.ilbytes_jitted_persec.as_ref()
    }

    /// Sets the value of NumberofILBytesJitted
    pub fn set_numberof_ilbytes_jitted(&mut self, value: u32) {
        self.numberof_ilbytes_jitted = Some(value);
    }

    /// Gets the value of NumberofILBytesJitted
    pub fn get_numberof_ilbytes_jitted(&self) -> Option<&u32> {
        self.numberof_ilbytes_jitted.as_ref()
    }

    /// Sets the value of NumberofMethodsJitted
    pub fn set_numberof_methods_jitted(&mut self, value: u32) {
        self.numberof_methods_jitted = Some(value);
    }

    /// Gets the value of NumberofMethodsJitted
    pub fn get_numberof_methods_jitted(&self) -> Option<&u32> {
        self.numberof_methods_jitted.as_ref()
    }

    /// Sets the value of PercentTimeinJit
    pub fn set_percent_timein_jit(&mut self, value: u32) {
        self.percent_timein_jit = Some(value);
    }

    /// Gets the value of PercentTimeinJit
    pub fn get_percent_timein_jit(&self) -> Option<&u32> {
        self.percent_timein_jit.as_ref()
    }

    /// Sets the value of StandardJitFailures
    pub fn set_standard_jit_failures(&mut self, value: u32) {
        self.standard_jit_failures = Some(value);
    }

    /// Gets the value of StandardJitFailures
    pub fn get_standard_jit_failures(&self) -> Option<&u32> {
        self.standard_jit_failures.as_ref()
    }

    /// Sets the value of TotalNumberofILBytesJitted
    pub fn set_total_numberof_ilbytes_jitted(&mut self, value: u32) {
        self.total_numberof_ilbytes_jitted = Some(value);
    }

    /// Gets the value of TotalNumberofILBytesJitted
    pub fn get_total_numberof_ilbytes_jitted(&self) -> Option<&u32> {
        self.total_numberof_ilbytes_jitted.as_ref()
    }
}


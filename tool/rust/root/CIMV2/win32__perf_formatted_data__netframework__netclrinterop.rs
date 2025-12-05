// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETFramework_NETCLRInterop struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETFramework_NETCLRInterop {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "NumberofCCWs")]
    pub numberof_ccws: Option<u32>,

/// 
    #[serde(rename = "Numberofmarshalling")]
    pub numberofmarshalling: Option<u32>,

/// 
    #[serde(rename = "NumberofStubs")]
    pub numberof_stubs: Option<u32>,

/// 
    #[serde(rename = "NumberofTLBexportsPersec")]
    pub numberof_tlbexports_persec: Option<u32>,

/// 
    #[serde(rename = "NumberofTLBimportsPersec")]
    pub numberof_tlbimports_persec: Option<u32>,
}

impl Win32_PerfFormattedData_NETFramework_NETCLRInterop {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            numberof_ccws: None,
            numberofmarshalling: None,
            numberof_stubs: None,
            numberof_tlbexports_persec: None,
            numberof_tlbimports_persec: None,
        }
    }


    /// Sets the value of NumberofCCWs
    pub fn set_numberof_ccws(&mut self, value: u32) {
        self.numberof_ccws = Some(value);
    }

    /// Gets the value of NumberofCCWs
    pub fn get_numberof_ccws(&self) -> Option<&u32> {
        self.numberof_ccws.as_ref()
    }

    /// Sets the value of Numberofmarshalling
    pub fn set_numberofmarshalling(&mut self, value: u32) {
        self.numberofmarshalling = Some(value);
    }

    /// Gets the value of Numberofmarshalling
    pub fn get_numberofmarshalling(&self) -> Option<&u32> {
        self.numberofmarshalling.as_ref()
    }

    /// Sets the value of NumberofStubs
    pub fn set_numberof_stubs(&mut self, value: u32) {
        self.numberof_stubs = Some(value);
    }

    /// Gets the value of NumberofStubs
    pub fn get_numberof_stubs(&self) -> Option<&u32> {
        self.numberof_stubs.as_ref()
    }

    /// Sets the value of NumberofTLBexportsPersec
    pub fn set_numberof_tlbexports_persec(&mut self, value: u32) {
        self.numberof_tlbexports_persec = Some(value);
    }

    /// Gets the value of NumberofTLBexportsPersec
    pub fn get_numberof_tlbexports_persec(&self) -> Option<&u32> {
        self.numberof_tlbexports_persec.as_ref()
    }

    /// Sets the value of NumberofTLBimportsPersec
    pub fn set_numberof_tlbimports_persec(&mut self, value: u32) {
        self.numberof_tlbimports_persec = Some(value);
    }

    /// Gets the value of NumberofTLBimportsPersec
    pub fn get_numberof_tlbimports_persec(&self) -> Option<&u32> {
        self.numberof_tlbimports_persec.as_ref()
    }
}


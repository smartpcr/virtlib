// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETFramework_NETCLRExceptions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETFramework_NETCLRExceptions {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "NumberofExcepsThrown")]
    pub numberof_exceps_thrown: Option<u32>,

/// 
    #[serde(rename = "NumberofExcepsThrownPersec")]
    pub numberof_exceps_thrown_persec: Option<u32>,

/// 
    #[serde(rename = "NumberofFiltersPersec")]
    pub numberof_filters_persec: Option<u32>,

/// 
    #[serde(rename = "NumberofFinallysPersec")]
    pub numberof_finallys_persec: Option<u32>,

/// 
    #[serde(rename = "ThrowToCatchDepthPersec")]
    pub throw_to_catch_depth_persec: Option<u32>,
}

impl Win32_PerfFormattedData_NETFramework_NETCLRExceptions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            numberof_exceps_thrown: None,
            numberof_exceps_thrown_persec: None,
            numberof_filters_persec: None,
            numberof_finallys_persec: None,
            throw_to_catch_depth_persec: None,
        }
    }


    /// Sets the value of NumberofExcepsThrown
    pub fn set_numberof_exceps_thrown(&mut self, value: u32) {
        self.numberof_exceps_thrown = Some(value);
    }

    /// Gets the value of NumberofExcepsThrown
    pub fn get_numberof_exceps_thrown(&self) -> Option<&u32> {
        self.numberof_exceps_thrown.as_ref()
    }

    /// Sets the value of NumberofExcepsThrownPersec
    pub fn set_numberof_exceps_thrown_persec(&mut self, value: u32) {
        self.numberof_exceps_thrown_persec = Some(value);
    }

    /// Gets the value of NumberofExcepsThrownPersec
    pub fn get_numberof_exceps_thrown_persec(&self) -> Option<&u32> {
        self.numberof_exceps_thrown_persec.as_ref()
    }

    /// Sets the value of NumberofFiltersPersec
    pub fn set_numberof_filters_persec(&mut self, value: u32) {
        self.numberof_filters_persec = Some(value);
    }

    /// Gets the value of NumberofFiltersPersec
    pub fn get_numberof_filters_persec(&self) -> Option<&u32> {
        self.numberof_filters_persec.as_ref()
    }

    /// Sets the value of NumberofFinallysPersec
    pub fn set_numberof_finallys_persec(&mut self, value: u32) {
        self.numberof_finallys_persec = Some(value);
    }

    /// Gets the value of NumberofFinallysPersec
    pub fn get_numberof_finallys_persec(&self) -> Option<&u32> {
        self.numberof_finallys_persec.as_ref()
    }

    /// Sets the value of ThrowToCatchDepthPersec
    pub fn set_throw_to_catch_depth_persec(&mut self, value: u32) {
        self.throw_to_catch_depth_persec = Some(value);
    }

    /// Gets the value of ThrowToCatchDepthPersec
    pub fn get_throw_to_catch_depth_persec(&self) -> Option<&u32> {
        self.throw_to_catch_depth_persec.as_ref()
    }
}


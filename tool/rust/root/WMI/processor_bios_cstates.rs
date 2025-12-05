// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorBiosCStates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorBiosCStates {
    #[serde(flatten)]
    pub base: MSProcessorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Cst")]
    pub cst: Option<ProcessorAcpiCst>,

/// 
    #[serde(rename = "CStateVersionInUse")]
    pub cstate_version_in_use: Option<u32>,

/// 
    #[serde(rename = "FadtC2Latency")]
    pub fadt_c2_latency: Option<u16>,

/// 
    #[serde(rename = "FadtC3Latency")]
    pub fadt_c3_latency: Option<u16>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NtNumber")]
    pub nt_number: Option<u32>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u32>,

/// 
    #[serde(rename = "Reserved3")]
    pub reserved3: Option<u64>,
}

impl ProcessorBiosCStates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSProcessorClass::new(),
            active: None,
            cst: None,
            cstate_version_in_use: None,
            fadt_c2_latency: None,
            fadt_c3_latency: None,
            instance_name: None,
            nt_number: None,
            reserved1: None,
            reserved2: None,
            reserved3: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of Cst
    pub fn set_cst(&mut self, value: ProcessorAcpiCst) {
        self.cst = Some(value);
    }

    /// Gets the value of Cst
    pub fn get_cst(&self) -> Option<&ProcessorAcpiCst> {
        self.cst.as_ref()
    }

    /// Sets the value of CStateVersionInUse
    pub fn set_cstate_version_in_use(&mut self, value: u32) {
        self.cstate_version_in_use = Some(value);
    }

    /// Gets the value of CStateVersionInUse
    pub fn get_cstate_version_in_use(&self) -> Option<&u32> {
        self.cstate_version_in_use.as_ref()
    }

    /// Sets the value of FadtC2Latency
    pub fn set_fadt_c2_latency(&mut self, value: u16) {
        self.fadt_c2_latency = Some(value);
    }

    /// Gets the value of FadtC2Latency
    pub fn get_fadt_c2_latency(&self) -> Option<&u16> {
        self.fadt_c2_latency.as_ref()
    }

    /// Sets the value of FadtC3Latency
    pub fn set_fadt_c3_latency(&mut self, value: u16) {
        self.fadt_c3_latency = Some(value);
    }

    /// Gets the value of FadtC3Latency
    pub fn get_fadt_c3_latency(&self) -> Option<&u16> {
        self.fadt_c3_latency.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NtNumber
    pub fn set_nt_number(&mut self, value: u32) {
        self.nt_number = Some(value);
    }

    /// Gets the value of NtNumber
    pub fn get_nt_number(&self) -> Option<&u32> {
        self.nt_number.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u32) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u32> {
        self.reserved2.as_ref()
    }

    /// Sets the value of Reserved3
    pub fn set_reserved3(&mut self, value: u64) {
        self.reserved3 = Some(value);
    }

    /// Gets the value of Reserved3
    pub fn get_reserved3(&self) -> Option<&u64> {
        self.reserved3.as_ref()
    }
}


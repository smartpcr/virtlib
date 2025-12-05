// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ExternalFcPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ExternalFcPort {
    #[serde(flatten)]
    pub base: CIM_FCPort,

/// 
    #[serde(rename = "IsHyperVCapable")]
    pub is_hyper_vcapable: Option<bool>,

/// 
    #[serde(rename = "WWNN")]
    pub wwnn: Option<String>,

/// 
    #[serde(rename = "WWPN")]
    pub wwpn: Option<String>,
}

impl Msvm_ExternalFcPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FCPort::new(),
            is_hyper_vcapable: None,
            wwnn: None,
            wwpn: None,
        }
    }


    /// Sets the value of IsHyperVCapable
    pub fn set_is_hyper_vcapable(&mut self, value: bool) {
        self.is_hyper_vcapable = Some(value);
    }

    /// Gets the value of IsHyperVCapable
    pub fn get_is_hyper_vcapable(&self) -> Option<&bool> {
        self.is_hyper_vcapable.as_ref()
    }

    /// Sets the value of WWNN
    pub fn set_wwnn(&mut self, value: String) {
        self.wwnn = Some(value);
    }

    /// Gets the value of WWNN
    pub fn get_wwnn(&self) -> Option<&String> {
        self.wwnn.as_ref()
    }

    /// Sets the value of WWPN
    pub fn set_wwpn(&mut self, value: String) {
        self.wwpn = Some(value);
    }

    /// Gets the value of WWPN
    pub fn get_wwpn(&self) -> Option<&String> {
        self.wwpn.as_ref()
    }
}


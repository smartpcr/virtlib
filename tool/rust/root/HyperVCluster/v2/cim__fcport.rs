// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FCPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FCPort {
    #[serde(flatten)]
    pub base: CIM_NetworkPort,

/// An array of integers that indicates the Classes of Service that are active. The Active COS is indicated in ActiveCOS.
    #[serde(rename = "ActiveCOS")]
    pub active_cos: Vec<FCPort_ActiveCOS>,

/// An array of integers that indicates the Fibre Channel FC-4 protocols currently running. A list of all supported protocols is indicated in the SupportedFC4Types property.
    #[serde(rename = "ActiveFC4Types")]
    pub active_fc4_types: Vec<FCPort_ActiveFC4Types>,

/// An array of integers that indicates the Fibre Channel Classes of Service that are supported. The active COS are indicated in ActiveCOS.
    #[serde(rename = "SupportedCOS")]
    pub supported_cos: Vec<FCPort_SupportedCOS>,

/// An array of integers that indicates the Fibre Channel FC-4 protocols supported. The protocols that are active and running are indicated in the ActiveFC4Types property.
    #[serde(rename = "SupportedFC4Types")]
    pub supported_fc4_types: Vec<FCPort_SupportedFC4Types>,
}

impl CIM_FCPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkPort::new(),
            active_cos: Vec::new(),
            active_fc4_types: Vec::new(),
            supported_cos: Vec::new(),
            supported_fc4_types: Vec::new(),
        }
    }


    /// Sets the value of ActiveCOS
    pub fn set_active_cos(&mut self, value: Vec<FCPort_ActiveCOS>) {
        self.active_cos = value;
    }

    /// Gets the value of ActiveCOS
    pub fn get_active_cos(&self) -> &Vec<FCPort_ActiveCOS> {
        &self.active_cos
    }

    /// Sets the value of ActiveFC4Types
    pub fn set_active_fc4_types(&mut self, value: Vec<FCPort_ActiveFC4Types>) {
        self.active_fc4_types = value;
    }

    /// Gets the value of ActiveFC4Types
    pub fn get_active_fc4_types(&self) -> &Vec<FCPort_ActiveFC4Types> {
        &self.active_fc4_types
    }

    /// Sets the value of SupportedCOS
    pub fn set_supported_cos(&mut self, value: Vec<FCPort_SupportedCOS>) {
        self.supported_cos = value;
    }

    /// Gets the value of SupportedCOS
    pub fn get_supported_cos(&self) -> &Vec<FCPort_SupportedCOS> {
        &self.supported_cos
    }

    /// Sets the value of SupportedFC4Types
    pub fn set_supported_fc4_types(&mut self, value: Vec<FCPort_SupportedFC4Types>) {
        self.supported_fc4_types = value;
    }

    /// Gets the value of SupportedFC4Types
    pub fn get_supported_fc4_types(&self) -> &Vec<FCPort_SupportedFC4Types> {
        &self.supported_fc4_types
    }
}


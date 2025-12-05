// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IPsecSAEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IPsecSAEndpoint {
    #[serde(flatten)]
    pub base: CIM_SecurityAssociationEndpoint,

/// 
    #[serde(rename = "DFHandling")]
    pub dfhandling: Option<u16>,

/// 
    #[serde(rename = "EncapsulationMode")]
    pub encapsulation_mode: Option<u16>,

/// 
    #[serde(rename = "InboundDirection")]
    pub inbound_direction: Option<bool>,

/// 
    #[serde(rename = "PFSInUse")]
    pub pfsin_use: Option<bool>,

/// 
    #[serde(rename = "SPI")]
    pub spi: Option<u32>,
}

impl CIM_IPsecSAEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SecurityAssociationEndpoint::new(),
            dfhandling: None,
            encapsulation_mode: None,
            inbound_direction: None,
            pfsin_use: None,
            spi: None,
        }
    }


    /// Sets the value of DFHandling
    pub fn set_dfhandling(&mut self, value: u16) {
        self.dfhandling = Some(value);
    }

    /// Gets the value of DFHandling
    pub fn get_dfhandling(&self) -> Option<&u16> {
        self.dfhandling.as_ref()
    }

    /// Sets the value of EncapsulationMode
    pub fn set_encapsulation_mode(&mut self, value: u16) {
        self.encapsulation_mode = Some(value);
    }

    /// Gets the value of EncapsulationMode
    pub fn get_encapsulation_mode(&self) -> Option<&u16> {
        self.encapsulation_mode.as_ref()
    }

    /// Sets the value of InboundDirection
    pub fn set_inbound_direction(&mut self, value: bool) {
        self.inbound_direction = Some(value);
    }

    /// Gets the value of InboundDirection
    pub fn get_inbound_direction(&self) -> Option<&bool> {
        self.inbound_direction.as_ref()
    }

    /// Sets the value of PFSInUse
    pub fn set_pfsin_use(&mut self, value: bool) {
        self.pfsin_use = Some(value);
    }

    /// Gets the value of PFSInUse
    pub fn get_pfsin_use(&self) -> Option<&bool> {
        self.pfsin_use.as_ref()
    }

    /// Sets the value of SPI
    pub fn set_spi(&mut self, value: u32) {
        self.spi = Some(value);
    }

    /// Gets the value of SPI
    pub fn get_spi(&self) -> Option<&u32> {
        self.spi.as_ref()
    }
}


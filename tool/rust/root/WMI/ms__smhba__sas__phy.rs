// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_SAS_PHY struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_SAS_PHY {

/// 
    #[serde(rename = "domainPortWWN")]
    pub domain_port_wwn: Vec<u8>,

/// 
    #[serde(rename = "HardwareMaxLinkRate")]
    pub hardware_max_link_rate: Option<u32>,

/// 
    #[serde(rename = "HardwareMinLinkRate")]
    pub hardware_min_link_rate: Option<u32>,

/// 
    #[serde(rename = "NegotiatedLinkRate")]
    pub negotiated_link_rate: Option<u32>,

/// 
    #[serde(rename = "PhyIdentifier")]
    pub phy_identifier: Option<u8>,

/// 
    #[serde(rename = "ProgrammedMaxLinkRate")]
    pub programmed_max_link_rate: Option<u32>,

/// 
    #[serde(rename = "ProgrammedMinLinkRate")]
    pub programmed_min_link_rate: Option<u32>,
}

impl MS_SMHBA_SAS_PHY {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            domain_port_wwn: Vec::new(),
            hardware_max_link_rate: None,
            hardware_min_link_rate: None,
            negotiated_link_rate: None,
            phy_identifier: None,
            programmed_max_link_rate: None,
            programmed_min_link_rate: None,
        }
    }


    /// Sets the value of domainPortWWN
    pub fn set_domain_port_wwn(&mut self, value: Vec<u8>) {
        self.domain_port_wwn = value;
    }

    /// Gets the value of domainPortWWN
    pub fn get_domain_port_wwn(&self) -> &Vec<u8> {
        &self.domain_port_wwn
    }

    /// Sets the value of HardwareMaxLinkRate
    pub fn set_hardware_max_link_rate(&mut self, value: u32) {
        self.hardware_max_link_rate = Some(value);
    }

    /// Gets the value of HardwareMaxLinkRate
    pub fn get_hardware_max_link_rate(&self) -> Option<&u32> {
        self.hardware_max_link_rate.as_ref()
    }

    /// Sets the value of HardwareMinLinkRate
    pub fn set_hardware_min_link_rate(&mut self, value: u32) {
        self.hardware_min_link_rate = Some(value);
    }

    /// Gets the value of HardwareMinLinkRate
    pub fn get_hardware_min_link_rate(&self) -> Option<&u32> {
        self.hardware_min_link_rate.as_ref()
    }

    /// Sets the value of NegotiatedLinkRate
    pub fn set_negotiated_link_rate(&mut self, value: u32) {
        self.negotiated_link_rate = Some(value);
    }

    /// Gets the value of NegotiatedLinkRate
    pub fn get_negotiated_link_rate(&self) -> Option<&u32> {
        self.negotiated_link_rate.as_ref()
    }

    /// Sets the value of PhyIdentifier
    pub fn set_phy_identifier(&mut self, value: u8) {
        self.phy_identifier = Some(value);
    }

    /// Gets the value of PhyIdentifier
    pub fn get_phy_identifier(&self) -> Option<&u8> {
        self.phy_identifier.as_ref()
    }

    /// Sets the value of ProgrammedMaxLinkRate
    pub fn set_programmed_max_link_rate(&mut self, value: u32) {
        self.programmed_max_link_rate = Some(value);
    }

    /// Gets the value of ProgrammedMaxLinkRate
    pub fn get_programmed_max_link_rate(&self) -> Option<&u32> {
        self.programmed_max_link_rate.as_ref()
    }

    /// Sets the value of ProgrammedMinLinkRate
    pub fn set_programmed_min_link_rate(&mut self, value: u32) {
        self.programmed_min_link_rate = Some(value);
    }

    /// Gets the value of ProgrammedMinLinkRate
    pub fn get_programmed_min_link_rate(&self) -> Option<&u32> {
        self.programmed_min_link_rate.as_ref()
    }
}


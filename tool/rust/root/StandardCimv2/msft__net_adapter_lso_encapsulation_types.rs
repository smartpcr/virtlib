// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterLsoEncapsulationTypes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterLsoEncapsulationTypes {

/// 
    #[serde(rename = "NdisEncapsulationIeee802_3")]
    pub ndis_encapsulation_ieee802_3: Option<bool>,

/// 
    #[serde(rename = "NdisEncapsulationIeee802_3pAndq")]
    pub ndis_encapsulation_ieee802_3p_andq: Option<bool>,

/// 
    #[serde(rename = "NdisEncapsulationIeee802_3PAndQInOob")]
    pub ndis_encapsulation_ieee802_3_pand_qin_oob: Option<bool>,

/// 
    #[serde(rename = "NdisEncapsulationIeeLlcSnapRouted")]
    pub ndis_encapsulation_iee_llc_snap_routed: Option<bool>,

/// 
    #[serde(rename = "NdisEncapsulationNotNull")]
    pub ndis_encapsulation_not_null: Option<bool>,

/// 
    #[serde(rename = "NdisEncapsulationNotSupported")]
    pub ndis_encapsulation_not_supported: Option<bool>,
}

impl MSFT_NetAdapterLsoEncapsulationTypes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ndis_encapsulation_ieee802_3: None,
            ndis_encapsulation_ieee802_3p_andq: None,
            ndis_encapsulation_ieee802_3_pand_qin_oob: None,
            ndis_encapsulation_iee_llc_snap_routed: None,
            ndis_encapsulation_not_null: None,
            ndis_encapsulation_not_supported: None,
        }
    }


    /// Sets the value of NdisEncapsulationIeee802_3
    pub fn set_ndis_encapsulation_ieee802_3(&mut self, value: bool) {
        self.ndis_encapsulation_ieee802_3 = Some(value);
    }

    /// Gets the value of NdisEncapsulationIeee802_3
    pub fn get_ndis_encapsulation_ieee802_3(&self) -> Option<&bool> {
        self.ndis_encapsulation_ieee802_3.as_ref()
    }

    /// Sets the value of NdisEncapsulationIeee802_3pAndq
    pub fn set_ndis_encapsulation_ieee802_3p_andq(&mut self, value: bool) {
        self.ndis_encapsulation_ieee802_3p_andq = Some(value);
    }

    /// Gets the value of NdisEncapsulationIeee802_3pAndq
    pub fn get_ndis_encapsulation_ieee802_3p_andq(&self) -> Option<&bool> {
        self.ndis_encapsulation_ieee802_3p_andq.as_ref()
    }

    /// Sets the value of NdisEncapsulationIeee802_3PAndQInOob
    pub fn set_ndis_encapsulation_ieee802_3_pand_qin_oob(&mut self, value: bool) {
        self.ndis_encapsulation_ieee802_3_pand_qin_oob = Some(value);
    }

    /// Gets the value of NdisEncapsulationIeee802_3PAndQInOob
    pub fn get_ndis_encapsulation_ieee802_3_pand_qin_oob(&self) -> Option<&bool> {
        self.ndis_encapsulation_ieee802_3_pand_qin_oob.as_ref()
    }

    /// Sets the value of NdisEncapsulationIeeLlcSnapRouted
    pub fn set_ndis_encapsulation_iee_llc_snap_routed(&mut self, value: bool) {
        self.ndis_encapsulation_iee_llc_snap_routed = Some(value);
    }

    /// Gets the value of NdisEncapsulationIeeLlcSnapRouted
    pub fn get_ndis_encapsulation_iee_llc_snap_routed(&self) -> Option<&bool> {
        self.ndis_encapsulation_iee_llc_snap_routed.as_ref()
    }

    /// Sets the value of NdisEncapsulationNotNull
    pub fn set_ndis_encapsulation_not_null(&mut self, value: bool) {
        self.ndis_encapsulation_not_null = Some(value);
    }

    /// Gets the value of NdisEncapsulationNotNull
    pub fn get_ndis_encapsulation_not_null(&self) -> Option<&bool> {
        self.ndis_encapsulation_not_null.as_ref()
    }

    /// Sets the value of NdisEncapsulationNotSupported
    pub fn set_ndis_encapsulation_not_supported(&mut self, value: bool) {
        self.ndis_encapsulation_not_supported = Some(value);
    }

    /// Gets the value of NdisEncapsulationNotSupported
    pub fn get_ndis_encapsulation_not_supported(&self) -> Option<&bool> {
        self.ndis_encapsulation_not_supported.as_ref()
    }
}


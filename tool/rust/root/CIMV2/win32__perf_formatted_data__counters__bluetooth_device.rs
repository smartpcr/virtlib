// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_BluetoothDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_BluetoothDevice {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ClassicACLbytesreadPersec")]
    pub classic_aclbytesread_persec: Option<u32>,

/// 
    #[serde(rename = "ClassicACLbyteswrittenPersec")]
    pub classic_aclbyteswritten_persec: Option<u32>,

/// 
    #[serde(rename = "EstablishedLECISCount")]
    pub established_leciscount: Option<u32>,

/// 
    #[serde(rename = "LEACLbytesreadPersec")]
    pub leaclbytesread_persec: Option<u32>,

/// 
    #[serde(rename = "LEACLbyteswrittenPersec")]
    pub leaclbyteswritten_persec: Option<u32>,

/// 
    #[serde(rename = "LEConnectionInterval")]
    pub leconnection_interval: Option<u32>,

/// 
    #[serde(rename = "LEConnectionSupervisionTimeout")]
    pub leconnection_supervision_timeout: Option<u32>,

/// 
    #[serde(rename = "LEContinuationNumber")]
    pub lecontinuation_number: Option<u32>,

/// 
    #[serde(rename = "LEPeripheralLatency")]
    pub leperipheral_latency: Option<u32>,

/// 
    #[serde(rename = "LESubrateFactor")]
    pub lesubrate_factor: Option<u32>,

/// 
    #[serde(rename = "SCObytesreadPersec")]
    pub scobytesread_persec: Option<u32>,

/// 
    #[serde(rename = "SCObyteswrittenPersec")]
    pub scobyteswritten_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_BluetoothDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            classic_aclbytesread_persec: None,
            classic_aclbyteswritten_persec: None,
            established_leciscount: None,
            leaclbytesread_persec: None,
            leaclbyteswritten_persec: None,
            leconnection_interval: None,
            leconnection_supervision_timeout: None,
            lecontinuation_number: None,
            leperipheral_latency: None,
            lesubrate_factor: None,
            scobytesread_persec: None,
            scobyteswritten_persec: None,
        }
    }


    /// Sets the value of ClassicACLbytesreadPersec
    pub fn set_classic_aclbytesread_persec(&mut self, value: u32) {
        self.classic_aclbytesread_persec = Some(value);
    }

    /// Gets the value of ClassicACLbytesreadPersec
    pub fn get_classic_aclbytesread_persec(&self) -> Option<&u32> {
        self.classic_aclbytesread_persec.as_ref()
    }

    /// Sets the value of ClassicACLbyteswrittenPersec
    pub fn set_classic_aclbyteswritten_persec(&mut self, value: u32) {
        self.classic_aclbyteswritten_persec = Some(value);
    }

    /// Gets the value of ClassicACLbyteswrittenPersec
    pub fn get_classic_aclbyteswritten_persec(&self) -> Option<&u32> {
        self.classic_aclbyteswritten_persec.as_ref()
    }

    /// Sets the value of EstablishedLECISCount
    pub fn set_established_leciscount(&mut self, value: u32) {
        self.established_leciscount = Some(value);
    }

    /// Gets the value of EstablishedLECISCount
    pub fn get_established_leciscount(&self) -> Option<&u32> {
        self.established_leciscount.as_ref()
    }

    /// Sets the value of LEACLbytesreadPersec
    pub fn set_leaclbytesread_persec(&mut self, value: u32) {
        self.leaclbytesread_persec = Some(value);
    }

    /// Gets the value of LEACLbytesreadPersec
    pub fn get_leaclbytesread_persec(&self) -> Option<&u32> {
        self.leaclbytesread_persec.as_ref()
    }

    /// Sets the value of LEACLbyteswrittenPersec
    pub fn set_leaclbyteswritten_persec(&mut self, value: u32) {
        self.leaclbyteswritten_persec = Some(value);
    }

    /// Gets the value of LEACLbyteswrittenPersec
    pub fn get_leaclbyteswritten_persec(&self) -> Option<&u32> {
        self.leaclbyteswritten_persec.as_ref()
    }

    /// Sets the value of LEConnectionInterval
    pub fn set_leconnection_interval(&mut self, value: u32) {
        self.leconnection_interval = Some(value);
    }

    /// Gets the value of LEConnectionInterval
    pub fn get_leconnection_interval(&self) -> Option<&u32> {
        self.leconnection_interval.as_ref()
    }

    /// Sets the value of LEConnectionSupervisionTimeout
    pub fn set_leconnection_supervision_timeout(&mut self, value: u32) {
        self.leconnection_supervision_timeout = Some(value);
    }

    /// Gets the value of LEConnectionSupervisionTimeout
    pub fn get_leconnection_supervision_timeout(&self) -> Option<&u32> {
        self.leconnection_supervision_timeout.as_ref()
    }

    /// Sets the value of LEContinuationNumber
    pub fn set_lecontinuation_number(&mut self, value: u32) {
        self.lecontinuation_number = Some(value);
    }

    /// Gets the value of LEContinuationNumber
    pub fn get_lecontinuation_number(&self) -> Option<&u32> {
        self.lecontinuation_number.as_ref()
    }

    /// Sets the value of LEPeripheralLatency
    pub fn set_leperipheral_latency(&mut self, value: u32) {
        self.leperipheral_latency = Some(value);
    }

    /// Gets the value of LEPeripheralLatency
    pub fn get_leperipheral_latency(&self) -> Option<&u32> {
        self.leperipheral_latency.as_ref()
    }

    /// Sets the value of LESubrateFactor
    pub fn set_lesubrate_factor(&mut self, value: u32) {
        self.lesubrate_factor = Some(value);
    }

    /// Gets the value of LESubrateFactor
    pub fn get_lesubrate_factor(&self) -> Option<&u32> {
        self.lesubrate_factor.as_ref()
    }

    /// Sets the value of SCObytesreadPersec
    pub fn set_scobytesread_persec(&mut self, value: u32) {
        self.scobytesread_persec = Some(value);
    }

    /// Gets the value of SCObytesreadPersec
    pub fn get_scobytesread_persec(&self) -> Option<&u32> {
        self.scobytesread_persec.as_ref()
    }

    /// Sets the value of SCObyteswrittenPersec
    pub fn set_scobyteswritten_persec(&mut self, value: u32) {
        self.scobyteswritten_persec = Some(value);
    }

    /// Gets the value of SCObyteswrittenPersec
    pub fn get_scobyteswritten_persec(&self) -> Option<&u32> {
        self.scobyteswritten_persec.as_ref()
    }
}


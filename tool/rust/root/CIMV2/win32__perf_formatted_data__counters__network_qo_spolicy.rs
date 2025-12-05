// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_NetworkQoSPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_NetworkQoSPolicy {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Bytestransmitted")]
    pub bytestransmitted: Option<u64>,

/// 
    #[serde(rename = "BytestransmittedPersec")]
    pub bytestransmitted_persec: Option<u64>,

/// 
    #[serde(rename = "Packetsdropped")]
    pub packetsdropped: Option<u32>,

/// 
    #[serde(rename = "PacketsdroppedPersec")]
    pub packetsdropped_persec: Option<u32>,

/// 
    #[serde(rename = "Packetstransmitted")]
    pub packetstransmitted: Option<u32>,

/// 
    #[serde(rename = "PacketstransmittedPersec")]
    pub packetstransmitted_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_NetworkQoSPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytestransmitted: None,
            bytestransmitted_persec: None,
            packetsdropped: None,
            packetsdropped_persec: None,
            packetstransmitted: None,
            packetstransmitted_persec: None,
        }
    }


    /// Sets the value of Bytestransmitted
    pub fn set_bytestransmitted(&mut self, value: u64) {
        self.bytestransmitted = Some(value);
    }

    /// Gets the value of Bytestransmitted
    pub fn get_bytestransmitted(&self) -> Option<&u64> {
        self.bytestransmitted.as_ref()
    }

    /// Sets the value of BytestransmittedPersec
    pub fn set_bytestransmitted_persec(&mut self, value: u64) {
        self.bytestransmitted_persec = Some(value);
    }

    /// Gets the value of BytestransmittedPersec
    pub fn get_bytestransmitted_persec(&self) -> Option<&u64> {
        self.bytestransmitted_persec.as_ref()
    }

    /// Sets the value of Packetsdropped
    pub fn set_packetsdropped(&mut self, value: u32) {
        self.packetsdropped = Some(value);
    }

    /// Gets the value of Packetsdropped
    pub fn get_packetsdropped(&self) -> Option<&u32> {
        self.packetsdropped.as_ref()
    }

    /// Sets the value of PacketsdroppedPersec
    pub fn set_packetsdropped_persec(&mut self, value: u32) {
        self.packetsdropped_persec = Some(value);
    }

    /// Gets the value of PacketsdroppedPersec
    pub fn get_packetsdropped_persec(&self) -> Option<&u32> {
        self.packetsdropped_persec.as_ref()
    }

    /// Sets the value of Packetstransmitted
    pub fn set_packetstransmitted(&mut self, value: u32) {
        self.packetstransmitted = Some(value);
    }

    /// Gets the value of Packetstransmitted
    pub fn get_packetstransmitted(&self) -> Option<&u32> {
        self.packetstransmitted.as_ref()
    }

    /// Sets the value of PacketstransmittedPersec
    pub fn set_packetstransmitted_persec(&mut self, value: u32) {
        self.packetstransmitted_persec = Some(value);
    }

    /// Gets the value of PacketstransmittedPersec
    pub fn get_packetstransmitted_persec(&self) -> Option<&u32> {
        self.packetstransmitted_persec.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PacerFlow struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PacerFlow {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Averagepacketsinnetcard")]
    pub averagepacketsinnetcard: Option<u32>,

/// 
    #[serde(rename = "Averagepacketsinsequencer")]
    pub averagepacketsinsequencer: Option<u32>,

/// 
    #[serde(rename = "Averagepacketsinshaper")]
    pub averagepacketsinshaper: Option<u32>,

/// 
    #[serde(rename = "Bytesscheduled")]
    pub bytesscheduled: Option<u64>,

/// 
    #[serde(rename = "BytesscheduledPersec")]
    pub bytesscheduled_persec: Option<u64>,

/// 
    #[serde(rename = "Bytestransmitted")]
    pub bytestransmitted: Option<u64>,

/// 
    #[serde(rename = "BytestransmittedPersec")]
    pub bytestransmitted_persec: Option<u64>,

/// 
    #[serde(rename = "Maximumpacketsinnetcard")]
    pub maximumpacketsinnetcard: Option<u32>,

/// 
    #[serde(rename = "Maxpacketsinsequencer")]
    pub maxpacketsinsequencer: Option<u32>,

/// 
    #[serde(rename = "Maxpacketsinshaper")]
    pub maxpacketsinshaper: Option<u32>,

/// 
    #[serde(rename = "Nonconformingpacketsscheduled")]
    pub nonconformingpacketsscheduled: Option<u32>,

/// 
    #[serde(rename = "NonconformingpacketsscheduledPersec")]
    pub nonconformingpacketsscheduled_persec: Option<u32>,

/// 
    #[serde(rename = "Nonconformingpacketstransmitted")]
    pub nonconformingpacketstransmitted: Option<u32>,

/// 
    #[serde(rename = "NonconformingpacketstransmittedPersec")]
    pub nonconformingpacketstransmitted_persec: Option<u32>,

/// 
    #[serde(rename = "Packetsdropped")]
    pub packetsdropped: Option<u32>,

/// 
    #[serde(rename = "PacketsdroppedPersec")]
    pub packetsdropped_persec: Option<u32>,

/// 
    #[serde(rename = "Packetsscheduled")]
    pub packetsscheduled: Option<u32>,

/// 
    #[serde(rename = "PacketsscheduledPersec")]
    pub packetsscheduled_persec: Option<u32>,

/// 
    #[serde(rename = "Packetstransmitted")]
    pub packetstransmitted: Option<u32>,

/// 
    #[serde(rename = "PacketstransmittedPersec")]
    pub packetstransmitted_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_PacerFlow {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            averagepacketsinnetcard: None,
            averagepacketsinsequencer: None,
            averagepacketsinshaper: None,
            bytesscheduled: None,
            bytesscheduled_persec: None,
            bytestransmitted: None,
            bytestransmitted_persec: None,
            maximumpacketsinnetcard: None,
            maxpacketsinsequencer: None,
            maxpacketsinshaper: None,
            nonconformingpacketsscheduled: None,
            nonconformingpacketsscheduled_persec: None,
            nonconformingpacketstransmitted: None,
            nonconformingpacketstransmitted_persec: None,
            packetsdropped: None,
            packetsdropped_persec: None,
            packetsscheduled: None,
            packetsscheduled_persec: None,
            packetstransmitted: None,
            packetstransmitted_persec: None,
        }
    }


    /// Sets the value of Averagepacketsinnetcard
    pub fn set_averagepacketsinnetcard(&mut self, value: u32) {
        self.averagepacketsinnetcard = Some(value);
    }

    /// Gets the value of Averagepacketsinnetcard
    pub fn get_averagepacketsinnetcard(&self) -> Option<&u32> {
        self.averagepacketsinnetcard.as_ref()
    }

    /// Sets the value of Averagepacketsinsequencer
    pub fn set_averagepacketsinsequencer(&mut self, value: u32) {
        self.averagepacketsinsequencer = Some(value);
    }

    /// Gets the value of Averagepacketsinsequencer
    pub fn get_averagepacketsinsequencer(&self) -> Option<&u32> {
        self.averagepacketsinsequencer.as_ref()
    }

    /// Sets the value of Averagepacketsinshaper
    pub fn set_averagepacketsinshaper(&mut self, value: u32) {
        self.averagepacketsinshaper = Some(value);
    }

    /// Gets the value of Averagepacketsinshaper
    pub fn get_averagepacketsinshaper(&self) -> Option<&u32> {
        self.averagepacketsinshaper.as_ref()
    }

    /// Sets the value of Bytesscheduled
    pub fn set_bytesscheduled(&mut self, value: u64) {
        self.bytesscheduled = Some(value);
    }

    /// Gets the value of Bytesscheduled
    pub fn get_bytesscheduled(&self) -> Option<&u64> {
        self.bytesscheduled.as_ref()
    }

    /// Sets the value of BytesscheduledPersec
    pub fn set_bytesscheduled_persec(&mut self, value: u64) {
        self.bytesscheduled_persec = Some(value);
    }

    /// Gets the value of BytesscheduledPersec
    pub fn get_bytesscheduled_persec(&self) -> Option<&u64> {
        self.bytesscheduled_persec.as_ref()
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

    /// Sets the value of Maximumpacketsinnetcard
    pub fn set_maximumpacketsinnetcard(&mut self, value: u32) {
        self.maximumpacketsinnetcard = Some(value);
    }

    /// Gets the value of Maximumpacketsinnetcard
    pub fn get_maximumpacketsinnetcard(&self) -> Option<&u32> {
        self.maximumpacketsinnetcard.as_ref()
    }

    /// Sets the value of Maxpacketsinsequencer
    pub fn set_maxpacketsinsequencer(&mut self, value: u32) {
        self.maxpacketsinsequencer = Some(value);
    }

    /// Gets the value of Maxpacketsinsequencer
    pub fn get_maxpacketsinsequencer(&self) -> Option<&u32> {
        self.maxpacketsinsequencer.as_ref()
    }

    /// Sets the value of Maxpacketsinshaper
    pub fn set_maxpacketsinshaper(&mut self, value: u32) {
        self.maxpacketsinshaper = Some(value);
    }

    /// Gets the value of Maxpacketsinshaper
    pub fn get_maxpacketsinshaper(&self) -> Option<&u32> {
        self.maxpacketsinshaper.as_ref()
    }

    /// Sets the value of Nonconformingpacketsscheduled
    pub fn set_nonconformingpacketsscheduled(&mut self, value: u32) {
        self.nonconformingpacketsscheduled = Some(value);
    }

    /// Gets the value of Nonconformingpacketsscheduled
    pub fn get_nonconformingpacketsscheduled(&self) -> Option<&u32> {
        self.nonconformingpacketsscheduled.as_ref()
    }

    /// Sets the value of NonconformingpacketsscheduledPersec
    pub fn set_nonconformingpacketsscheduled_persec(&mut self, value: u32) {
        self.nonconformingpacketsscheduled_persec = Some(value);
    }

    /// Gets the value of NonconformingpacketsscheduledPersec
    pub fn get_nonconformingpacketsscheduled_persec(&self) -> Option<&u32> {
        self.nonconformingpacketsscheduled_persec.as_ref()
    }

    /// Sets the value of Nonconformingpacketstransmitted
    pub fn set_nonconformingpacketstransmitted(&mut self, value: u32) {
        self.nonconformingpacketstransmitted = Some(value);
    }

    /// Gets the value of Nonconformingpacketstransmitted
    pub fn get_nonconformingpacketstransmitted(&self) -> Option<&u32> {
        self.nonconformingpacketstransmitted.as_ref()
    }

    /// Sets the value of NonconformingpacketstransmittedPersec
    pub fn set_nonconformingpacketstransmitted_persec(&mut self, value: u32) {
        self.nonconformingpacketstransmitted_persec = Some(value);
    }

    /// Gets the value of NonconformingpacketstransmittedPersec
    pub fn get_nonconformingpacketstransmitted_persec(&self) -> Option<&u32> {
        self.nonconformingpacketstransmitted_persec.as_ref()
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

    /// Sets the value of Packetsscheduled
    pub fn set_packetsscheduled(&mut self, value: u32) {
        self.packetsscheduled = Some(value);
    }

    /// Gets the value of Packetsscheduled
    pub fn get_packetsscheduled(&self) -> Option<&u32> {
        self.packetsscheduled.as_ref()
    }

    /// Sets the value of PacketsscheduledPersec
    pub fn set_packetsscheduled_persec(&mut self, value: u32) {
        self.packetsscheduled_persec = Some(value);
    }

    /// Gets the value of PacketsscheduledPersec
    pub fn get_packetsscheduled_persec(&self) -> Option<&u32> {
        self.packetsscheduled_persec.as_ref()
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


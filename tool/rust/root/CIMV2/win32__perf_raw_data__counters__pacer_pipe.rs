// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_PacerPipe struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_PacerPipe {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

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
    #[serde(rename = "Flowmodsrejected")]
    pub flowmodsrejected: Option<u32>,

/// 
    #[serde(rename = "Flowsclosed")]
    pub flowsclosed: Option<u32>,

/// 
    #[serde(rename = "Flowsmodified")]
    pub flowsmodified: Option<u32>,

/// 
    #[serde(rename = "Flowsopened")]
    pub flowsopened: Option<u32>,

/// 
    #[serde(rename = "Flowsrejected")]
    pub flowsrejected: Option<u32>,

/// 
    #[serde(rename = "Maxpacketsinnetcard")]
    pub maxpacketsinnetcard: Option<u32>,

/// 
    #[serde(rename = "Maxpacketsinsequencer")]
    pub maxpacketsinsequencer: Option<u32>,

/// 
    #[serde(rename = "Maxpacketsinshaper")]
    pub maxpacketsinshaper: Option<u32>,

/// 
    #[serde(rename = "Maxsimultaneousflows")]
    pub maxsimultaneousflows: Option<u32>,

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
    #[serde(rename = "Outofpackets")]
    pub outofpackets: Option<u32>,
}

impl Win32_PerfRawData_Counters_PacerPipe {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            averagepacketsinnetcard: None,
            averagepacketsinsequencer: None,
            averagepacketsinshaper: None,
            flowmodsrejected: None,
            flowsclosed: None,
            flowsmodified: None,
            flowsopened: None,
            flowsrejected: None,
            maxpacketsinnetcard: None,
            maxpacketsinsequencer: None,
            maxpacketsinshaper: None,
            maxsimultaneousflows: None,
            nonconformingpacketsscheduled: None,
            nonconformingpacketsscheduled_persec: None,
            nonconformingpacketstransmitted: None,
            nonconformingpacketstransmitted_persec: None,
            outofpackets: None,
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

    /// Sets the value of Flowmodsrejected
    pub fn set_flowmodsrejected(&mut self, value: u32) {
        self.flowmodsrejected = Some(value);
    }

    /// Gets the value of Flowmodsrejected
    pub fn get_flowmodsrejected(&self) -> Option<&u32> {
        self.flowmodsrejected.as_ref()
    }

    /// Sets the value of Flowsclosed
    pub fn set_flowsclosed(&mut self, value: u32) {
        self.flowsclosed = Some(value);
    }

    /// Gets the value of Flowsclosed
    pub fn get_flowsclosed(&self) -> Option<&u32> {
        self.flowsclosed.as_ref()
    }

    /// Sets the value of Flowsmodified
    pub fn set_flowsmodified(&mut self, value: u32) {
        self.flowsmodified = Some(value);
    }

    /// Gets the value of Flowsmodified
    pub fn get_flowsmodified(&self) -> Option<&u32> {
        self.flowsmodified.as_ref()
    }

    /// Sets the value of Flowsopened
    pub fn set_flowsopened(&mut self, value: u32) {
        self.flowsopened = Some(value);
    }

    /// Gets the value of Flowsopened
    pub fn get_flowsopened(&self) -> Option<&u32> {
        self.flowsopened.as_ref()
    }

    /// Sets the value of Flowsrejected
    pub fn set_flowsrejected(&mut self, value: u32) {
        self.flowsrejected = Some(value);
    }

    /// Gets the value of Flowsrejected
    pub fn get_flowsrejected(&self) -> Option<&u32> {
        self.flowsrejected.as_ref()
    }

    /// Sets the value of Maxpacketsinnetcard
    pub fn set_maxpacketsinnetcard(&mut self, value: u32) {
        self.maxpacketsinnetcard = Some(value);
    }

    /// Gets the value of Maxpacketsinnetcard
    pub fn get_maxpacketsinnetcard(&self) -> Option<&u32> {
        self.maxpacketsinnetcard.as_ref()
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

    /// Sets the value of Maxsimultaneousflows
    pub fn set_maxsimultaneousflows(&mut self, value: u32) {
        self.maxsimultaneousflows = Some(value);
    }

    /// Gets the value of Maxsimultaneousflows
    pub fn get_maxsimultaneousflows(&self) -> Option<&u32> {
        self.maxsimultaneousflows.as_ref()
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

    /// Sets the value of Outofpackets
    pub fn set_outofpackets(&mut self, value: u32) {
        self.outofpackets = Some(value);
    }

    /// Gets the value of Outofpackets
    pub fn get_outofpackets(&self) -> Option<&u32> {
        self.outofpackets.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_MSDTCBridge4000_MSDTCBridge4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_MSDTCBridge4000_MSDTCBridge4000 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Averageparticipantcommitresponsetime")]
    pub averageparticipantcommitresponsetime: Option<u32>,

/// 
    #[serde(rename = "Averageparticipantprepareresponsetime")]
    pub averageparticipantprepareresponsetime: Option<u32>,

/// 
    #[serde(rename = "CommitretrycountPersec")]
    pub commitretrycount_persec: Option<u32>,

/// 
    #[serde(rename = "FaultsreceivedcountPersec")]
    pub faultsreceivedcount_persec: Option<u32>,

/// 
    #[serde(rename = "FaultssentcountPersec")]
    pub faultssentcount_persec: Option<u32>,

/// 
    #[serde(rename = "MessagesendfailuresPersec")]
    pub messagesendfailures_persec: Option<u32>,

/// 
    #[serde(rename = "PreparedretrycountPersec")]
    pub preparedretrycount_persec: Option<u32>,

/// 
    #[serde(rename = "PrepareretrycountPersec")]
    pub prepareretrycount_persec: Option<u32>,

/// 
    #[serde(rename = "ReplayretrycountPersec")]
    pub replayretrycount_persec: Option<u32>,
}

impl Win32_PerfFormattedData_MSDTCBridge4000_MSDTCBridge4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            averageparticipantcommitresponsetime: None,
            averageparticipantprepareresponsetime: None,
            commitretrycount_persec: None,
            faultsreceivedcount_persec: None,
            faultssentcount_persec: None,
            messagesendfailures_persec: None,
            preparedretrycount_persec: None,
            prepareretrycount_persec: None,
            replayretrycount_persec: None,
        }
    }


    /// Sets the value of Averageparticipantcommitresponsetime
    pub fn set_averageparticipantcommitresponsetime(&mut self, value: u32) {
        self.averageparticipantcommitresponsetime = Some(value);
    }

    /// Gets the value of Averageparticipantcommitresponsetime
    pub fn get_averageparticipantcommitresponsetime(&self) -> Option<&u32> {
        self.averageparticipantcommitresponsetime.as_ref()
    }

    /// Sets the value of Averageparticipantprepareresponsetime
    pub fn set_averageparticipantprepareresponsetime(&mut self, value: u32) {
        self.averageparticipantprepareresponsetime = Some(value);
    }

    /// Gets the value of Averageparticipantprepareresponsetime
    pub fn get_averageparticipantprepareresponsetime(&self) -> Option<&u32> {
        self.averageparticipantprepareresponsetime.as_ref()
    }

    /// Sets the value of CommitretrycountPersec
    pub fn set_commitretrycount_persec(&mut self, value: u32) {
        self.commitretrycount_persec = Some(value);
    }

    /// Gets the value of CommitretrycountPersec
    pub fn get_commitretrycount_persec(&self) -> Option<&u32> {
        self.commitretrycount_persec.as_ref()
    }

    /// Sets the value of FaultsreceivedcountPersec
    pub fn set_faultsreceivedcount_persec(&mut self, value: u32) {
        self.faultsreceivedcount_persec = Some(value);
    }

    /// Gets the value of FaultsreceivedcountPersec
    pub fn get_faultsreceivedcount_persec(&self) -> Option<&u32> {
        self.faultsreceivedcount_persec.as_ref()
    }

    /// Sets the value of FaultssentcountPersec
    pub fn set_faultssentcount_persec(&mut self, value: u32) {
        self.faultssentcount_persec = Some(value);
    }

    /// Gets the value of FaultssentcountPersec
    pub fn get_faultssentcount_persec(&self) -> Option<&u32> {
        self.faultssentcount_persec.as_ref()
    }

    /// Sets the value of MessagesendfailuresPersec
    pub fn set_messagesendfailures_persec(&mut self, value: u32) {
        self.messagesendfailures_persec = Some(value);
    }

    /// Gets the value of MessagesendfailuresPersec
    pub fn get_messagesendfailures_persec(&self) -> Option<&u32> {
        self.messagesendfailures_persec.as_ref()
    }

    /// Sets the value of PreparedretrycountPersec
    pub fn set_preparedretrycount_persec(&mut self, value: u32) {
        self.preparedretrycount_persec = Some(value);
    }

    /// Gets the value of PreparedretrycountPersec
    pub fn get_preparedretrycount_persec(&self) -> Option<&u32> {
        self.preparedretrycount_persec.as_ref()
    }

    /// Sets the value of PrepareretrycountPersec
    pub fn set_prepareretrycount_persec(&mut self, value: u32) {
        self.prepareretrycount_persec = Some(value);
    }

    /// Gets the value of PrepareretrycountPersec
    pub fn get_prepareretrycount_persec(&self) -> Option<&u32> {
        self.prepareretrycount_persec.as_ref()
    }

    /// Sets the value of ReplayretrycountPersec
    pub fn set_replayretrycount_persec(&mut self, value: u32) {
        self.replayretrycount_persec = Some(value);
    }

    /// Gets the value of ReplayretrycountPersec
    pub fn get_replayretrycount_persec(&self) -> Option<&u32> {
        self.replayretrycount_persec.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_Offload_RsnRekey struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_Offload_RsnRekey {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterPowerManagement_Offload,

/// 
    #[serde(rename = "KCK")]
    pub kck: Vec<u8>,

/// 
    #[serde(rename = "KEK")]
    pub kek: Vec<u8>,

/// 
    #[serde(rename = "ReplayCounter")]
    pub replay_counter: Option<u64>,
}

impl MSFT_NetAdapterPowerManagement_Offload_RsnRekey {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterPowerManagement_Offload::new(),
            kck: Vec::new(),
            kek: Vec::new(),
            replay_counter: None,
        }
    }


    /// Sets the value of KCK
    pub fn set_kck(&mut self, value: Vec<u8>) {
        self.kck = value;
    }

    /// Gets the value of KCK
    pub fn get_kck(&self) -> &Vec<u8> {
        &self.kck
    }

    /// Sets the value of KEK
    pub fn set_kek(&mut self, value: Vec<u8>) {
        self.kek = value;
    }

    /// Gets the value of KEK
    pub fn get_kek(&self) -> &Vec<u8> {
        &self.kek
    }

    /// Sets the value of ReplayCounter
    pub fn set_replay_counter(&mut self, value: u64) {
        self.replay_counter = Some(value);
    }

    /// Gets the value of ReplayCounter
    pub fn get_replay_counter(&self) -> Option<&u64> {
        self.replay_counter.as_ref()
    }
}


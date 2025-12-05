// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAURun_Report_Chunk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAURun_Report_Chunk {
    #[serde(flatten)]
    pub base: MSFT_CAURun_Report_ID,

/// 
    #[serde(rename = "Data")]
    pub data: Option<String>,

/// 
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<u32>,
}

impl MSFT_CAURun_Report_Chunk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_CAURun_Report_ID::new(),
            data: None,
            sequence_number: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: String) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    /// Sets the value of SequenceNumber
    pub fn set_sequence_number(&mut self, value: u32) {
        self.sequence_number = Some(value);
    }

    /// Gets the value of SequenceNumber
    pub fn get_sequence_number(&self) -> Option<&u32> {
        self.sequence_number.as_ref()
    }
}


// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_CancelQuery_Post struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_CancelQuery_Post {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Post,

/// 
    #[serde(rename = "QueryId")]
    pub query_id: Option<u32>,

/// 
    #[serde(rename = "Result")]
    pub result: Option<u32>,
}

impl Msft_WmiProvider_CancelQuery_Post {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Post::new(),
            query_id: None,
            result: None,
        }
    }


    /// Sets the value of QueryId
    pub fn set_query_id(&mut self, value: u32) {
        self.query_id = Some(value);
    }

    /// Gets the value of QueryId
    pub fn get_query_id(&self) -> Option<&u32> {
        self.query_id.as_ref()
    }

    /// Sets the value of Result
    pub fn set_result(&mut self, value: u32) {
        self.result = Some(value);
    }

    /// Gets the value of Result
    pub fn get_result(&self) -> Option<&u32> {
        self.result.as_ref()
    }
}


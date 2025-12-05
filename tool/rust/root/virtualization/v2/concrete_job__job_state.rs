// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConcreteJob_JobState
//////////////////////////////////////////////

/// ConcreteJob_JobState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConcreteJob_JobState {
    /// New
    #[serde(rename = "New")]
    New = 2,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 3,
    /// Running
    #[serde(rename = "Running")]
    Running = 4,
    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended = 5,
    /// Shutting_Down
    #[serde(rename = "Shutting_Down")]
    ShuttingDown = 6,
    /// Completed
    #[serde(rename = "Completed")]
    Completed = 7,
    /// Terminated
    #[serde(rename = "Terminated")]
    Terminated = 8,
    /// Killed
    #[serde(rename = "Killed")]
    Killed = 9,
    /// Exception
    #[serde(rename = "Exception")]
    Exception = 10,
    /// Service
    #[serde(rename = "Service")]
    Service = 11,
    /// Query_Pending
    #[serde(rename = "Query_Pending")]
    QueryPending = 12,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 13,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 14,
}

impl Default for ConcreteJob_JobState {
    fn default() -> Self {
        Self::New
    }
}


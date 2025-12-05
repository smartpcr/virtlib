// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidBthMtpEnum_Flags
//////////////////////////////////////////////

/// CtlGuidBthMtpEnum_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidBthMtpEnum_Flags {
    /// CRITICAL_Flag
    #[serde(rename = "CRITICAL_Flag")]
    CRITICALFlag = 1,
    /// ERROR_Flag
    #[serde(rename = "ERROR_Flag")]
    ERRORFlag = 2,
    /// WARNING_Flag
    #[serde(rename = "WARNING_Flag")]
    WARNINGFlag = 3,
    /// ASSERT_Flag
    #[serde(rename = "ASSERT_Flag")]
    ASSERTFlag = 4,
    /// FUNC_TRACE_Flag
    #[serde(rename = "FUNC_TRACE_Flag")]
    FUNCTRACEFlag = 5,
    /// INFO_Flag
    #[serde(rename = "INFO_Flag")]
    INFOFlag = 6,
    /// TRACE_Flag
    #[serde(rename = "TRACE_Flag")]
    TRACEFlag = 7,
    /// DEV_CONFIG_Flag
    #[serde(rename = "DEV_CONFIG_Flag")]
    DEVCONFIGFlag = 8,
    /// PnP_Flag
    #[serde(rename = "PnP_Flag")]
    PnPFlag = 9,
    /// Event_Flag
    #[serde(rename = "Event_Flag")]
    EventFlag = 10,
    /// Cancel_Flag
    #[serde(rename = "Cancel_Flag")]
    CancelFlag = 11,
    /// Perf_Flag
    #[serde(rename = "Perf_Flag")]
    PerfFlag = 12,
    /// PerfVerbose_Flag
    #[serde(rename = "PerfVerbose_Flag")]
    PerfVerboseFlag = 13,
    /// Bulk_Flag
    #[serde(rename = "Bulk_Flag")]
    BulkFlag = 14,
    /// MtpOp_Flag
    #[serde(rename = "MtpOp_Flag")]
    MtpOpFlag = 15,
    /// Config_Flag
    #[serde(rename = "Config_Flag")]
    ConfigFlag = 16,
    /// L2CAP_Flag
    #[serde(rename = "L2CAP_Flag")]
    L2CAPFlag = 17,
    /// IO_Flag
    #[serde(rename = "IO_Flag")]
    IOFlag = 18,
    /// Packet
    #[serde(rename = "Packet")]
    Packet = 19,
}

impl Default for CtlGuidBthMtpEnum_Flags {
    fn default() -> Self {
        Self::CRITICALFlag
    }
}


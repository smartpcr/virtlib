// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NETIO_Flags
//////////////////////////////////////////////

/// NETIO_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NETIO_Flags {
    /// NETIO_TRACE_SEND
    #[serde(rename = "NETIO_TRACE_SEND")]
    NETIOTRACESEND = 1,
    /// NETIO_TRACE_RECEIVE
    #[serde(rename = "NETIO_TRACE_RECEIVE")]
    NETIOTRACERECEIVE = 2,
    /// NETIO_TRACE_INTERFACE
    #[serde(rename = "NETIO_TRACE_INTERFACE")]
    NETIOTRACEINTERFACE = 3,
    /// NETIO_TRACE_NMR
    #[serde(rename = "NETIO_TRACE_NMR")]
    NETIOTRACENMR = 4,
    /// NETIO_TRACE_NSI
    #[serde(rename = "NETIO_TRACE_NSI")]
    NETIOTRACENSI = 5,
    /// NETIO_TRACE_FRAMING
    #[serde(rename = "NETIO_TRACE_FRAMING")]
    NETIOTRACEFRAMING = 6,
    /// NETIO_TRACE_NETWORK
    #[serde(rename = "NETIO_TRACE_NETWORK")]
    NETIOTRACENETWORK = 7,
    /// NETIO_TRACE_TRANSPORT
    #[serde(rename = "NETIO_TRACE_TRANSPORT")]
    NETIOTRACETRANSPORT = 8,
    /// NETIO_TRACE_TUNNEL
    #[serde(rename = "NETIO_TRACE_TUNNEL")]
    NETIOTRACETUNNEL = 9,
    /// NETIO_TRACE_TDX
    #[serde(rename = "NETIO_TRACE_TDX")]
    NETIOTRACETDX = 10,
    /// NETIO_TRACE_LEGACY
    #[serde(rename = "NETIO_TRACE_LEGACY")]
    NETIOTRACELEGACY = 11,
    /// NETIO_TRACE_PACER
    #[serde(rename = "NETIO_TRACE_PACER")]
    NETIOTRACEPACER = 12,
    /// WFP_TRACE_BASE
    #[serde(rename = "WFP_TRACE_BASE")]
    WFPTRACEBASE = 13,
    /// WFP_TRACE_FE
    #[serde(rename = "WFP_TRACE_FE")]
    WFPTRACEFE = 14,
    /// WFP_TRACE_STM
    #[serde(rename = "WFP_TRACE_STM")]
    WFPTRACESTM = 15,
    /// WFP_TRACE_IPSEC
    #[serde(rename = "WFP_TRACE_IPSEC")]
    WFPTRACEIPSEC = 16,
    /// WFP_TRACE_ALE
    #[serde(rename = "WFP_TRACE_ALE")]
    WFPTRACEALE = 17,
    /// WFP_TRACE_OFFLOAD
    #[serde(rename = "WFP_TRACE_OFFLOAD")]
    WFPTRACEOFFLOAD = 18,
    /// NETIO_TRACE_EQOS
    #[serde(rename = "NETIO_TRACE_EQOS")]
    NETIOTRACEEQOS = 19,
    /// NETIO_TRACE_RSS
    #[serde(rename = "NETIO_TRACE_RSS")]
    NETIOTRACERSS = 20,
    /// NETIO_TRACE_NRT
    #[serde(rename = "NETIO_TRACE_NRT")]
    NETIOTRACENRT = 21,
    /// NETIO_TRACE_NCM
    #[serde(rename = "NETIO_TRACE_NCM")]
    NETIOTRACENCM = 22,
    /// NETIO_TRACE_IPHLPAPI
    #[serde(rename = "NETIO_TRACE_IPHLPAPI")]
    NETIOTRACEIPHLPAPI = 23,
    /// NETIO_TRACE_IPSNPI
    #[serde(rename = "NETIO_TRACE_IPSNPI")]
    NETIOTRACEIPSNPI = 24,
}

impl Default for NETIO_Flags {
    fn default() -> Self {
        Self::NETIOTRACESEND
    }
}


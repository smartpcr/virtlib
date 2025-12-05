// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NetAdapterCimNDISWMI_Flags
//////////////////////////////////////////////

/// NetAdapterCimNDISWMI_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NetAdapterCimNDISWMI_Flags {
    /// NDISWMI_TRACE_ADVPROP
    #[serde(rename = "NDISWMI_TRACE_ADVPROP")]
    NDISWMITRACEADVPROP = 0,
    /// NDISWMI_TRACE_BINDING
    #[serde(rename = "NDISWMI_TRACE_BINDING")]
    NDISWMITRACEBINDING = 1,
    /// NDISWMI_TRACE_CALL
    #[serde(rename = "NDISWMI_TRACE_CALL")]
    NDISWMITRACECALL = 2,
    /// NDISWMI_TRACE_CHKSUM
    #[serde(rename = "NDISWMI_TRACE_CHKSUM")]
    NDISWMITRACECHKSUM = 3,
    /// NDISWMI_TRACE_ENCAP
    #[serde(rename = "NDISWMI_TRACE_ENCAP")]
    NDISWMITRACEENCAP = 4,
    /// NDISWMI_TRACE_GENERAL
    #[serde(rename = "NDISWMI_TRACE_GENERAL")]
    NDISWMITRACEGENERAL = 5,
    /// NDISWMI_TRACE_HWINFO
    #[serde(rename = "NDISWMI_TRACE_HWINFO")]
    NDISWMITRACEHWINFO = 6,
    /// NDISWMI_TRACE_LSO
    #[serde(rename = "NDISWMI_TRACE_LSO")]
    NDISWMITRACELSO = 7,
    /// NDISWMI_TRACE_NETCFG
    #[serde(rename = "NDISWMI_TRACE_NETCFG")]
    NDISWMITRACENETCFG = 8,
    /// NDISWMI_TRACE_POWER
    #[serde(rename = "NDISWMI_TRACE_POWER")]
    NDISWMITRACEPOWER = 9,
    /// NDISWMI_TRACE_QOS
    #[serde(rename = "NDISWMI_TRACE_QOS")]
    NDISWMITRACEQOS = 10,
    /// NDISWMI_TRACE_RDMA
    #[serde(rename = "NDISWMI_TRACE_RDMA")]
    NDISWMITRACERDMA = 11,
    /// NDISWMI_TRACE_RSC
    #[serde(rename = "NDISWMI_TRACE_RSC")]
    NDISWMITRACERSC = 12,
    /// NDISWMI_TRACE_RSS
    #[serde(rename = "NDISWMI_TRACE_RSS")]
    NDISWMITRACERSS = 13,
    /// NDISWMI_TRACE_SRIOV
    #[serde(rename = "NDISWMI_TRACE_SRIOV")]
    NDISWMITRACESRIOV = 14,
    /// NDISWMI_TRACE_STATS
    #[serde(rename = "NDISWMI_TRACE_STATS")]
    NDISWMITRACESTATS = 15,
    /// NDISWMI_TRACE_VMQ
    #[serde(rename = "NDISWMI_TRACE_VMQ")]
    NDISWMITRACEVMQ = 16,
    /// NDISWMI_TRACE_USO
    #[serde(rename = "NDISWMI_TRACE_USO")]
    NDISWMITRACEUSO = 17,
    /// NDISWMI_TRACE_DATAPATHCONFIGURATION
    #[serde(rename = "NDISWMI_TRACE_DATAPATHCONFIGURATION")]
    NDISWMITRACEDATAPATHCONFIGURATION = 18,
    /// NDISWMI_TRACE_URO
    #[serde(rename = "NDISWMI_TRACE_URO")]
    NDISWMITRACEURO = 19,
}

impl Default for NetAdapterCimNDISWMI_Flags {
    fn default() -> Self {
        Self::NDISWMITRACEADVPROP
    }
}


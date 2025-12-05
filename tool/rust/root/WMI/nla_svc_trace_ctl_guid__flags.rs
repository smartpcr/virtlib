// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NlaSvcTraceCtlGuid_Flags
//////////////////////////////////////////////

/// NlaSvcTraceCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NlaSvcTraceCtlGuid_Flags {
    /// TM_LIST
    #[serde(rename = "TM_LIST")]
    TMLIST = 1,
    /// TM_LOCK
    #[serde(rename = "TM_LOCK")]
    TMLOCK = 2,
    /// TM_MEMORY
    #[serde(rename = "TM_MEMORY")]
    TMMEMORY = 3,
    /// TM_REFOBJ
    #[serde(rename = "TM_REFOBJ")]
    TMREFOBJ = 4,
    /// TM_HANDLE
    #[serde(rename = "TM_HANDLE")]
    TMHANDLE = 5,
    /// TM_FSM
    #[serde(rename = "TM_FSM")]
    TMFSM = 6,
    /// TM_TRIEMAP
    #[serde(rename = "TM_TRIEMAP")]
    TMTRIEMAP = 7,
    /// TM_CACHE
    #[serde(rename = "TM_CACHE")]
    TMCACHE = 8,
    /// TM_UNITTEST
    #[serde(rename = "TM_UNITTEST")]
    TMUNITTEST = 9,
    /// TM_DLLMAIN
    #[serde(rename = "TM_DLLMAIN")]
    TMDLLMAIN = 10,
    /// TM_ME
    #[serde(rename = "TM_ME")]
    TMME = 11,
    /// TM_STORE
    #[serde(rename = "TM_STORE")]
    TMSTORE = 12,
    /// TM_PMUX
    #[serde(rename = "TM_PMUX")]
    TMPMUX = 13,
    /// TM_FUSER
    #[serde(rename = "TM_FUSER")]
    TMFUSER = 14,
    /// TM_PLGWORKER
    #[serde(rename = "TM_PLGWORKER")]
    TMPLGWORKER = 15,
    /// TM_PLGEXT
    #[serde(rename = "TM_PLGEXT")]
    TMPLGEXT = 16,
    /// TM_PLGINT
    #[serde(rename = "TM_PLGINT")]
    TMPLGINT = 17,
    /// TM_IPPC
    #[serde(rename = "TM_IPPC")]
    TMIPPC = 18,
    /// TM_APPSRV
    #[serde(rename = "TM_APPSRV")]
    TMAPPSRV = 19,
    /// TM_NLAAPI
    #[serde(rename = "TM_NLAAPI")]
    TMNLAAPI = 20,
    /// TM_PNP
    #[serde(rename = "TM_PNP")]
    TMPNP = 21,
    /// TM_INTRANET
    #[serde(rename = "TM_INTRANET")]
    TMINTRANET = 22,
    /// TM_INTERNET
    #[serde(rename = "TM_INTERNET")]
    TMINTERNET = 23,
    /// TM_ARPND
    #[serde(rename = "TM_ARPND")]
    TMARPND = 24,
    /// TM_COMPCLNT
    #[serde(rename = "TM_COMPCLNT")]
    TMCOMPCLNT = 25,
    /// TM_ICSC
    #[serde(rename = "TM_ICSC")]
    TMICSC = 26,
    /// TM_NLAWPLG
    #[serde(rename = "TM_NLAWPLG")]
    TMNLAWPLG = 27,
    /// TM_BWC
    #[serde(rename = "TM_BWC")]
    TMBWC = 28,
}

impl Default for NlaSvcTraceCtlGuid_Flags {
    fn default() -> Self {
        Self::TMLIST
    }
}


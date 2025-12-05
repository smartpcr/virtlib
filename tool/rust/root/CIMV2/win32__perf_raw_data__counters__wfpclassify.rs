// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_WFPClassify struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_WFPClassify {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "FWPMLAYERALEACCEPTREDIRECTV4")]
    pub fwpmlayeraleacceptredirectv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEACCEPTREDIRECTV6")]
    pub fwpmlayeraleacceptredirectv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHCONNECTV4")]
    pub fwpmlayeraleauthconnectv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHCONNECTV4DISCARD")]
    pub fwpmlayeraleauthconnectv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHCONNECTV6")]
    pub fwpmlayeraleauthconnectv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHCONNECTV6DISCARD")]
    pub fwpmlayeraleauthconnectv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHLISTENV4")]
    pub fwpmlayeraleauthlistenv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHLISTENV4DISCARD")]
    pub fwpmlayeraleauthlistenv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHLISTENV6")]
    pub fwpmlayeraleauthlistenv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHLISTENV6DISCARD")]
    pub fwpmlayeraleauthlistenv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHRECVACCEPTV4")]
    pub fwpmlayeraleauthrecvacceptv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHRECVACCEPTV4DISCARD")]
    pub fwpmlayeraleauthrecvacceptv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHRECVACCEPTV6")]
    pub fwpmlayeraleauthrecvacceptv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEAUTHRECVACCEPTV6DISCARD")]
    pub fwpmlayeraleauthrecvacceptv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEBINDREDIRECTV4")]
    pub fwpmlayeralebindredirectv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEBINDREDIRECTV6")]
    pub fwpmlayeralebindredirectv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALECONNECTREDIRECTV4")]
    pub fwpmlayeraleconnectredirectv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALECONNECTREDIRECTV6")]
    pub fwpmlayeraleconnectredirectv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEENDPOINTCLOSUREV4")]
    pub fwpmlayeraleendpointclosurev4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEENDPOINTCLOSUREV6")]
    pub fwpmlayeraleendpointclosurev6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEFLOWESTABLISHEDV4")]
    pub fwpmlayeraleflowestablishedv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEFLOWESTABLISHEDV4DISCARD")]
    pub fwpmlayeraleflowestablishedv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEFLOWESTABLISHEDV6")]
    pub fwpmlayeraleflowestablishedv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALEFLOWESTABLISHEDV6DISCARD")]
    pub fwpmlayeraleflowestablishedv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCEASSIGNMENTV4")]
    pub fwpmlayeraleresourceassignmentv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCEASSIGNMENTV4DISCARD")]
    pub fwpmlayeraleresourceassignmentv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCEASSIGNMENTV6")]
    pub fwpmlayeraleresourceassignmentv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCEASSIGNMENTV6DISCARD")]
    pub fwpmlayeraleresourceassignmentv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCERELEASEV4")]
    pub fwpmlayeraleresourcereleasev4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERALERESOURCERELEASEV6")]
    pub fwpmlayeraleresourcereleasev6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERDATAGRAMDATAV4")]
    pub fwpmlayerdatagramdatav4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERDATAGRAMDATAV4DISCARD")]
    pub fwpmlayerdatagramdatav4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERDATAGRAMDATAV6")]
    pub fwpmlayerdatagramdatav6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERDATAGRAMDATAV6DISCARD")]
    pub fwpmlayerdatagramdatav6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEREGRESSVSWITCHETHERNET")]
    pub fwpmlayeregressvswitchethernet: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEREGRESSVSWITCHTRANSPORTV4")]
    pub fwpmlayeregressvswitchtransportv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEREGRESSVSWITCHTRANSPORTV6")]
    pub fwpmlayeregressvswitchtransportv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIKEEXTV4")]
    pub fwpmlayerikeextv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIKEEXTV6")]
    pub fwpmlayerikeextv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDICMPERRORV4")]
    pub fwpmlayerinboundicmperrorv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDICMPERRORV4DISCARD")]
    pub fwpmlayerinboundicmperrorv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDICMPERRORV6")]
    pub fwpmlayerinboundicmperrorv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDICMPERRORV6DISCARD")]
    pub fwpmlayerinboundicmperrorv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDIPPACKETV4")]
    pub fwpmlayerinboundippacketv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDIPPACKETV4DISCARD")]
    pub fwpmlayerinboundippacketv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDIPPACKETV6")]
    pub fwpmlayerinboundippacketv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDIPPACKETV6DISCARD")]
    pub fwpmlayerinboundippacketv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDMACFRAMEETHERNET")]
    pub fwpmlayerinboundmacframeethernet: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDMACFRAMENATIVE")]
    pub fwpmlayerinboundmacframenative: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDMACFRAMENATIVEFAST")]
    pub fwpmlayerinboundmacframenativefast: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDRESERVED2")]
    pub fwpmlayerinboundreserved2: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDTRANSPORTFAST")]
    pub fwpmlayerinboundtransportfast: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDTRANSPORTV4")]
    pub fwpmlayerinboundtransportv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDTRANSPORTV4DISCARD")]
    pub fwpmlayerinboundtransportv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDTRANSPORTV6")]
    pub fwpmlayerinboundtransportv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINBOUNDTRANSPORTV6DISCARD")]
    pub fwpmlayerinboundtransportv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINGRESSVSWITCHETHERNET")]
    pub fwpmlayeringressvswitchethernet: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINGRESSVSWITCHTRANSPORTV4")]
    pub fwpmlayeringressvswitchtransportv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERINGRESSVSWITCHTRANSPORTV6")]
    pub fwpmlayeringressvswitchtransportv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPFORWARDV4")]
    pub fwpmlayeripforwardv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPFORWARDV4DISCARD")]
    pub fwpmlayeripforwardv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPFORWARDV6")]
    pub fwpmlayeripforwardv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPFORWARDV6DISCARD")]
    pub fwpmlayeripforwardv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPSECKMDEMUXV4")]
    pub fwpmlayeripseckmdemuxv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPSECKMDEMUXV6")]
    pub fwpmlayeripseckmdemuxv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPSECV4")]
    pub fwpmlayeripsecv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERIPSECV6")]
    pub fwpmlayeripsecv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERKMAUTHORIZATION")]
    pub fwpmlayerkmauthorization: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERNAMERESOLUTIONCACHEV4")]
    pub fwpmlayernameresolutioncachev4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERNAMERESOLUTIONCACHEV6")]
    pub fwpmlayernameresolutioncachev6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDICMPERRORV4")]
    pub fwpmlayeroutboundicmperrorv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDICMPERRORV4DISCARD")]
    pub fwpmlayeroutboundicmperrorv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDICMPERRORV6")]
    pub fwpmlayeroutboundicmperrorv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDICMPERRORV6DISCARD")]
    pub fwpmlayeroutboundicmperrorv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDIPPACKETV4")]
    pub fwpmlayeroutboundippacketv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDIPPACKETV4DISCARD")]
    pub fwpmlayeroutboundippacketv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDIPPACKETV6")]
    pub fwpmlayeroutboundippacketv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDIPPACKETV6DISCARD")]
    pub fwpmlayeroutboundippacketv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDMACFRAMEETHERNET")]
    pub fwpmlayeroutboundmacframeethernet: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDMACFRAMENATIVE")]
    pub fwpmlayeroutboundmacframenative: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDMACFRAMENATIVEFAST")]
    pub fwpmlayeroutboundmacframenativefast: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV4")]
    pub fwpmlayeroutboundnetworkconnectionpolicyv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV6")]
    pub fwpmlayeroutboundnetworkconnectionpolicyv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDTRANSPORTFAST")]
    pub fwpmlayeroutboundtransportfast: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDTRANSPORTV4")]
    pub fwpmlayeroutboundtransportv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDTRANSPORTV4DISCARD")]
    pub fwpmlayeroutboundtransportv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDTRANSPORTV6")]
    pub fwpmlayeroutboundtransportv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYEROUTBOUNDTRANSPORTV6DISCARD")]
    pub fwpmlayeroutboundtransportv6_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERRPCEPADD")]
    pub fwpmlayerrpcepadd: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERRPCEPMAP")]
    pub fwpmlayerrpcepmap: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERRPCPROXYCONN")]
    pub fwpmlayerrpcproxyconn: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERRPCPROXYIF")]
    pub fwpmlayerrpcproxyif: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERRPCUM")]
    pub fwpmlayerrpcum: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMPACKETV4")]
    pub fwpmlayerstreampacketv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMPACKETV6")]
    pub fwpmlayerstreampacketv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMV4")]
    pub fwpmlayerstreamv4: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMV4DISCARD")]
    pub fwpmlayerstreamv4_discard: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMV6")]
    pub fwpmlayerstreamv6: Option<u64>,

/// 
    #[serde(rename = "FWPMLAYERSTREAMV6DISCARD")]
    pub fwpmlayerstreamv6_discard: Option<u64>,

/// 
    #[serde(rename = "Total")]
    pub total: Option<u64>,
}

impl Win32_PerfRawData_Counters_WFPClassify {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            fwpmlayeraleacceptredirectv4: None,
            fwpmlayeraleacceptredirectv6: None,
            fwpmlayeraleauthconnectv4: None,
            fwpmlayeraleauthconnectv4_discard: None,
            fwpmlayeraleauthconnectv6: None,
            fwpmlayeraleauthconnectv6_discard: None,
            fwpmlayeraleauthlistenv4: None,
            fwpmlayeraleauthlistenv4_discard: None,
            fwpmlayeraleauthlistenv6: None,
            fwpmlayeraleauthlistenv6_discard: None,
            fwpmlayeraleauthrecvacceptv4: None,
            fwpmlayeraleauthrecvacceptv4_discard: None,
            fwpmlayeraleauthrecvacceptv6: None,
            fwpmlayeraleauthrecvacceptv6_discard: None,
            fwpmlayeralebindredirectv4: None,
            fwpmlayeralebindredirectv6: None,
            fwpmlayeraleconnectredirectv4: None,
            fwpmlayeraleconnectredirectv6: None,
            fwpmlayeraleendpointclosurev4: None,
            fwpmlayeraleendpointclosurev6: None,
            fwpmlayeraleflowestablishedv4: None,
            fwpmlayeraleflowestablishedv4_discard: None,
            fwpmlayeraleflowestablishedv6: None,
            fwpmlayeraleflowestablishedv6_discard: None,
            fwpmlayeraleresourceassignmentv4: None,
            fwpmlayeraleresourceassignmentv4_discard: None,
            fwpmlayeraleresourceassignmentv6: None,
            fwpmlayeraleresourceassignmentv6_discard: None,
            fwpmlayeraleresourcereleasev4: None,
            fwpmlayeraleresourcereleasev6: None,
            fwpmlayerdatagramdatav4: None,
            fwpmlayerdatagramdatav4_discard: None,
            fwpmlayerdatagramdatav6: None,
            fwpmlayerdatagramdatav6_discard: None,
            fwpmlayeregressvswitchethernet: None,
            fwpmlayeregressvswitchtransportv4: None,
            fwpmlayeregressvswitchtransportv6: None,
            fwpmlayerikeextv4: None,
            fwpmlayerikeextv6: None,
            fwpmlayerinboundicmperrorv4: None,
            fwpmlayerinboundicmperrorv4_discard: None,
            fwpmlayerinboundicmperrorv6: None,
            fwpmlayerinboundicmperrorv6_discard: None,
            fwpmlayerinboundippacketv4: None,
            fwpmlayerinboundippacketv4_discard: None,
            fwpmlayerinboundippacketv6: None,
            fwpmlayerinboundippacketv6_discard: None,
            fwpmlayerinboundmacframeethernet: None,
            fwpmlayerinboundmacframenative: None,
            fwpmlayerinboundmacframenativefast: None,
            fwpmlayerinboundreserved2: None,
            fwpmlayerinboundtransportfast: None,
            fwpmlayerinboundtransportv4: None,
            fwpmlayerinboundtransportv4_discard: None,
            fwpmlayerinboundtransportv6: None,
            fwpmlayerinboundtransportv6_discard: None,
            fwpmlayeringressvswitchethernet: None,
            fwpmlayeringressvswitchtransportv4: None,
            fwpmlayeringressvswitchtransportv6: None,
            fwpmlayeripforwardv4: None,
            fwpmlayeripforwardv4_discard: None,
            fwpmlayeripforwardv6: None,
            fwpmlayeripforwardv6_discard: None,
            fwpmlayeripseckmdemuxv4: None,
            fwpmlayeripseckmdemuxv6: None,
            fwpmlayeripsecv4: None,
            fwpmlayeripsecv6: None,
            fwpmlayerkmauthorization: None,
            fwpmlayernameresolutioncachev4: None,
            fwpmlayernameresolutioncachev6: None,
            fwpmlayeroutboundicmperrorv4: None,
            fwpmlayeroutboundicmperrorv4_discard: None,
            fwpmlayeroutboundicmperrorv6: None,
            fwpmlayeroutboundicmperrorv6_discard: None,
            fwpmlayeroutboundippacketv4: None,
            fwpmlayeroutboundippacketv4_discard: None,
            fwpmlayeroutboundippacketv6: None,
            fwpmlayeroutboundippacketv6_discard: None,
            fwpmlayeroutboundmacframeethernet: None,
            fwpmlayeroutboundmacframenative: None,
            fwpmlayeroutboundmacframenativefast: None,
            fwpmlayeroutboundnetworkconnectionpolicyv4: None,
            fwpmlayeroutboundnetworkconnectionpolicyv6: None,
            fwpmlayeroutboundtransportfast: None,
            fwpmlayeroutboundtransportv4: None,
            fwpmlayeroutboundtransportv4_discard: None,
            fwpmlayeroutboundtransportv6: None,
            fwpmlayeroutboundtransportv6_discard: None,
            fwpmlayerrpcepadd: None,
            fwpmlayerrpcepmap: None,
            fwpmlayerrpcproxyconn: None,
            fwpmlayerrpcproxyif: None,
            fwpmlayerrpcum: None,
            fwpmlayerstreampacketv4: None,
            fwpmlayerstreampacketv6: None,
            fwpmlayerstreamv4: None,
            fwpmlayerstreamv4_discard: None,
            fwpmlayerstreamv6: None,
            fwpmlayerstreamv6_discard: None,
            total: None,
        }
    }


    /// Sets the value of FWPMLAYERALEACCEPTREDIRECTV4
    pub fn set_fwpmlayeraleacceptredirectv4(&mut self, value: u64) {
        self.fwpmlayeraleacceptredirectv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEACCEPTREDIRECTV4
    pub fn get_fwpmlayeraleacceptredirectv4(&self) -> Option<&u64> {
        self.fwpmlayeraleacceptredirectv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEACCEPTREDIRECTV6
    pub fn set_fwpmlayeraleacceptredirectv6(&mut self, value: u64) {
        self.fwpmlayeraleacceptredirectv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEACCEPTREDIRECTV6
    pub fn get_fwpmlayeraleacceptredirectv6(&self) -> Option<&u64> {
        self.fwpmlayeraleacceptredirectv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHCONNECTV4
    pub fn set_fwpmlayeraleauthconnectv4(&mut self, value: u64) {
        self.fwpmlayeraleauthconnectv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHCONNECTV4
    pub fn get_fwpmlayeraleauthconnectv4(&self) -> Option<&u64> {
        self.fwpmlayeraleauthconnectv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHCONNECTV4DISCARD
    pub fn set_fwpmlayeraleauthconnectv4_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthconnectv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHCONNECTV4DISCARD
    pub fn get_fwpmlayeraleauthconnectv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthconnectv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHCONNECTV6
    pub fn set_fwpmlayeraleauthconnectv6(&mut self, value: u64) {
        self.fwpmlayeraleauthconnectv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHCONNECTV6
    pub fn get_fwpmlayeraleauthconnectv6(&self) -> Option<&u64> {
        self.fwpmlayeraleauthconnectv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHCONNECTV6DISCARD
    pub fn set_fwpmlayeraleauthconnectv6_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthconnectv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHCONNECTV6DISCARD
    pub fn get_fwpmlayeraleauthconnectv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthconnectv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHLISTENV4
    pub fn set_fwpmlayeraleauthlistenv4(&mut self, value: u64) {
        self.fwpmlayeraleauthlistenv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHLISTENV4
    pub fn get_fwpmlayeraleauthlistenv4(&self) -> Option<&u64> {
        self.fwpmlayeraleauthlistenv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHLISTENV4DISCARD
    pub fn set_fwpmlayeraleauthlistenv4_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthlistenv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHLISTENV4DISCARD
    pub fn get_fwpmlayeraleauthlistenv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthlistenv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHLISTENV6
    pub fn set_fwpmlayeraleauthlistenv6(&mut self, value: u64) {
        self.fwpmlayeraleauthlistenv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHLISTENV6
    pub fn get_fwpmlayeraleauthlistenv6(&self) -> Option<&u64> {
        self.fwpmlayeraleauthlistenv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHLISTENV6DISCARD
    pub fn set_fwpmlayeraleauthlistenv6_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthlistenv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHLISTENV6DISCARD
    pub fn get_fwpmlayeraleauthlistenv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthlistenv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHRECVACCEPTV4
    pub fn set_fwpmlayeraleauthrecvacceptv4(&mut self, value: u64) {
        self.fwpmlayeraleauthrecvacceptv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHRECVACCEPTV4
    pub fn get_fwpmlayeraleauthrecvacceptv4(&self) -> Option<&u64> {
        self.fwpmlayeraleauthrecvacceptv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHRECVACCEPTV4DISCARD
    pub fn set_fwpmlayeraleauthrecvacceptv4_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthrecvacceptv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHRECVACCEPTV4DISCARD
    pub fn get_fwpmlayeraleauthrecvacceptv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthrecvacceptv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHRECVACCEPTV6
    pub fn set_fwpmlayeraleauthrecvacceptv6(&mut self, value: u64) {
        self.fwpmlayeraleauthrecvacceptv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHRECVACCEPTV6
    pub fn get_fwpmlayeraleauthrecvacceptv6(&self) -> Option<&u64> {
        self.fwpmlayeraleauthrecvacceptv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEAUTHRECVACCEPTV6DISCARD
    pub fn set_fwpmlayeraleauthrecvacceptv6_discard(&mut self, value: u64) {
        self.fwpmlayeraleauthrecvacceptv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEAUTHRECVACCEPTV6DISCARD
    pub fn get_fwpmlayeraleauthrecvacceptv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleauthrecvacceptv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEBINDREDIRECTV4
    pub fn set_fwpmlayeralebindredirectv4(&mut self, value: u64) {
        self.fwpmlayeralebindredirectv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEBINDREDIRECTV4
    pub fn get_fwpmlayeralebindredirectv4(&self) -> Option<&u64> {
        self.fwpmlayeralebindredirectv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEBINDREDIRECTV6
    pub fn set_fwpmlayeralebindredirectv6(&mut self, value: u64) {
        self.fwpmlayeralebindredirectv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEBINDREDIRECTV6
    pub fn get_fwpmlayeralebindredirectv6(&self) -> Option<&u64> {
        self.fwpmlayeralebindredirectv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALECONNECTREDIRECTV4
    pub fn set_fwpmlayeraleconnectredirectv4(&mut self, value: u64) {
        self.fwpmlayeraleconnectredirectv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALECONNECTREDIRECTV4
    pub fn get_fwpmlayeraleconnectredirectv4(&self) -> Option<&u64> {
        self.fwpmlayeraleconnectredirectv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALECONNECTREDIRECTV6
    pub fn set_fwpmlayeraleconnectredirectv6(&mut self, value: u64) {
        self.fwpmlayeraleconnectredirectv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALECONNECTREDIRECTV6
    pub fn get_fwpmlayeraleconnectredirectv6(&self) -> Option<&u64> {
        self.fwpmlayeraleconnectredirectv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEENDPOINTCLOSUREV4
    pub fn set_fwpmlayeraleendpointclosurev4(&mut self, value: u64) {
        self.fwpmlayeraleendpointclosurev4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEENDPOINTCLOSUREV4
    pub fn get_fwpmlayeraleendpointclosurev4(&self) -> Option<&u64> {
        self.fwpmlayeraleendpointclosurev4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEENDPOINTCLOSUREV6
    pub fn set_fwpmlayeraleendpointclosurev6(&mut self, value: u64) {
        self.fwpmlayeraleendpointclosurev6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEENDPOINTCLOSUREV6
    pub fn get_fwpmlayeraleendpointclosurev6(&self) -> Option<&u64> {
        self.fwpmlayeraleendpointclosurev6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEFLOWESTABLISHEDV4
    pub fn set_fwpmlayeraleflowestablishedv4(&mut self, value: u64) {
        self.fwpmlayeraleflowestablishedv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEFLOWESTABLISHEDV4
    pub fn get_fwpmlayeraleflowestablishedv4(&self) -> Option<&u64> {
        self.fwpmlayeraleflowestablishedv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALEFLOWESTABLISHEDV4DISCARD
    pub fn set_fwpmlayeraleflowestablishedv4_discard(&mut self, value: u64) {
        self.fwpmlayeraleflowestablishedv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEFLOWESTABLISHEDV4DISCARD
    pub fn get_fwpmlayeraleflowestablishedv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleflowestablishedv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALEFLOWESTABLISHEDV6
    pub fn set_fwpmlayeraleflowestablishedv6(&mut self, value: u64) {
        self.fwpmlayeraleflowestablishedv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALEFLOWESTABLISHEDV6
    pub fn get_fwpmlayeraleflowestablishedv6(&self) -> Option<&u64> {
        self.fwpmlayeraleflowestablishedv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALEFLOWESTABLISHEDV6DISCARD
    pub fn set_fwpmlayeraleflowestablishedv6_discard(&mut self, value: u64) {
        self.fwpmlayeraleflowestablishedv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALEFLOWESTABLISHEDV6DISCARD
    pub fn get_fwpmlayeraleflowestablishedv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleflowestablishedv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCEASSIGNMENTV4
    pub fn set_fwpmlayeraleresourceassignmentv4(&mut self, value: u64) {
        self.fwpmlayeraleresourceassignmentv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCEASSIGNMENTV4
    pub fn get_fwpmlayeraleresourceassignmentv4(&self) -> Option<&u64> {
        self.fwpmlayeraleresourceassignmentv4.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCEASSIGNMENTV4DISCARD
    pub fn set_fwpmlayeraleresourceassignmentv4_discard(&mut self, value: u64) {
        self.fwpmlayeraleresourceassignmentv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCEASSIGNMENTV4DISCARD
    pub fn get_fwpmlayeraleresourceassignmentv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleresourceassignmentv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCEASSIGNMENTV6
    pub fn set_fwpmlayeraleresourceassignmentv6(&mut self, value: u64) {
        self.fwpmlayeraleresourceassignmentv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCEASSIGNMENTV6
    pub fn get_fwpmlayeraleresourceassignmentv6(&self) -> Option<&u64> {
        self.fwpmlayeraleresourceassignmentv6.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCEASSIGNMENTV6DISCARD
    pub fn set_fwpmlayeraleresourceassignmentv6_discard(&mut self, value: u64) {
        self.fwpmlayeraleresourceassignmentv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCEASSIGNMENTV6DISCARD
    pub fn get_fwpmlayeraleresourceassignmentv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeraleresourceassignmentv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCERELEASEV4
    pub fn set_fwpmlayeraleresourcereleasev4(&mut self, value: u64) {
        self.fwpmlayeraleresourcereleasev4 = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCERELEASEV4
    pub fn get_fwpmlayeraleresourcereleasev4(&self) -> Option<&u64> {
        self.fwpmlayeraleresourcereleasev4.as_ref()
    }

    /// Sets the value of FWPMLAYERALERESOURCERELEASEV6
    pub fn set_fwpmlayeraleresourcereleasev6(&mut self, value: u64) {
        self.fwpmlayeraleresourcereleasev6 = Some(value);
    }

    /// Gets the value of FWPMLAYERALERESOURCERELEASEV6
    pub fn get_fwpmlayeraleresourcereleasev6(&self) -> Option<&u64> {
        self.fwpmlayeraleresourcereleasev6.as_ref()
    }

    /// Sets the value of FWPMLAYERDATAGRAMDATAV4
    pub fn set_fwpmlayerdatagramdatav4(&mut self, value: u64) {
        self.fwpmlayerdatagramdatav4 = Some(value);
    }

    /// Gets the value of FWPMLAYERDATAGRAMDATAV4
    pub fn get_fwpmlayerdatagramdatav4(&self) -> Option<&u64> {
        self.fwpmlayerdatagramdatav4.as_ref()
    }

    /// Sets the value of FWPMLAYERDATAGRAMDATAV4DISCARD
    pub fn set_fwpmlayerdatagramdatav4_discard(&mut self, value: u64) {
        self.fwpmlayerdatagramdatav4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERDATAGRAMDATAV4DISCARD
    pub fn get_fwpmlayerdatagramdatav4_discard(&self) -> Option<&u64> {
        self.fwpmlayerdatagramdatav4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERDATAGRAMDATAV6
    pub fn set_fwpmlayerdatagramdatav6(&mut self, value: u64) {
        self.fwpmlayerdatagramdatav6 = Some(value);
    }

    /// Gets the value of FWPMLAYERDATAGRAMDATAV6
    pub fn get_fwpmlayerdatagramdatav6(&self) -> Option<&u64> {
        self.fwpmlayerdatagramdatav6.as_ref()
    }

    /// Sets the value of FWPMLAYERDATAGRAMDATAV6DISCARD
    pub fn set_fwpmlayerdatagramdatav6_discard(&mut self, value: u64) {
        self.fwpmlayerdatagramdatav6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERDATAGRAMDATAV6DISCARD
    pub fn get_fwpmlayerdatagramdatav6_discard(&self) -> Option<&u64> {
        self.fwpmlayerdatagramdatav6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEREGRESSVSWITCHETHERNET
    pub fn set_fwpmlayeregressvswitchethernet(&mut self, value: u64) {
        self.fwpmlayeregressvswitchethernet = Some(value);
    }

    /// Gets the value of FWPMLAYEREGRESSVSWITCHETHERNET
    pub fn get_fwpmlayeregressvswitchethernet(&self) -> Option<&u64> {
        self.fwpmlayeregressvswitchethernet.as_ref()
    }

    /// Sets the value of FWPMLAYEREGRESSVSWITCHTRANSPORTV4
    pub fn set_fwpmlayeregressvswitchtransportv4(&mut self, value: u64) {
        self.fwpmlayeregressvswitchtransportv4 = Some(value);
    }

    /// Gets the value of FWPMLAYEREGRESSVSWITCHTRANSPORTV4
    pub fn get_fwpmlayeregressvswitchtransportv4(&self) -> Option<&u64> {
        self.fwpmlayeregressvswitchtransportv4.as_ref()
    }

    /// Sets the value of FWPMLAYEREGRESSVSWITCHTRANSPORTV6
    pub fn set_fwpmlayeregressvswitchtransportv6(&mut self, value: u64) {
        self.fwpmlayeregressvswitchtransportv6 = Some(value);
    }

    /// Gets the value of FWPMLAYEREGRESSVSWITCHTRANSPORTV6
    pub fn get_fwpmlayeregressvswitchtransportv6(&self) -> Option<&u64> {
        self.fwpmlayeregressvswitchtransportv6.as_ref()
    }

    /// Sets the value of FWPMLAYERIKEEXTV4
    pub fn set_fwpmlayerikeextv4(&mut self, value: u64) {
        self.fwpmlayerikeextv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERIKEEXTV4
    pub fn get_fwpmlayerikeextv4(&self) -> Option<&u64> {
        self.fwpmlayerikeextv4.as_ref()
    }

    /// Sets the value of FWPMLAYERIKEEXTV6
    pub fn set_fwpmlayerikeextv6(&mut self, value: u64) {
        self.fwpmlayerikeextv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERIKEEXTV6
    pub fn get_fwpmlayerikeextv6(&self) -> Option<&u64> {
        self.fwpmlayerikeextv6.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDICMPERRORV4
    pub fn set_fwpmlayerinboundicmperrorv4(&mut self, value: u64) {
        self.fwpmlayerinboundicmperrorv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDICMPERRORV4
    pub fn get_fwpmlayerinboundicmperrorv4(&self) -> Option<&u64> {
        self.fwpmlayerinboundicmperrorv4.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDICMPERRORV4DISCARD
    pub fn set_fwpmlayerinboundicmperrorv4_discard(&mut self, value: u64) {
        self.fwpmlayerinboundicmperrorv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDICMPERRORV4DISCARD
    pub fn get_fwpmlayerinboundicmperrorv4_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundicmperrorv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDICMPERRORV6
    pub fn set_fwpmlayerinboundicmperrorv6(&mut self, value: u64) {
        self.fwpmlayerinboundicmperrorv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDICMPERRORV6
    pub fn get_fwpmlayerinboundicmperrorv6(&self) -> Option<&u64> {
        self.fwpmlayerinboundicmperrorv6.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDICMPERRORV6DISCARD
    pub fn set_fwpmlayerinboundicmperrorv6_discard(&mut self, value: u64) {
        self.fwpmlayerinboundicmperrorv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDICMPERRORV6DISCARD
    pub fn get_fwpmlayerinboundicmperrorv6_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundicmperrorv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDIPPACKETV4
    pub fn set_fwpmlayerinboundippacketv4(&mut self, value: u64) {
        self.fwpmlayerinboundippacketv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDIPPACKETV4
    pub fn get_fwpmlayerinboundippacketv4(&self) -> Option<&u64> {
        self.fwpmlayerinboundippacketv4.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDIPPACKETV4DISCARD
    pub fn set_fwpmlayerinboundippacketv4_discard(&mut self, value: u64) {
        self.fwpmlayerinboundippacketv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDIPPACKETV4DISCARD
    pub fn get_fwpmlayerinboundippacketv4_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundippacketv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDIPPACKETV6
    pub fn set_fwpmlayerinboundippacketv6(&mut self, value: u64) {
        self.fwpmlayerinboundippacketv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDIPPACKETV6
    pub fn get_fwpmlayerinboundippacketv6(&self) -> Option<&u64> {
        self.fwpmlayerinboundippacketv6.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDIPPACKETV6DISCARD
    pub fn set_fwpmlayerinboundippacketv6_discard(&mut self, value: u64) {
        self.fwpmlayerinboundippacketv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDIPPACKETV6DISCARD
    pub fn get_fwpmlayerinboundippacketv6_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundippacketv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDMACFRAMEETHERNET
    pub fn set_fwpmlayerinboundmacframeethernet(&mut self, value: u64) {
        self.fwpmlayerinboundmacframeethernet = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDMACFRAMEETHERNET
    pub fn get_fwpmlayerinboundmacframeethernet(&self) -> Option<&u64> {
        self.fwpmlayerinboundmacframeethernet.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDMACFRAMENATIVE
    pub fn set_fwpmlayerinboundmacframenative(&mut self, value: u64) {
        self.fwpmlayerinboundmacframenative = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDMACFRAMENATIVE
    pub fn get_fwpmlayerinboundmacframenative(&self) -> Option<&u64> {
        self.fwpmlayerinboundmacframenative.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDMACFRAMENATIVEFAST
    pub fn set_fwpmlayerinboundmacframenativefast(&mut self, value: u64) {
        self.fwpmlayerinboundmacframenativefast = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDMACFRAMENATIVEFAST
    pub fn get_fwpmlayerinboundmacframenativefast(&self) -> Option<&u64> {
        self.fwpmlayerinboundmacframenativefast.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDRESERVED2
    pub fn set_fwpmlayerinboundreserved2(&mut self, value: u64) {
        self.fwpmlayerinboundreserved2 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDRESERVED2
    pub fn get_fwpmlayerinboundreserved2(&self) -> Option<&u64> {
        self.fwpmlayerinboundreserved2.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDTRANSPORTFAST
    pub fn set_fwpmlayerinboundtransportfast(&mut self, value: u64) {
        self.fwpmlayerinboundtransportfast = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDTRANSPORTFAST
    pub fn get_fwpmlayerinboundtransportfast(&self) -> Option<&u64> {
        self.fwpmlayerinboundtransportfast.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDTRANSPORTV4
    pub fn set_fwpmlayerinboundtransportv4(&mut self, value: u64) {
        self.fwpmlayerinboundtransportv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDTRANSPORTV4
    pub fn get_fwpmlayerinboundtransportv4(&self) -> Option<&u64> {
        self.fwpmlayerinboundtransportv4.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDTRANSPORTV4DISCARD
    pub fn set_fwpmlayerinboundtransportv4_discard(&mut self, value: u64) {
        self.fwpmlayerinboundtransportv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDTRANSPORTV4DISCARD
    pub fn get_fwpmlayerinboundtransportv4_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundtransportv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDTRANSPORTV6
    pub fn set_fwpmlayerinboundtransportv6(&mut self, value: u64) {
        self.fwpmlayerinboundtransportv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDTRANSPORTV6
    pub fn get_fwpmlayerinboundtransportv6(&self) -> Option<&u64> {
        self.fwpmlayerinboundtransportv6.as_ref()
    }

    /// Sets the value of FWPMLAYERINBOUNDTRANSPORTV6DISCARD
    pub fn set_fwpmlayerinboundtransportv6_discard(&mut self, value: u64) {
        self.fwpmlayerinboundtransportv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERINBOUNDTRANSPORTV6DISCARD
    pub fn get_fwpmlayerinboundtransportv6_discard(&self) -> Option<&u64> {
        self.fwpmlayerinboundtransportv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERINGRESSVSWITCHETHERNET
    pub fn set_fwpmlayeringressvswitchethernet(&mut self, value: u64) {
        self.fwpmlayeringressvswitchethernet = Some(value);
    }

    /// Gets the value of FWPMLAYERINGRESSVSWITCHETHERNET
    pub fn get_fwpmlayeringressvswitchethernet(&self) -> Option<&u64> {
        self.fwpmlayeringressvswitchethernet.as_ref()
    }

    /// Sets the value of FWPMLAYERINGRESSVSWITCHTRANSPORTV4
    pub fn set_fwpmlayeringressvswitchtransportv4(&mut self, value: u64) {
        self.fwpmlayeringressvswitchtransportv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERINGRESSVSWITCHTRANSPORTV4
    pub fn get_fwpmlayeringressvswitchtransportv4(&self) -> Option<&u64> {
        self.fwpmlayeringressvswitchtransportv4.as_ref()
    }

    /// Sets the value of FWPMLAYERINGRESSVSWITCHTRANSPORTV6
    pub fn set_fwpmlayeringressvswitchtransportv6(&mut self, value: u64) {
        self.fwpmlayeringressvswitchtransportv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERINGRESSVSWITCHTRANSPORTV6
    pub fn get_fwpmlayeringressvswitchtransportv6(&self) -> Option<&u64> {
        self.fwpmlayeringressvswitchtransportv6.as_ref()
    }

    /// Sets the value of FWPMLAYERIPFORWARDV4
    pub fn set_fwpmlayeripforwardv4(&mut self, value: u64) {
        self.fwpmlayeripforwardv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPFORWARDV4
    pub fn get_fwpmlayeripforwardv4(&self) -> Option<&u64> {
        self.fwpmlayeripforwardv4.as_ref()
    }

    /// Sets the value of FWPMLAYERIPFORWARDV4DISCARD
    pub fn set_fwpmlayeripforwardv4_discard(&mut self, value: u64) {
        self.fwpmlayeripforwardv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERIPFORWARDV4DISCARD
    pub fn get_fwpmlayeripforwardv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeripforwardv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERIPFORWARDV6
    pub fn set_fwpmlayeripforwardv6(&mut self, value: u64) {
        self.fwpmlayeripforwardv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPFORWARDV6
    pub fn get_fwpmlayeripforwardv6(&self) -> Option<&u64> {
        self.fwpmlayeripforwardv6.as_ref()
    }

    /// Sets the value of FWPMLAYERIPFORWARDV6DISCARD
    pub fn set_fwpmlayeripforwardv6_discard(&mut self, value: u64) {
        self.fwpmlayeripforwardv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERIPFORWARDV6DISCARD
    pub fn get_fwpmlayeripforwardv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeripforwardv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERIPSECKMDEMUXV4
    pub fn set_fwpmlayeripseckmdemuxv4(&mut self, value: u64) {
        self.fwpmlayeripseckmdemuxv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPSECKMDEMUXV4
    pub fn get_fwpmlayeripseckmdemuxv4(&self) -> Option<&u64> {
        self.fwpmlayeripseckmdemuxv4.as_ref()
    }

    /// Sets the value of FWPMLAYERIPSECKMDEMUXV6
    pub fn set_fwpmlayeripseckmdemuxv6(&mut self, value: u64) {
        self.fwpmlayeripseckmdemuxv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPSECKMDEMUXV6
    pub fn get_fwpmlayeripseckmdemuxv6(&self) -> Option<&u64> {
        self.fwpmlayeripseckmdemuxv6.as_ref()
    }

    /// Sets the value of FWPMLAYERIPSECV4
    pub fn set_fwpmlayeripsecv4(&mut self, value: u64) {
        self.fwpmlayeripsecv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPSECV4
    pub fn get_fwpmlayeripsecv4(&self) -> Option<&u64> {
        self.fwpmlayeripsecv4.as_ref()
    }

    /// Sets the value of FWPMLAYERIPSECV6
    pub fn set_fwpmlayeripsecv6(&mut self, value: u64) {
        self.fwpmlayeripsecv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERIPSECV6
    pub fn get_fwpmlayeripsecv6(&self) -> Option<&u64> {
        self.fwpmlayeripsecv6.as_ref()
    }

    /// Sets the value of FWPMLAYERKMAUTHORIZATION
    pub fn set_fwpmlayerkmauthorization(&mut self, value: u64) {
        self.fwpmlayerkmauthorization = Some(value);
    }

    /// Gets the value of FWPMLAYERKMAUTHORIZATION
    pub fn get_fwpmlayerkmauthorization(&self) -> Option<&u64> {
        self.fwpmlayerkmauthorization.as_ref()
    }

    /// Sets the value of FWPMLAYERNAMERESOLUTIONCACHEV4
    pub fn set_fwpmlayernameresolutioncachev4(&mut self, value: u64) {
        self.fwpmlayernameresolutioncachev4 = Some(value);
    }

    /// Gets the value of FWPMLAYERNAMERESOLUTIONCACHEV4
    pub fn get_fwpmlayernameresolutioncachev4(&self) -> Option<&u64> {
        self.fwpmlayernameresolutioncachev4.as_ref()
    }

    /// Sets the value of FWPMLAYERNAMERESOLUTIONCACHEV6
    pub fn set_fwpmlayernameresolutioncachev6(&mut self, value: u64) {
        self.fwpmlayernameresolutioncachev6 = Some(value);
    }

    /// Gets the value of FWPMLAYERNAMERESOLUTIONCACHEV6
    pub fn get_fwpmlayernameresolutioncachev6(&self) -> Option<&u64> {
        self.fwpmlayernameresolutioncachev6.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDICMPERRORV4
    pub fn set_fwpmlayeroutboundicmperrorv4(&mut self, value: u64) {
        self.fwpmlayeroutboundicmperrorv4 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDICMPERRORV4
    pub fn get_fwpmlayeroutboundicmperrorv4(&self) -> Option<&u64> {
        self.fwpmlayeroutboundicmperrorv4.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDICMPERRORV4DISCARD
    pub fn set_fwpmlayeroutboundicmperrorv4_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundicmperrorv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDICMPERRORV4DISCARD
    pub fn get_fwpmlayeroutboundicmperrorv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundicmperrorv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDICMPERRORV6
    pub fn set_fwpmlayeroutboundicmperrorv6(&mut self, value: u64) {
        self.fwpmlayeroutboundicmperrorv6 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDICMPERRORV6
    pub fn get_fwpmlayeroutboundicmperrorv6(&self) -> Option<&u64> {
        self.fwpmlayeroutboundicmperrorv6.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDICMPERRORV6DISCARD
    pub fn set_fwpmlayeroutboundicmperrorv6_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundicmperrorv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDICMPERRORV6DISCARD
    pub fn get_fwpmlayeroutboundicmperrorv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundicmperrorv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDIPPACKETV4
    pub fn set_fwpmlayeroutboundippacketv4(&mut self, value: u64) {
        self.fwpmlayeroutboundippacketv4 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDIPPACKETV4
    pub fn get_fwpmlayeroutboundippacketv4(&self) -> Option<&u64> {
        self.fwpmlayeroutboundippacketv4.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDIPPACKETV4DISCARD
    pub fn set_fwpmlayeroutboundippacketv4_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundippacketv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDIPPACKETV4DISCARD
    pub fn get_fwpmlayeroutboundippacketv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundippacketv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDIPPACKETV6
    pub fn set_fwpmlayeroutboundippacketv6(&mut self, value: u64) {
        self.fwpmlayeroutboundippacketv6 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDIPPACKETV6
    pub fn get_fwpmlayeroutboundippacketv6(&self) -> Option<&u64> {
        self.fwpmlayeroutboundippacketv6.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDIPPACKETV6DISCARD
    pub fn set_fwpmlayeroutboundippacketv6_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundippacketv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDIPPACKETV6DISCARD
    pub fn get_fwpmlayeroutboundippacketv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundippacketv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDMACFRAMEETHERNET
    pub fn set_fwpmlayeroutboundmacframeethernet(&mut self, value: u64) {
        self.fwpmlayeroutboundmacframeethernet = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDMACFRAMEETHERNET
    pub fn get_fwpmlayeroutboundmacframeethernet(&self) -> Option<&u64> {
        self.fwpmlayeroutboundmacframeethernet.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDMACFRAMENATIVE
    pub fn set_fwpmlayeroutboundmacframenative(&mut self, value: u64) {
        self.fwpmlayeroutboundmacframenative = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDMACFRAMENATIVE
    pub fn get_fwpmlayeroutboundmacframenative(&self) -> Option<&u64> {
        self.fwpmlayeroutboundmacframenative.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDMACFRAMENATIVEFAST
    pub fn set_fwpmlayeroutboundmacframenativefast(&mut self, value: u64) {
        self.fwpmlayeroutboundmacframenativefast = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDMACFRAMENATIVEFAST
    pub fn get_fwpmlayeroutboundmacframenativefast(&self) -> Option<&u64> {
        self.fwpmlayeroutboundmacframenativefast.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV4
    pub fn set_fwpmlayeroutboundnetworkconnectionpolicyv4(&mut self, value: u64) {
        self.fwpmlayeroutboundnetworkconnectionpolicyv4 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV4
    pub fn get_fwpmlayeroutboundnetworkconnectionpolicyv4(&self) -> Option<&u64> {
        self.fwpmlayeroutboundnetworkconnectionpolicyv4.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV6
    pub fn set_fwpmlayeroutboundnetworkconnectionpolicyv6(&mut self, value: u64) {
        self.fwpmlayeroutboundnetworkconnectionpolicyv6 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDNETWORKCONNECTIONPOLICYV6
    pub fn get_fwpmlayeroutboundnetworkconnectionpolicyv6(&self) -> Option<&u64> {
        self.fwpmlayeroutboundnetworkconnectionpolicyv6.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDTRANSPORTFAST
    pub fn set_fwpmlayeroutboundtransportfast(&mut self, value: u64) {
        self.fwpmlayeroutboundtransportfast = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDTRANSPORTFAST
    pub fn get_fwpmlayeroutboundtransportfast(&self) -> Option<&u64> {
        self.fwpmlayeroutboundtransportfast.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDTRANSPORTV4
    pub fn set_fwpmlayeroutboundtransportv4(&mut self, value: u64) {
        self.fwpmlayeroutboundtransportv4 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDTRANSPORTV4
    pub fn get_fwpmlayeroutboundtransportv4(&self) -> Option<&u64> {
        self.fwpmlayeroutboundtransportv4.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDTRANSPORTV4DISCARD
    pub fn set_fwpmlayeroutboundtransportv4_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundtransportv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDTRANSPORTV4DISCARD
    pub fn get_fwpmlayeroutboundtransportv4_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundtransportv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDTRANSPORTV6
    pub fn set_fwpmlayeroutboundtransportv6(&mut self, value: u64) {
        self.fwpmlayeroutboundtransportv6 = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDTRANSPORTV6
    pub fn get_fwpmlayeroutboundtransportv6(&self) -> Option<&u64> {
        self.fwpmlayeroutboundtransportv6.as_ref()
    }

    /// Sets the value of FWPMLAYEROUTBOUNDTRANSPORTV6DISCARD
    pub fn set_fwpmlayeroutboundtransportv6_discard(&mut self, value: u64) {
        self.fwpmlayeroutboundtransportv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYEROUTBOUNDTRANSPORTV6DISCARD
    pub fn get_fwpmlayeroutboundtransportv6_discard(&self) -> Option<&u64> {
        self.fwpmlayeroutboundtransportv6_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERRPCEPADD
    pub fn set_fwpmlayerrpcepadd(&mut self, value: u64) {
        self.fwpmlayerrpcepadd = Some(value);
    }

    /// Gets the value of FWPMLAYERRPCEPADD
    pub fn get_fwpmlayerrpcepadd(&self) -> Option<&u64> {
        self.fwpmlayerrpcepadd.as_ref()
    }

    /// Sets the value of FWPMLAYERRPCEPMAP
    pub fn set_fwpmlayerrpcepmap(&mut self, value: u64) {
        self.fwpmlayerrpcepmap = Some(value);
    }

    /// Gets the value of FWPMLAYERRPCEPMAP
    pub fn get_fwpmlayerrpcepmap(&self) -> Option<&u64> {
        self.fwpmlayerrpcepmap.as_ref()
    }

    /// Sets the value of FWPMLAYERRPCPROXYCONN
    pub fn set_fwpmlayerrpcproxyconn(&mut self, value: u64) {
        self.fwpmlayerrpcproxyconn = Some(value);
    }

    /// Gets the value of FWPMLAYERRPCPROXYCONN
    pub fn get_fwpmlayerrpcproxyconn(&self) -> Option<&u64> {
        self.fwpmlayerrpcproxyconn.as_ref()
    }

    /// Sets the value of FWPMLAYERRPCPROXYIF
    pub fn set_fwpmlayerrpcproxyif(&mut self, value: u64) {
        self.fwpmlayerrpcproxyif = Some(value);
    }

    /// Gets the value of FWPMLAYERRPCPROXYIF
    pub fn get_fwpmlayerrpcproxyif(&self) -> Option<&u64> {
        self.fwpmlayerrpcproxyif.as_ref()
    }

    /// Sets the value of FWPMLAYERRPCUM
    pub fn set_fwpmlayerrpcum(&mut self, value: u64) {
        self.fwpmlayerrpcum = Some(value);
    }

    /// Gets the value of FWPMLAYERRPCUM
    pub fn get_fwpmlayerrpcum(&self) -> Option<&u64> {
        self.fwpmlayerrpcum.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMPACKETV4
    pub fn set_fwpmlayerstreampacketv4(&mut self, value: u64) {
        self.fwpmlayerstreampacketv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMPACKETV4
    pub fn get_fwpmlayerstreampacketv4(&self) -> Option<&u64> {
        self.fwpmlayerstreampacketv4.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMPACKETV6
    pub fn set_fwpmlayerstreampacketv6(&mut self, value: u64) {
        self.fwpmlayerstreampacketv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMPACKETV6
    pub fn get_fwpmlayerstreampacketv6(&self) -> Option<&u64> {
        self.fwpmlayerstreampacketv6.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMV4
    pub fn set_fwpmlayerstreamv4(&mut self, value: u64) {
        self.fwpmlayerstreamv4 = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMV4
    pub fn get_fwpmlayerstreamv4(&self) -> Option<&u64> {
        self.fwpmlayerstreamv4.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMV4DISCARD
    pub fn set_fwpmlayerstreamv4_discard(&mut self, value: u64) {
        self.fwpmlayerstreamv4_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMV4DISCARD
    pub fn get_fwpmlayerstreamv4_discard(&self) -> Option<&u64> {
        self.fwpmlayerstreamv4_discard.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMV6
    pub fn set_fwpmlayerstreamv6(&mut self, value: u64) {
        self.fwpmlayerstreamv6 = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMV6
    pub fn get_fwpmlayerstreamv6(&self) -> Option<&u64> {
        self.fwpmlayerstreamv6.as_ref()
    }

    /// Sets the value of FWPMLAYERSTREAMV6DISCARD
    pub fn set_fwpmlayerstreamv6_discard(&mut self, value: u64) {
        self.fwpmlayerstreamv6_discard = Some(value);
    }

    /// Gets the value of FWPMLAYERSTREAMV6DISCARD
    pub fn get_fwpmlayerstreamv6_discard(&self) -> Option<&u64> {
        self.fwpmlayerstreamv6_discard.as_ref()
    }

    /// Sets the value of Total
    pub fn set_total(&mut self, value: u64) {
        self.total = Some(value);
    }

    /// Gets the value of Total
    pub fn get_total(&self) -> Option<&u64> {
        self.total.as_ref()
    }
}


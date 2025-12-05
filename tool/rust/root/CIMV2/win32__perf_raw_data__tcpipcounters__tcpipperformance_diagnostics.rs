// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnostics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnostics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Bytesoflostretransmitsretransmitted")]
    pub bytesoflostretransmitsretransmitted: Option<u32>,

/// 
    #[serde(rename = "Deniedconnectorsendrequestsinlowpowermode")]
    pub deniedconnectorsendrequestsinlowpowermode: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLsindicatedwithlowresourceflag")]
    pub ipv4_nblsindicatedwithlowresourceflag: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLsindicatedwithoutprevalidation")]
    pub ipv4_nblsindicatedwithoutprevalidation: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLsPersecindicatedwithlowresourceflag")]
    pub ipv4_nbls_persecindicatedwithlowresourceflag: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLsPersecindicatedwithoutprevalidation")]
    pub ipv4_nbls_persecindicatedwithoutprevalidation: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLsPersectreatedasnonprevalidated")]
    pub ipv4_nbls_persectreatedasnonprevalidated: Option<u32>,

/// 
    #[serde(rename = "IPv4NBLstreatedasnonprevalidated")]
    pub ipv4_nblstreatedasnonprevalidated: Option<u32>,

/// 
    #[serde(rename = "IPv4outboundNBLsnotprocessedviafastpath")]
    pub ipv4outbound_nblsnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "IPv4outboundNBLsPersecnotprocessedviafastpath")]
    pub ipv4outbound_nbls_persecnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLsindicatedwithlowresourceflag")]
    pub ipv6_nblsindicatedwithlowresourceflag: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLsindicatedwithoutprevalidation")]
    pub ipv6_nblsindicatedwithoutprevalidation: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLsPersecindicatedwithlowresourceflag")]
    pub ipv6_nbls_persecindicatedwithlowresourceflag: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLsPersecindicatedwithoutprevalidation")]
    pub ipv6_nbls_persecindicatedwithoutprevalidation: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLsPersectreatedasnonprevalidated")]
    pub ipv6_nbls_persectreatedasnonprevalidated: Option<u32>,

/// 
    #[serde(rename = "IPv6NBLstreatedasnonprevalidated")]
    pub ipv6_nblstreatedasnonprevalidated: Option<u32>,

/// 
    #[serde(rename = "IPv6outboundNBLsnotprocessedviafastpath")]
    pub ipv6outbound_nblsnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "IPv6outboundNBLsPersecnotprocessedviafastpath")]
    pub ipv6outbound_nbls_persecnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "NumberofSACKblocksdropped")]
    pub numberof_sackblocksdropped: Option<u32>,

/// 
    #[serde(rename = "NumberofTCPRXfastpathbatchesinspected")]
    pub numberof_tcprxfastpathbatchesinspected: Option<u64>,

/// 
    #[serde(rename = "NumberofTCPRXfastpathbatchesnotinspected")]
    pub numberof_tcprxfastpathbatchesnotinspected: Option<u64>,

/// 
    #[serde(rename = "NumberofUSOpacketssegmentedandchecksummedinsoftware")]
    pub numberof_usopacketssegmentedandchecksummedinsoftware: Option<u32>,

/// 
    #[serde(rename = "NumberofUSOpacketssegmentedinsoftware")]
    pub numberof_usopacketssegmentedinsoftware: Option<u32>,

/// 
    #[serde(rename = "RSCsegmentforwardingfailuresduringsoftwaresegmentation")]
    pub rscsegmentforwardingfailuresduringsoftwaresegmentation: Option<u32>,

/// 
    #[serde(rename = "RSCsegmentsforwardedviaLSO")]
    pub rscsegmentsforwardedvia_lso: Option<u32>,

/// 
    #[serde(rename = "RSCsegmentsforwardedviasoftwaresegmentation")]
    pub rscsegmentsforwardedviasoftwaresegmentation: Option<u32>,

/// 
    #[serde(rename = "RSCsegmentsforwardedviasoftwaresegmentationandchecksum")]
    pub rscsegmentsforwardedviasoftwaresegmentationandchecksum: Option<u32>,

/// 
    #[serde(rename = "TCPchecksumerrors")]
    pub tcpchecksumerrors: Option<u32>,

/// 
    #[serde(rename = "TCPconnectrequestsfallenoffloopbackfastpath")]
    pub tcpconnectrequestsfallenoffloopbackfastpath: Option<u32>,

/// 
    #[serde(rename = "TCPconnectrequestsPersecfallenoffloopbackfastpath")]
    pub tcpconnectrequests_persecfallenoffloopbackfastpath: Option<u32>,

/// 
    #[serde(rename = "TCPinboundsegmentsnotprocessedviafastpath")]
    pub tcpinboundsegmentsnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "TCPinboundsegmentsPersecnotprocessedviafastpath")]
    pub tcpinboundsegments_persecnotprocessedviafastpath: Option<u32>,

/// 
    #[serde(rename = "TCPlossrecoveryepisodes")]
    pub tcplossrecoveryepisodes: Option<u32>,

/// 
    #[serde(rename = "TCPRSCbytesreceived")]
    pub tcprscbytesreceived: Option<u32>,

/// 
    #[serde(rename = "TCPRSCevents")]
    pub tcprscevents: Option<u32>,

/// 
    #[serde(rename = "TCPsuccessfullossrecoveryepisodes")]
    pub tcpsuccessfullossrecoveryepisodes: Option<u32>,

/// 
    #[serde(rename = "TCPtimeouts")]
    pub tcptimeouts: Option<u32>,

/// 
    #[serde(rename = "UDPdatagramscreatedviasoftwaresegmentation")]
    pub udpdatagramscreatedviasoftwaresegmentation: Option<u32>,

/// 
    #[serde(rename = "UDPURObytesreceived")]
    pub udpurobytesreceived: Option<u32>,

/// 
    #[serde(rename = "UDPUROevents")]
    pub udpuroevents: Option<u32>,

/// 
    #[serde(rename = "UROsegmentationfailuresforrawsockets")]
    pub urosegmentationfailuresforrawsockets: Option<u32>,

/// 
    #[serde(rename = "UROsegmentationsforrawsockets")]
    pub urosegmentationsforrawsockets: Option<u32>,

/// 
    #[serde(rename = "UROsegmentforwardingfailuresduringsoftwaresegmentation")]
    pub urosegmentforwardingfailuresduringsoftwaresegmentation: Option<u32>,

/// 
    #[serde(rename = "UROsegmentsforwardedviasoftwaresegmentation")]
    pub urosegmentsforwardedviasoftwaresegmentation: Option<u32>,

/// 
    #[serde(rename = "UROsegmentsforwardedviasoftwaresegmentationandchecksum")]
    pub urosegmentsforwardedviasoftwaresegmentationandchecksum: Option<u32>,
}

impl Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnostics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytesoflostretransmitsretransmitted: None,
            deniedconnectorsendrequestsinlowpowermode: None,
            ipv4_nblsindicatedwithlowresourceflag: None,
            ipv4_nblsindicatedwithoutprevalidation: None,
            ipv4_nbls_persecindicatedwithlowresourceflag: None,
            ipv4_nbls_persecindicatedwithoutprevalidation: None,
            ipv4_nbls_persectreatedasnonprevalidated: None,
            ipv4_nblstreatedasnonprevalidated: None,
            ipv4outbound_nblsnotprocessedviafastpath: None,
            ipv4outbound_nbls_persecnotprocessedviafastpath: None,
            ipv6_nblsindicatedwithlowresourceflag: None,
            ipv6_nblsindicatedwithoutprevalidation: None,
            ipv6_nbls_persecindicatedwithlowresourceflag: None,
            ipv6_nbls_persecindicatedwithoutprevalidation: None,
            ipv6_nbls_persectreatedasnonprevalidated: None,
            ipv6_nblstreatedasnonprevalidated: None,
            ipv6outbound_nblsnotprocessedviafastpath: None,
            ipv6outbound_nbls_persecnotprocessedviafastpath: None,
            numberof_sackblocksdropped: None,
            numberof_tcprxfastpathbatchesinspected: None,
            numberof_tcprxfastpathbatchesnotinspected: None,
            numberof_usopacketssegmentedandchecksummedinsoftware: None,
            numberof_usopacketssegmentedinsoftware: None,
            rscsegmentforwardingfailuresduringsoftwaresegmentation: None,
            rscsegmentsforwardedvia_lso: None,
            rscsegmentsforwardedviasoftwaresegmentation: None,
            rscsegmentsforwardedviasoftwaresegmentationandchecksum: None,
            tcpchecksumerrors: None,
            tcpconnectrequestsfallenoffloopbackfastpath: None,
            tcpconnectrequests_persecfallenoffloopbackfastpath: None,
            tcpinboundsegmentsnotprocessedviafastpath: None,
            tcpinboundsegments_persecnotprocessedviafastpath: None,
            tcplossrecoveryepisodes: None,
            tcprscbytesreceived: None,
            tcprscevents: None,
            tcpsuccessfullossrecoveryepisodes: None,
            tcptimeouts: None,
            udpdatagramscreatedviasoftwaresegmentation: None,
            udpurobytesreceived: None,
            udpuroevents: None,
            urosegmentationfailuresforrawsockets: None,
            urosegmentationsforrawsockets: None,
            urosegmentforwardingfailuresduringsoftwaresegmentation: None,
            urosegmentsforwardedviasoftwaresegmentation: None,
            urosegmentsforwardedviasoftwaresegmentationandchecksum: None,
        }
    }


    /// Sets the value of Bytesoflostretransmitsretransmitted
    pub fn set_bytesoflostretransmitsretransmitted(&mut self, value: u32) {
        self.bytesoflostretransmitsretransmitted = Some(value);
    }

    /// Gets the value of Bytesoflostretransmitsretransmitted
    pub fn get_bytesoflostretransmitsretransmitted(&self) -> Option<&u32> {
        self.bytesoflostretransmitsretransmitted.as_ref()
    }

    /// Sets the value of Deniedconnectorsendrequestsinlowpowermode
    pub fn set_deniedconnectorsendrequestsinlowpowermode(&mut self, value: u32) {
        self.deniedconnectorsendrequestsinlowpowermode = Some(value);
    }

    /// Gets the value of Deniedconnectorsendrequestsinlowpowermode
    pub fn get_deniedconnectorsendrequestsinlowpowermode(&self) -> Option<&u32> {
        self.deniedconnectorsendrequestsinlowpowermode.as_ref()
    }

    /// Sets the value of IPv4NBLsindicatedwithlowresourceflag
    pub fn set_ipv4_nblsindicatedwithlowresourceflag(&mut self, value: u32) {
        self.ipv4_nblsindicatedwithlowresourceflag = Some(value);
    }

    /// Gets the value of IPv4NBLsindicatedwithlowresourceflag
    pub fn get_ipv4_nblsindicatedwithlowresourceflag(&self) -> Option<&u32> {
        self.ipv4_nblsindicatedwithlowresourceflag.as_ref()
    }

    /// Sets the value of IPv4NBLsindicatedwithoutprevalidation
    pub fn set_ipv4_nblsindicatedwithoutprevalidation(&mut self, value: u32) {
        self.ipv4_nblsindicatedwithoutprevalidation = Some(value);
    }

    /// Gets the value of IPv4NBLsindicatedwithoutprevalidation
    pub fn get_ipv4_nblsindicatedwithoutprevalidation(&self) -> Option<&u32> {
        self.ipv4_nblsindicatedwithoutprevalidation.as_ref()
    }

    /// Sets the value of IPv4NBLsPersecindicatedwithlowresourceflag
    pub fn set_ipv4_nbls_persecindicatedwithlowresourceflag(&mut self, value: u32) {
        self.ipv4_nbls_persecindicatedwithlowresourceflag = Some(value);
    }

    /// Gets the value of IPv4NBLsPersecindicatedwithlowresourceflag
    pub fn get_ipv4_nbls_persecindicatedwithlowresourceflag(&self) -> Option<&u32> {
        self.ipv4_nbls_persecindicatedwithlowresourceflag.as_ref()
    }

    /// Sets the value of IPv4NBLsPersecindicatedwithoutprevalidation
    pub fn set_ipv4_nbls_persecindicatedwithoutprevalidation(&mut self, value: u32) {
        self.ipv4_nbls_persecindicatedwithoutprevalidation = Some(value);
    }

    /// Gets the value of IPv4NBLsPersecindicatedwithoutprevalidation
    pub fn get_ipv4_nbls_persecindicatedwithoutprevalidation(&self) -> Option<&u32> {
        self.ipv4_nbls_persecindicatedwithoutprevalidation.as_ref()
    }

    /// Sets the value of IPv4NBLsPersectreatedasnonprevalidated
    pub fn set_ipv4_nbls_persectreatedasnonprevalidated(&mut self, value: u32) {
        self.ipv4_nbls_persectreatedasnonprevalidated = Some(value);
    }

    /// Gets the value of IPv4NBLsPersectreatedasnonprevalidated
    pub fn get_ipv4_nbls_persectreatedasnonprevalidated(&self) -> Option<&u32> {
        self.ipv4_nbls_persectreatedasnonprevalidated.as_ref()
    }

    /// Sets the value of IPv4NBLstreatedasnonprevalidated
    pub fn set_ipv4_nblstreatedasnonprevalidated(&mut self, value: u32) {
        self.ipv4_nblstreatedasnonprevalidated = Some(value);
    }

    /// Gets the value of IPv4NBLstreatedasnonprevalidated
    pub fn get_ipv4_nblstreatedasnonprevalidated(&self) -> Option<&u32> {
        self.ipv4_nblstreatedasnonprevalidated.as_ref()
    }

    /// Sets the value of IPv4outboundNBLsnotprocessedviafastpath
    pub fn set_ipv4outbound_nblsnotprocessedviafastpath(&mut self, value: u32) {
        self.ipv4outbound_nblsnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of IPv4outboundNBLsnotprocessedviafastpath
    pub fn get_ipv4outbound_nblsnotprocessedviafastpath(&self) -> Option<&u32> {
        self.ipv4outbound_nblsnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of IPv4outboundNBLsPersecnotprocessedviafastpath
    pub fn set_ipv4outbound_nbls_persecnotprocessedviafastpath(&mut self, value: u32) {
        self.ipv4outbound_nbls_persecnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of IPv4outboundNBLsPersecnotprocessedviafastpath
    pub fn get_ipv4outbound_nbls_persecnotprocessedviafastpath(&self) -> Option<&u32> {
        self.ipv4outbound_nbls_persecnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of IPv6NBLsindicatedwithlowresourceflag
    pub fn set_ipv6_nblsindicatedwithlowresourceflag(&mut self, value: u32) {
        self.ipv6_nblsindicatedwithlowresourceflag = Some(value);
    }

    /// Gets the value of IPv6NBLsindicatedwithlowresourceflag
    pub fn get_ipv6_nblsindicatedwithlowresourceflag(&self) -> Option<&u32> {
        self.ipv6_nblsindicatedwithlowresourceflag.as_ref()
    }

    /// Sets the value of IPv6NBLsindicatedwithoutprevalidation
    pub fn set_ipv6_nblsindicatedwithoutprevalidation(&mut self, value: u32) {
        self.ipv6_nblsindicatedwithoutprevalidation = Some(value);
    }

    /// Gets the value of IPv6NBLsindicatedwithoutprevalidation
    pub fn get_ipv6_nblsindicatedwithoutprevalidation(&self) -> Option<&u32> {
        self.ipv6_nblsindicatedwithoutprevalidation.as_ref()
    }

    /// Sets the value of IPv6NBLsPersecindicatedwithlowresourceflag
    pub fn set_ipv6_nbls_persecindicatedwithlowresourceflag(&mut self, value: u32) {
        self.ipv6_nbls_persecindicatedwithlowresourceflag = Some(value);
    }

    /// Gets the value of IPv6NBLsPersecindicatedwithlowresourceflag
    pub fn get_ipv6_nbls_persecindicatedwithlowresourceflag(&self) -> Option<&u32> {
        self.ipv6_nbls_persecindicatedwithlowresourceflag.as_ref()
    }

    /// Sets the value of IPv6NBLsPersecindicatedwithoutprevalidation
    pub fn set_ipv6_nbls_persecindicatedwithoutprevalidation(&mut self, value: u32) {
        self.ipv6_nbls_persecindicatedwithoutprevalidation = Some(value);
    }

    /// Gets the value of IPv6NBLsPersecindicatedwithoutprevalidation
    pub fn get_ipv6_nbls_persecindicatedwithoutprevalidation(&self) -> Option<&u32> {
        self.ipv6_nbls_persecindicatedwithoutprevalidation.as_ref()
    }

    /// Sets the value of IPv6NBLsPersectreatedasnonprevalidated
    pub fn set_ipv6_nbls_persectreatedasnonprevalidated(&mut self, value: u32) {
        self.ipv6_nbls_persectreatedasnonprevalidated = Some(value);
    }

    /// Gets the value of IPv6NBLsPersectreatedasnonprevalidated
    pub fn get_ipv6_nbls_persectreatedasnonprevalidated(&self) -> Option<&u32> {
        self.ipv6_nbls_persectreatedasnonprevalidated.as_ref()
    }

    /// Sets the value of IPv6NBLstreatedasnonprevalidated
    pub fn set_ipv6_nblstreatedasnonprevalidated(&mut self, value: u32) {
        self.ipv6_nblstreatedasnonprevalidated = Some(value);
    }

    /// Gets the value of IPv6NBLstreatedasnonprevalidated
    pub fn get_ipv6_nblstreatedasnonprevalidated(&self) -> Option<&u32> {
        self.ipv6_nblstreatedasnonprevalidated.as_ref()
    }

    /// Sets the value of IPv6outboundNBLsnotprocessedviafastpath
    pub fn set_ipv6outbound_nblsnotprocessedviafastpath(&mut self, value: u32) {
        self.ipv6outbound_nblsnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of IPv6outboundNBLsnotprocessedviafastpath
    pub fn get_ipv6outbound_nblsnotprocessedviafastpath(&self) -> Option<&u32> {
        self.ipv6outbound_nblsnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of IPv6outboundNBLsPersecnotprocessedviafastpath
    pub fn set_ipv6outbound_nbls_persecnotprocessedviafastpath(&mut self, value: u32) {
        self.ipv6outbound_nbls_persecnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of IPv6outboundNBLsPersecnotprocessedviafastpath
    pub fn get_ipv6outbound_nbls_persecnotprocessedviafastpath(&self) -> Option<&u32> {
        self.ipv6outbound_nbls_persecnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of NumberofSACKblocksdropped
    pub fn set_numberof_sackblocksdropped(&mut self, value: u32) {
        self.numberof_sackblocksdropped = Some(value);
    }

    /// Gets the value of NumberofSACKblocksdropped
    pub fn get_numberof_sackblocksdropped(&self) -> Option<&u32> {
        self.numberof_sackblocksdropped.as_ref()
    }

    /// Sets the value of NumberofTCPRXfastpathbatchesinspected
    pub fn set_numberof_tcprxfastpathbatchesinspected(&mut self, value: u64) {
        self.numberof_tcprxfastpathbatchesinspected = Some(value);
    }

    /// Gets the value of NumberofTCPRXfastpathbatchesinspected
    pub fn get_numberof_tcprxfastpathbatchesinspected(&self) -> Option<&u64> {
        self.numberof_tcprxfastpathbatchesinspected.as_ref()
    }

    /// Sets the value of NumberofTCPRXfastpathbatchesnotinspected
    pub fn set_numberof_tcprxfastpathbatchesnotinspected(&mut self, value: u64) {
        self.numberof_tcprxfastpathbatchesnotinspected = Some(value);
    }

    /// Gets the value of NumberofTCPRXfastpathbatchesnotinspected
    pub fn get_numberof_tcprxfastpathbatchesnotinspected(&self) -> Option<&u64> {
        self.numberof_tcprxfastpathbatchesnotinspected.as_ref()
    }

    /// Sets the value of NumberofUSOpacketssegmentedandchecksummedinsoftware
    pub fn set_numberof_usopacketssegmentedandchecksummedinsoftware(&mut self, value: u32) {
        self.numberof_usopacketssegmentedandchecksummedinsoftware = Some(value);
    }

    /// Gets the value of NumberofUSOpacketssegmentedandchecksummedinsoftware
    pub fn get_numberof_usopacketssegmentedandchecksummedinsoftware(&self) -> Option<&u32> {
        self.numberof_usopacketssegmentedandchecksummedinsoftware.as_ref()
    }

    /// Sets the value of NumberofUSOpacketssegmentedinsoftware
    pub fn set_numberof_usopacketssegmentedinsoftware(&mut self, value: u32) {
        self.numberof_usopacketssegmentedinsoftware = Some(value);
    }

    /// Gets the value of NumberofUSOpacketssegmentedinsoftware
    pub fn get_numberof_usopacketssegmentedinsoftware(&self) -> Option<&u32> {
        self.numberof_usopacketssegmentedinsoftware.as_ref()
    }

    /// Sets the value of RSCsegmentforwardingfailuresduringsoftwaresegmentation
    pub fn set_rscsegmentforwardingfailuresduringsoftwaresegmentation(&mut self, value: u32) {
        self.rscsegmentforwardingfailuresduringsoftwaresegmentation = Some(value);
    }

    /// Gets the value of RSCsegmentforwardingfailuresduringsoftwaresegmentation
    pub fn get_rscsegmentforwardingfailuresduringsoftwaresegmentation(&self) -> Option<&u32> {
        self.rscsegmentforwardingfailuresduringsoftwaresegmentation.as_ref()
    }

    /// Sets the value of RSCsegmentsforwardedviaLSO
    pub fn set_rscsegmentsforwardedvia_lso(&mut self, value: u32) {
        self.rscsegmentsforwardedvia_lso = Some(value);
    }

    /// Gets the value of RSCsegmentsforwardedviaLSO
    pub fn get_rscsegmentsforwardedvia_lso(&self) -> Option<&u32> {
        self.rscsegmentsforwardedvia_lso.as_ref()
    }

    /// Sets the value of RSCsegmentsforwardedviasoftwaresegmentation
    pub fn set_rscsegmentsforwardedviasoftwaresegmentation(&mut self, value: u32) {
        self.rscsegmentsforwardedviasoftwaresegmentation = Some(value);
    }

    /// Gets the value of RSCsegmentsforwardedviasoftwaresegmentation
    pub fn get_rscsegmentsforwardedviasoftwaresegmentation(&self) -> Option<&u32> {
        self.rscsegmentsforwardedviasoftwaresegmentation.as_ref()
    }

    /// Sets the value of RSCsegmentsforwardedviasoftwaresegmentationandchecksum
    pub fn set_rscsegmentsforwardedviasoftwaresegmentationandchecksum(&mut self, value: u32) {
        self.rscsegmentsforwardedviasoftwaresegmentationandchecksum = Some(value);
    }

    /// Gets the value of RSCsegmentsforwardedviasoftwaresegmentationandchecksum
    pub fn get_rscsegmentsforwardedviasoftwaresegmentationandchecksum(&self) -> Option<&u32> {
        self.rscsegmentsforwardedviasoftwaresegmentationandchecksum.as_ref()
    }

    /// Sets the value of TCPchecksumerrors
    pub fn set_tcpchecksumerrors(&mut self, value: u32) {
        self.tcpchecksumerrors = Some(value);
    }

    /// Gets the value of TCPchecksumerrors
    pub fn get_tcpchecksumerrors(&self) -> Option<&u32> {
        self.tcpchecksumerrors.as_ref()
    }

    /// Sets the value of TCPconnectrequestsfallenoffloopbackfastpath
    pub fn set_tcpconnectrequestsfallenoffloopbackfastpath(&mut self, value: u32) {
        self.tcpconnectrequestsfallenoffloopbackfastpath = Some(value);
    }

    /// Gets the value of TCPconnectrequestsfallenoffloopbackfastpath
    pub fn get_tcpconnectrequestsfallenoffloopbackfastpath(&self) -> Option<&u32> {
        self.tcpconnectrequestsfallenoffloopbackfastpath.as_ref()
    }

    /// Sets the value of TCPconnectrequestsPersecfallenoffloopbackfastpath
    pub fn set_tcpconnectrequests_persecfallenoffloopbackfastpath(&mut self, value: u32) {
        self.tcpconnectrequests_persecfallenoffloopbackfastpath = Some(value);
    }

    /// Gets the value of TCPconnectrequestsPersecfallenoffloopbackfastpath
    pub fn get_tcpconnectrequests_persecfallenoffloopbackfastpath(&self) -> Option<&u32> {
        self.tcpconnectrequests_persecfallenoffloopbackfastpath.as_ref()
    }

    /// Sets the value of TCPinboundsegmentsnotprocessedviafastpath
    pub fn set_tcpinboundsegmentsnotprocessedviafastpath(&mut self, value: u32) {
        self.tcpinboundsegmentsnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of TCPinboundsegmentsnotprocessedviafastpath
    pub fn get_tcpinboundsegmentsnotprocessedviafastpath(&self) -> Option<&u32> {
        self.tcpinboundsegmentsnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of TCPinboundsegmentsPersecnotprocessedviafastpath
    pub fn set_tcpinboundsegments_persecnotprocessedviafastpath(&mut self, value: u32) {
        self.tcpinboundsegments_persecnotprocessedviafastpath = Some(value);
    }

    /// Gets the value of TCPinboundsegmentsPersecnotprocessedviafastpath
    pub fn get_tcpinboundsegments_persecnotprocessedviafastpath(&self) -> Option<&u32> {
        self.tcpinboundsegments_persecnotprocessedviafastpath.as_ref()
    }

    /// Sets the value of TCPlossrecoveryepisodes
    pub fn set_tcplossrecoveryepisodes(&mut self, value: u32) {
        self.tcplossrecoveryepisodes = Some(value);
    }

    /// Gets the value of TCPlossrecoveryepisodes
    pub fn get_tcplossrecoveryepisodes(&self) -> Option<&u32> {
        self.tcplossrecoveryepisodes.as_ref()
    }

    /// Sets the value of TCPRSCbytesreceived
    pub fn set_tcprscbytesreceived(&mut self, value: u32) {
        self.tcprscbytesreceived = Some(value);
    }

    /// Gets the value of TCPRSCbytesreceived
    pub fn get_tcprscbytesreceived(&self) -> Option<&u32> {
        self.tcprscbytesreceived.as_ref()
    }

    /// Sets the value of TCPRSCevents
    pub fn set_tcprscevents(&mut self, value: u32) {
        self.tcprscevents = Some(value);
    }

    /// Gets the value of TCPRSCevents
    pub fn get_tcprscevents(&self) -> Option<&u32> {
        self.tcprscevents.as_ref()
    }

    /// Sets the value of TCPsuccessfullossrecoveryepisodes
    pub fn set_tcpsuccessfullossrecoveryepisodes(&mut self, value: u32) {
        self.tcpsuccessfullossrecoveryepisodes = Some(value);
    }

    /// Gets the value of TCPsuccessfullossrecoveryepisodes
    pub fn get_tcpsuccessfullossrecoveryepisodes(&self) -> Option<&u32> {
        self.tcpsuccessfullossrecoveryepisodes.as_ref()
    }

    /// Sets the value of TCPtimeouts
    pub fn set_tcptimeouts(&mut self, value: u32) {
        self.tcptimeouts = Some(value);
    }

    /// Gets the value of TCPtimeouts
    pub fn get_tcptimeouts(&self) -> Option<&u32> {
        self.tcptimeouts.as_ref()
    }

    /// Sets the value of UDPdatagramscreatedviasoftwaresegmentation
    pub fn set_udpdatagramscreatedviasoftwaresegmentation(&mut self, value: u32) {
        self.udpdatagramscreatedviasoftwaresegmentation = Some(value);
    }

    /// Gets the value of UDPdatagramscreatedviasoftwaresegmentation
    pub fn get_udpdatagramscreatedviasoftwaresegmentation(&self) -> Option<&u32> {
        self.udpdatagramscreatedviasoftwaresegmentation.as_ref()
    }

    /// Sets the value of UDPURObytesreceived
    pub fn set_udpurobytesreceived(&mut self, value: u32) {
        self.udpurobytesreceived = Some(value);
    }

    /// Gets the value of UDPURObytesreceived
    pub fn get_udpurobytesreceived(&self) -> Option<&u32> {
        self.udpurobytesreceived.as_ref()
    }

    /// Sets the value of UDPUROevents
    pub fn set_udpuroevents(&mut self, value: u32) {
        self.udpuroevents = Some(value);
    }

    /// Gets the value of UDPUROevents
    pub fn get_udpuroevents(&self) -> Option<&u32> {
        self.udpuroevents.as_ref()
    }

    /// Sets the value of UROsegmentationfailuresforrawsockets
    pub fn set_urosegmentationfailuresforrawsockets(&mut self, value: u32) {
        self.urosegmentationfailuresforrawsockets = Some(value);
    }

    /// Gets the value of UROsegmentationfailuresforrawsockets
    pub fn get_urosegmentationfailuresforrawsockets(&self) -> Option<&u32> {
        self.urosegmentationfailuresforrawsockets.as_ref()
    }

    /// Sets the value of UROsegmentationsforrawsockets
    pub fn set_urosegmentationsforrawsockets(&mut self, value: u32) {
        self.urosegmentationsforrawsockets = Some(value);
    }

    /// Gets the value of UROsegmentationsforrawsockets
    pub fn get_urosegmentationsforrawsockets(&self) -> Option<&u32> {
        self.urosegmentationsforrawsockets.as_ref()
    }

    /// Sets the value of UROsegmentforwardingfailuresduringsoftwaresegmentation
    pub fn set_urosegmentforwardingfailuresduringsoftwaresegmentation(&mut self, value: u32) {
        self.urosegmentforwardingfailuresduringsoftwaresegmentation = Some(value);
    }

    /// Gets the value of UROsegmentforwardingfailuresduringsoftwaresegmentation
    pub fn get_urosegmentforwardingfailuresduringsoftwaresegmentation(&self) -> Option<&u32> {
        self.urosegmentforwardingfailuresduringsoftwaresegmentation.as_ref()
    }

    /// Sets the value of UROsegmentsforwardedviasoftwaresegmentation
    pub fn set_urosegmentsforwardedviasoftwaresegmentation(&mut self, value: u32) {
        self.urosegmentsforwardedviasoftwaresegmentation = Some(value);
    }

    /// Gets the value of UROsegmentsforwardedviasoftwaresegmentation
    pub fn get_urosegmentsforwardedviasoftwaresegmentation(&self) -> Option<&u32> {
        self.urosegmentsforwardedviasoftwaresegmentation.as_ref()
    }

    /// Sets the value of UROsegmentsforwardedviasoftwaresegmentationandchecksum
    pub fn set_urosegmentsforwardedviasoftwaresegmentationandchecksum(&mut self, value: u32) {
        self.urosegmentsforwardedviasoftwaresegmentationandchecksum = Some(value);
    }

    /// Gets the value of UROsegmentsforwardedviasoftwaresegmentationandchecksum
    pub fn get_urosegmentsforwardedviasoftwaresegmentationandchecksum(&self) -> Option<&u32> {
        self.urosegmentsforwardedviasoftwaresegmentationandchecksum.as_ref()
    }
}


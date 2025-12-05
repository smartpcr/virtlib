// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfNet_Browser struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfNet_Browser {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AnnouncementsDomainPersec")]
    pub announcements_domain_persec: Option<u64>,

/// 
    #[serde(rename = "AnnouncementsServerPersec")]
    pub announcements_server_persec: Option<u64>,

/// 
    #[serde(rename = "AnnouncementsTotalPersec")]
    pub announcements_total_persec: Option<u64>,

/// 
    #[serde(rename = "DuplicateMasterAnnouncements")]
    pub duplicate_master_announcements: Option<u32>,

/// 
    #[serde(rename = "ElectionPacketsPersec")]
    pub election_packets_persec: Option<u32>,

/// 
    #[serde(rename = "EnumerationsDomainPersec")]
    pub enumerations_domain_persec: Option<u32>,

/// 
    #[serde(rename = "EnumerationsOtherPersec")]
    pub enumerations_other_persec: Option<u32>,

/// 
    #[serde(rename = "EnumerationsServerPersec")]
    pub enumerations_server_persec: Option<u32>,

/// 
    #[serde(rename = "EnumerationsTotalPersec")]
    pub enumerations_total_persec: Option<u32>,

/// 
    #[serde(rename = "IllegalDatagramsPersec")]
    pub illegal_datagrams_persec: Option<u64>,

/// 
    #[serde(rename = "MailslotAllocationsFailed")]
    pub mailslot_allocations_failed: Option<u32>,

/// 
    #[serde(rename = "MailslotOpensFailedPersec")]
    pub mailslot_opens_failed_persec: Option<u32>,

/// 
    #[serde(rename = "MailslotReceivesFailed")]
    pub mailslot_receives_failed: Option<u32>,

/// 
    #[serde(rename = "MailslotWritesFailed")]
    pub mailslot_writes_failed: Option<u32>,

/// 
    #[serde(rename = "MailslotWritesPersec")]
    pub mailslot_writes_persec: Option<u32>,

/// 
    #[serde(rename = "MissedMailslotDatagrams")]
    pub missed_mailslot_datagrams: Option<u32>,

/// 
    #[serde(rename = "MissedServerAnnouncements")]
    pub missed_server_announcements: Option<u32>,

/// 
    #[serde(rename = "MissedServerListRequests")]
    pub missed_server_list_requests: Option<u32>,

/// 
    #[serde(rename = "ServerAnnounceAllocationsFailedPersec")]
    pub server_announce_allocations_failed_persec: Option<u32>,

/// 
    #[serde(rename = "ServerListRequestsPersec")]
    pub server_list_requests_persec: Option<u32>,
}

impl Win32_PerfFormattedData_PerfNet_Browser {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            announcements_domain_persec: None,
            announcements_server_persec: None,
            announcements_total_persec: None,
            duplicate_master_announcements: None,
            election_packets_persec: None,
            enumerations_domain_persec: None,
            enumerations_other_persec: None,
            enumerations_server_persec: None,
            enumerations_total_persec: None,
            illegal_datagrams_persec: None,
            mailslot_allocations_failed: None,
            mailslot_opens_failed_persec: None,
            mailslot_receives_failed: None,
            mailslot_writes_failed: None,
            mailslot_writes_persec: None,
            missed_mailslot_datagrams: None,
            missed_server_announcements: None,
            missed_server_list_requests: None,
            server_announce_allocations_failed_persec: None,
            server_list_requests_persec: None,
        }
    }


    /// Sets the value of AnnouncementsDomainPersec
    pub fn set_announcements_domain_persec(&mut self, value: u64) {
        self.announcements_domain_persec = Some(value);
    }

    /// Gets the value of AnnouncementsDomainPersec
    pub fn get_announcements_domain_persec(&self) -> Option<&u64> {
        self.announcements_domain_persec.as_ref()
    }

    /// Sets the value of AnnouncementsServerPersec
    pub fn set_announcements_server_persec(&mut self, value: u64) {
        self.announcements_server_persec = Some(value);
    }

    /// Gets the value of AnnouncementsServerPersec
    pub fn get_announcements_server_persec(&self) -> Option<&u64> {
        self.announcements_server_persec.as_ref()
    }

    /// Sets the value of AnnouncementsTotalPersec
    pub fn set_announcements_total_persec(&mut self, value: u64) {
        self.announcements_total_persec = Some(value);
    }

    /// Gets the value of AnnouncementsTotalPersec
    pub fn get_announcements_total_persec(&self) -> Option<&u64> {
        self.announcements_total_persec.as_ref()
    }

    /// Sets the value of DuplicateMasterAnnouncements
    pub fn set_duplicate_master_announcements(&mut self, value: u32) {
        self.duplicate_master_announcements = Some(value);
    }

    /// Gets the value of DuplicateMasterAnnouncements
    pub fn get_duplicate_master_announcements(&self) -> Option<&u32> {
        self.duplicate_master_announcements.as_ref()
    }

    /// Sets the value of ElectionPacketsPersec
    pub fn set_election_packets_persec(&mut self, value: u32) {
        self.election_packets_persec = Some(value);
    }

    /// Gets the value of ElectionPacketsPersec
    pub fn get_election_packets_persec(&self) -> Option<&u32> {
        self.election_packets_persec.as_ref()
    }

    /// Sets the value of EnumerationsDomainPersec
    pub fn set_enumerations_domain_persec(&mut self, value: u32) {
        self.enumerations_domain_persec = Some(value);
    }

    /// Gets the value of EnumerationsDomainPersec
    pub fn get_enumerations_domain_persec(&self) -> Option<&u32> {
        self.enumerations_domain_persec.as_ref()
    }

    /// Sets the value of EnumerationsOtherPersec
    pub fn set_enumerations_other_persec(&mut self, value: u32) {
        self.enumerations_other_persec = Some(value);
    }

    /// Gets the value of EnumerationsOtherPersec
    pub fn get_enumerations_other_persec(&self) -> Option<&u32> {
        self.enumerations_other_persec.as_ref()
    }

    /// Sets the value of EnumerationsServerPersec
    pub fn set_enumerations_server_persec(&mut self, value: u32) {
        self.enumerations_server_persec = Some(value);
    }

    /// Gets the value of EnumerationsServerPersec
    pub fn get_enumerations_server_persec(&self) -> Option<&u32> {
        self.enumerations_server_persec.as_ref()
    }

    /// Sets the value of EnumerationsTotalPersec
    pub fn set_enumerations_total_persec(&mut self, value: u32) {
        self.enumerations_total_persec = Some(value);
    }

    /// Gets the value of EnumerationsTotalPersec
    pub fn get_enumerations_total_persec(&self) -> Option<&u32> {
        self.enumerations_total_persec.as_ref()
    }

    /// Sets the value of IllegalDatagramsPersec
    pub fn set_illegal_datagrams_persec(&mut self, value: u64) {
        self.illegal_datagrams_persec = Some(value);
    }

    /// Gets the value of IllegalDatagramsPersec
    pub fn get_illegal_datagrams_persec(&self) -> Option<&u64> {
        self.illegal_datagrams_persec.as_ref()
    }

    /// Sets the value of MailslotAllocationsFailed
    pub fn set_mailslot_allocations_failed(&mut self, value: u32) {
        self.mailslot_allocations_failed = Some(value);
    }

    /// Gets the value of MailslotAllocationsFailed
    pub fn get_mailslot_allocations_failed(&self) -> Option<&u32> {
        self.mailslot_allocations_failed.as_ref()
    }

    /// Sets the value of MailslotOpensFailedPersec
    pub fn set_mailslot_opens_failed_persec(&mut self, value: u32) {
        self.mailslot_opens_failed_persec = Some(value);
    }

    /// Gets the value of MailslotOpensFailedPersec
    pub fn get_mailslot_opens_failed_persec(&self) -> Option<&u32> {
        self.mailslot_opens_failed_persec.as_ref()
    }

    /// Sets the value of MailslotReceivesFailed
    pub fn set_mailslot_receives_failed(&mut self, value: u32) {
        self.mailslot_receives_failed = Some(value);
    }

    /// Gets the value of MailslotReceivesFailed
    pub fn get_mailslot_receives_failed(&self) -> Option<&u32> {
        self.mailslot_receives_failed.as_ref()
    }

    /// Sets the value of MailslotWritesFailed
    pub fn set_mailslot_writes_failed(&mut self, value: u32) {
        self.mailslot_writes_failed = Some(value);
    }

    /// Gets the value of MailslotWritesFailed
    pub fn get_mailslot_writes_failed(&self) -> Option<&u32> {
        self.mailslot_writes_failed.as_ref()
    }

    /// Sets the value of MailslotWritesPersec
    pub fn set_mailslot_writes_persec(&mut self, value: u32) {
        self.mailslot_writes_persec = Some(value);
    }

    /// Gets the value of MailslotWritesPersec
    pub fn get_mailslot_writes_persec(&self) -> Option<&u32> {
        self.mailslot_writes_persec.as_ref()
    }

    /// Sets the value of MissedMailslotDatagrams
    pub fn set_missed_mailslot_datagrams(&mut self, value: u32) {
        self.missed_mailslot_datagrams = Some(value);
    }

    /// Gets the value of MissedMailslotDatagrams
    pub fn get_missed_mailslot_datagrams(&self) -> Option<&u32> {
        self.missed_mailslot_datagrams.as_ref()
    }

    /// Sets the value of MissedServerAnnouncements
    pub fn set_missed_server_announcements(&mut self, value: u32) {
        self.missed_server_announcements = Some(value);
    }

    /// Gets the value of MissedServerAnnouncements
    pub fn get_missed_server_announcements(&self) -> Option<&u32> {
        self.missed_server_announcements.as_ref()
    }

    /// Sets the value of MissedServerListRequests
    pub fn set_missed_server_list_requests(&mut self, value: u32) {
        self.missed_server_list_requests = Some(value);
    }

    /// Gets the value of MissedServerListRequests
    pub fn get_missed_server_list_requests(&self) -> Option<&u32> {
        self.missed_server_list_requests.as_ref()
    }

    /// Sets the value of ServerAnnounceAllocationsFailedPersec
    pub fn set_server_announce_allocations_failed_persec(&mut self, value: u32) {
        self.server_announce_allocations_failed_persec = Some(value);
    }

    /// Gets the value of ServerAnnounceAllocationsFailedPersec
    pub fn get_server_announce_allocations_failed_persec(&self) -> Option<&u32> {
        self.server_announce_allocations_failed_persec.as_ref()
    }

    /// Sets the value of ServerListRequestsPersec
    pub fn set_server_list_requests_persec(&mut self, value: u32) {
        self.server_list_requests_persec = Some(value);
    }

    /// Gets the value of ServerListRequestsPersec
    pub fn get_server_list_requests_persec(&self) -> Option<&u32> {
        self.server_list_requests_persec.as_ref()
    }
}


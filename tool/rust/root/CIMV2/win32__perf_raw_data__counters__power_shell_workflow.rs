// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_PowerShellWorkflow struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_PowerShellWorkflow {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ActivityHostManagerhostprocessespoolsize")]
    pub activity_host_managerhostprocessespoolsize: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberofbusyhostprocesses")]
    pub activity_host_manager_numberofbusyhostprocesses: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberofcreatedhostprocesses")]
    pub activity_host_manager_numberofcreatedhostprocesses: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberofdisposedhostprocesses")]
    pub activity_host_manager_numberofdisposedhostprocesses: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberoffailedrequestsinqueue")]
    pub activity_host_manager_numberoffailedrequestsinqueue: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberoffailedrequestsPersec")]
    pub activity_host_manager_numberoffailedrequests_persec: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberofincomingrequestsPersec")]
    pub activity_host_manager_numberofincomingrequests_persec: Option<u32>,

/// 
    #[serde(rename = "ActivityHostManagerNumberofpendingrequestsinqueue")]
    pub activity_host_manager_numberofpendingrequestsinqueue: Option<u32>,

/// 
    #[serde(rename = "Numberoffailedworkflowjobs")]
    pub numberoffailedworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberoffailedworkflowjobsPersec")]
    pub numberoffailedworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofresumedworkflowjobs")]
    pub numberofresumedworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofresumedworkflowjobsPersec")]
    pub numberofresumedworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofrunningworkflowjobs")]
    pub numberofrunningworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofrunningworkflowjobsPersec")]
    pub numberofrunningworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofstoppedworkflowjobs")]
    pub numberofstoppedworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofstoppedworkflowjobsPersec")]
    pub numberofstoppedworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofsucceededworkflowjobs")]
    pub numberofsucceededworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofsucceededworkflowjobsPersec")]
    pub numberofsucceededworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofsuspendedworkflowjobs")]
    pub numberofsuspendedworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofsuspendedworkflowjobsPersec")]
    pub numberofsuspendedworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofterminatedworkflowjobs")]
    pub numberofterminatedworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "NumberofterminatedworkflowjobsPersec")]
    pub numberofterminatedworkflowjobs_persec: Option<u32>,

/// 
    #[serde(rename = "Numberofwaitingworkflowjobs")]
    pub numberofwaitingworkflowjobs: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofconnectionsclosedreopened")]
    pub power_shell_remoting_numberofconnectionsclosedreopened: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofcreatedconnections")]
    pub power_shell_remoting_numberofcreatedconnections: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofdisposedconnections")]
    pub power_shell_remoting_numberofdisposedconnections: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofforcedtowaitrequestsinqueue")]
    pub power_shell_remoting_numberofforcedtowaitrequestsinqueue: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofpendingrequestsinqueue")]
    pub power_shell_remoting_numberofpendingrequestsinqueue: Option<u32>,

/// 
    #[serde(rename = "PowerShellRemotingNumberofrequestsbeingserviced")]
    pub power_shell_remoting_numberofrequestsbeingserviced: Option<u32>,
}

impl Win32_PerfRawData_Counters_PowerShellWorkflow {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            activity_host_managerhostprocessespoolsize: None,
            activity_host_manager_numberofbusyhostprocesses: None,
            activity_host_manager_numberofcreatedhostprocesses: None,
            activity_host_manager_numberofdisposedhostprocesses: None,
            activity_host_manager_numberoffailedrequestsinqueue: None,
            activity_host_manager_numberoffailedrequests_persec: None,
            activity_host_manager_numberofincomingrequests_persec: None,
            activity_host_manager_numberofpendingrequestsinqueue: None,
            numberoffailedworkflowjobs: None,
            numberoffailedworkflowjobs_persec: None,
            numberofresumedworkflowjobs: None,
            numberofresumedworkflowjobs_persec: None,
            numberofrunningworkflowjobs: None,
            numberofrunningworkflowjobs_persec: None,
            numberofstoppedworkflowjobs: None,
            numberofstoppedworkflowjobs_persec: None,
            numberofsucceededworkflowjobs: None,
            numberofsucceededworkflowjobs_persec: None,
            numberofsuspendedworkflowjobs: None,
            numberofsuspendedworkflowjobs_persec: None,
            numberofterminatedworkflowjobs: None,
            numberofterminatedworkflowjobs_persec: None,
            numberofwaitingworkflowjobs: None,
            power_shell_remoting_numberofconnectionsclosedreopened: None,
            power_shell_remoting_numberofcreatedconnections: None,
            power_shell_remoting_numberofdisposedconnections: None,
            power_shell_remoting_numberofforcedtowaitrequestsinqueue: None,
            power_shell_remoting_numberofpendingrequestsinqueue: None,
            power_shell_remoting_numberofrequestsbeingserviced: None,
        }
    }


    /// Sets the value of ActivityHostManagerhostprocessespoolsize
    pub fn set_activity_host_managerhostprocessespoolsize(&mut self, value: u32) {
        self.activity_host_managerhostprocessespoolsize = Some(value);
    }

    /// Gets the value of ActivityHostManagerhostprocessespoolsize
    pub fn get_activity_host_managerhostprocessespoolsize(&self) -> Option<&u32> {
        self.activity_host_managerhostprocessespoolsize.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberofbusyhostprocesses
    pub fn set_activity_host_manager_numberofbusyhostprocesses(&mut self, value: u32) {
        self.activity_host_manager_numberofbusyhostprocesses = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberofbusyhostprocesses
    pub fn get_activity_host_manager_numberofbusyhostprocesses(&self) -> Option<&u32> {
        self.activity_host_manager_numberofbusyhostprocesses.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberofcreatedhostprocesses
    pub fn set_activity_host_manager_numberofcreatedhostprocesses(&mut self, value: u32) {
        self.activity_host_manager_numberofcreatedhostprocesses = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberofcreatedhostprocesses
    pub fn get_activity_host_manager_numberofcreatedhostprocesses(&self) -> Option<&u32> {
        self.activity_host_manager_numberofcreatedhostprocesses.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberofdisposedhostprocesses
    pub fn set_activity_host_manager_numberofdisposedhostprocesses(&mut self, value: u32) {
        self.activity_host_manager_numberofdisposedhostprocesses = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberofdisposedhostprocesses
    pub fn get_activity_host_manager_numberofdisposedhostprocesses(&self) -> Option<&u32> {
        self.activity_host_manager_numberofdisposedhostprocesses.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberoffailedrequestsinqueue
    pub fn set_activity_host_manager_numberoffailedrequestsinqueue(&mut self, value: u32) {
        self.activity_host_manager_numberoffailedrequestsinqueue = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberoffailedrequestsinqueue
    pub fn get_activity_host_manager_numberoffailedrequestsinqueue(&self) -> Option<&u32> {
        self.activity_host_manager_numberoffailedrequestsinqueue.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberoffailedrequestsPersec
    pub fn set_activity_host_manager_numberoffailedrequests_persec(&mut self, value: u32) {
        self.activity_host_manager_numberoffailedrequests_persec = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberoffailedrequestsPersec
    pub fn get_activity_host_manager_numberoffailedrequests_persec(&self) -> Option<&u32> {
        self.activity_host_manager_numberoffailedrequests_persec.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberofincomingrequestsPersec
    pub fn set_activity_host_manager_numberofincomingrequests_persec(&mut self, value: u32) {
        self.activity_host_manager_numberofincomingrequests_persec = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberofincomingrequestsPersec
    pub fn get_activity_host_manager_numberofincomingrequests_persec(&self) -> Option<&u32> {
        self.activity_host_manager_numberofincomingrequests_persec.as_ref()
    }

    /// Sets the value of ActivityHostManagerNumberofpendingrequestsinqueue
    pub fn set_activity_host_manager_numberofpendingrequestsinqueue(&mut self, value: u32) {
        self.activity_host_manager_numberofpendingrequestsinqueue = Some(value);
    }

    /// Gets the value of ActivityHostManagerNumberofpendingrequestsinqueue
    pub fn get_activity_host_manager_numberofpendingrequestsinqueue(&self) -> Option<&u32> {
        self.activity_host_manager_numberofpendingrequestsinqueue.as_ref()
    }

    /// Sets the value of Numberoffailedworkflowjobs
    pub fn set_numberoffailedworkflowjobs(&mut self, value: u32) {
        self.numberoffailedworkflowjobs = Some(value);
    }

    /// Gets the value of Numberoffailedworkflowjobs
    pub fn get_numberoffailedworkflowjobs(&self) -> Option<&u32> {
        self.numberoffailedworkflowjobs.as_ref()
    }

    /// Sets the value of NumberoffailedworkflowjobsPersec
    pub fn set_numberoffailedworkflowjobs_persec(&mut self, value: u32) {
        self.numberoffailedworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberoffailedworkflowjobsPersec
    pub fn get_numberoffailedworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberoffailedworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofresumedworkflowjobs
    pub fn set_numberofresumedworkflowjobs(&mut self, value: u32) {
        self.numberofresumedworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofresumedworkflowjobs
    pub fn get_numberofresumedworkflowjobs(&self) -> Option<&u32> {
        self.numberofresumedworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofresumedworkflowjobsPersec
    pub fn set_numberofresumedworkflowjobs_persec(&mut self, value: u32) {
        self.numberofresumedworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofresumedworkflowjobsPersec
    pub fn get_numberofresumedworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofresumedworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofrunningworkflowjobs
    pub fn set_numberofrunningworkflowjobs(&mut self, value: u32) {
        self.numberofrunningworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofrunningworkflowjobs
    pub fn get_numberofrunningworkflowjobs(&self) -> Option<&u32> {
        self.numberofrunningworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofrunningworkflowjobsPersec
    pub fn set_numberofrunningworkflowjobs_persec(&mut self, value: u32) {
        self.numberofrunningworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofrunningworkflowjobsPersec
    pub fn get_numberofrunningworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofrunningworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofstoppedworkflowjobs
    pub fn set_numberofstoppedworkflowjobs(&mut self, value: u32) {
        self.numberofstoppedworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofstoppedworkflowjobs
    pub fn get_numberofstoppedworkflowjobs(&self) -> Option<&u32> {
        self.numberofstoppedworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofstoppedworkflowjobsPersec
    pub fn set_numberofstoppedworkflowjobs_persec(&mut self, value: u32) {
        self.numberofstoppedworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofstoppedworkflowjobsPersec
    pub fn get_numberofstoppedworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofstoppedworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofsucceededworkflowjobs
    pub fn set_numberofsucceededworkflowjobs(&mut self, value: u32) {
        self.numberofsucceededworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofsucceededworkflowjobs
    pub fn get_numberofsucceededworkflowjobs(&self) -> Option<&u32> {
        self.numberofsucceededworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofsucceededworkflowjobsPersec
    pub fn set_numberofsucceededworkflowjobs_persec(&mut self, value: u32) {
        self.numberofsucceededworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofsucceededworkflowjobsPersec
    pub fn get_numberofsucceededworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofsucceededworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofsuspendedworkflowjobs
    pub fn set_numberofsuspendedworkflowjobs(&mut self, value: u32) {
        self.numberofsuspendedworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofsuspendedworkflowjobs
    pub fn get_numberofsuspendedworkflowjobs(&self) -> Option<&u32> {
        self.numberofsuspendedworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofsuspendedworkflowjobsPersec
    pub fn set_numberofsuspendedworkflowjobs_persec(&mut self, value: u32) {
        self.numberofsuspendedworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofsuspendedworkflowjobsPersec
    pub fn get_numberofsuspendedworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofsuspendedworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofterminatedworkflowjobs
    pub fn set_numberofterminatedworkflowjobs(&mut self, value: u32) {
        self.numberofterminatedworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofterminatedworkflowjobs
    pub fn get_numberofterminatedworkflowjobs(&self) -> Option<&u32> {
        self.numberofterminatedworkflowjobs.as_ref()
    }

    /// Sets the value of NumberofterminatedworkflowjobsPersec
    pub fn set_numberofterminatedworkflowjobs_persec(&mut self, value: u32) {
        self.numberofterminatedworkflowjobs_persec = Some(value);
    }

    /// Gets the value of NumberofterminatedworkflowjobsPersec
    pub fn get_numberofterminatedworkflowjobs_persec(&self) -> Option<&u32> {
        self.numberofterminatedworkflowjobs_persec.as_ref()
    }

    /// Sets the value of Numberofwaitingworkflowjobs
    pub fn set_numberofwaitingworkflowjobs(&mut self, value: u32) {
        self.numberofwaitingworkflowjobs = Some(value);
    }

    /// Gets the value of Numberofwaitingworkflowjobs
    pub fn get_numberofwaitingworkflowjobs(&self) -> Option<&u32> {
        self.numberofwaitingworkflowjobs.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofconnectionsclosedreopened
    pub fn set_power_shell_remoting_numberofconnectionsclosedreopened(&mut self, value: u32) {
        self.power_shell_remoting_numberofconnectionsclosedreopened = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofconnectionsclosedreopened
    pub fn get_power_shell_remoting_numberofconnectionsclosedreopened(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofconnectionsclosedreopened.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofcreatedconnections
    pub fn set_power_shell_remoting_numberofcreatedconnections(&mut self, value: u32) {
        self.power_shell_remoting_numberofcreatedconnections = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofcreatedconnections
    pub fn get_power_shell_remoting_numberofcreatedconnections(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofcreatedconnections.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofdisposedconnections
    pub fn set_power_shell_remoting_numberofdisposedconnections(&mut self, value: u32) {
        self.power_shell_remoting_numberofdisposedconnections = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofdisposedconnections
    pub fn get_power_shell_remoting_numberofdisposedconnections(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofdisposedconnections.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofforcedtowaitrequestsinqueue
    pub fn set_power_shell_remoting_numberofforcedtowaitrequestsinqueue(&mut self, value: u32) {
        self.power_shell_remoting_numberofforcedtowaitrequestsinqueue = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofforcedtowaitrequestsinqueue
    pub fn get_power_shell_remoting_numberofforcedtowaitrequestsinqueue(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofforcedtowaitrequestsinqueue.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofpendingrequestsinqueue
    pub fn set_power_shell_remoting_numberofpendingrequestsinqueue(&mut self, value: u32) {
        self.power_shell_remoting_numberofpendingrequestsinqueue = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofpendingrequestsinqueue
    pub fn get_power_shell_remoting_numberofpendingrequestsinqueue(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofpendingrequestsinqueue.as_ref()
    }

    /// Sets the value of PowerShellRemotingNumberofrequestsbeingserviced
    pub fn set_power_shell_remoting_numberofrequestsbeingserviced(&mut self, value: u32) {
        self.power_shell_remoting_numberofrequestsbeingserviced = Some(value);
    }

    /// Gets the value of PowerShellRemotingNumberofrequestsbeingserviced
    pub fn get_power_shell_remoting_numberofrequestsbeingserviced(&self) -> Option<&u32> {
        self.power_shell_remoting_numberofrequestsbeingserviced.as_ref()
    }
}


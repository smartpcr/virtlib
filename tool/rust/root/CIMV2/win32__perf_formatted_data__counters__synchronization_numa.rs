// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_SynchronizationNuma struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_SynchronizationNuma {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ExecResourceAcquiresAcqExclLitePersec")]
    pub exec_resource_acquires_acq_excl_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAcquiresAcqShrdLitePersec")]
    pub exec_resource_acquires_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAcquiresAcqShrdStarveExclPersec")]
    pub exec_resource_acquires_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAcquiresAcqShrdWaitForExclPersec")]
    pub exec_resource_acquires_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAttemptsAcqExclLitePersec")]
    pub exec_resource_attempts_acq_excl_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAttemptsAcqShrdLitePersec")]
    pub exec_resource_attempts_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAttemptsAcqShrdStarveExclPersec")]
    pub exec_resource_attempts_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceAttemptsAcqShrdWaitForExclPersec")]
    pub exec_resource_attempts_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceBoostExclOwnerPersec")]
    pub exec_resource_boost_excl_owner_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceBoostSharedOwnersPersec")]
    pub exec_resource_boost_shared_owners_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceContentionAcqExclLitePersec")]
    pub exec_resource_contention_acq_excl_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceContentionAcqShrdLitePersec")]
    pub exec_resource_contention_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceContentionAcqShrdStarveExclPersec")]
    pub exec_resource_contention_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceContentionAcqShrdWaitForExclPersec")]
    pub exec_resource_contention_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourcenoWaitsAcqExclLitePersec")]
    pub exec_resourceno_waits_acq_excl_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourcenoWaitsAcqShrdLitePersec")]
    pub exec_resourceno_waits_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourcenoWaitsAcqShrdStarveExclPersec")]
    pub exec_resourceno_waits_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourcenoWaitsAcqShrdWaitForExclPersec")]
    pub exec_resourceno_waits_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveExclAcquiresAcqExclLitePersec")]
    pub exec_resource_recursive_excl_acquires_acq_excl_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveExclAcquiresAcqShrdLitePersec")]
    pub exec_resource_recursive_excl_acquires_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveExclAcquiresAcqShrdStarveExclPersec")]
    pub exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveExclAcquiresAcqShrdWaitForExclPersec")]
    pub exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveShAcquiresAcqShrdLitePersec")]
    pub exec_resource_recursive_sh_acquires_acq_shrd_lite_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveShAcquiresAcqShrdStarveExclPersec")]
    pub exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceRecursiveShAcquiresAcqShrdWaitForExclPersec")]
    pub exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceSetOwnerPointerExclusivePersec")]
    pub exec_resource_set_owner_pointer_exclusive_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceSetOwnerPointerSharedExistingOwnerPersec")]
    pub exec_resource_set_owner_pointer_shared_existing_owner_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceSetOwnerPointerSharedNewOwnerPersec")]
    pub exec_resource_set_owner_pointer_shared_new_owner_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalAcquiresPersec")]
    pub exec_resource_total_acquires_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalContentionsPersec")]
    pub exec_resource_total_contentions_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalConvExclusiveToSharedPersec")]
    pub exec_resource_total_conv_exclusive_to_shared_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalDeletePersec")]
    pub exec_resource_total_delete_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalExclusiveReleasesPersec")]
    pub exec_resource_total_exclusive_releases_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalInitializePersec")]
    pub exec_resource_total_initialize_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalReInitializePersec")]
    pub exec_resource_total_re_initialize_persec: Option<u32>,

/// 
    #[serde(rename = "ExecResourceTotalSharedReleasesPersec")]
    pub exec_resource_total_shared_releases_persec: Option<u32>,

/// 
    #[serde(rename = "IPISendBroadcastRequestsPersec")]
    pub ipisend_broadcast_requests_persec: Option<u32>,

/// 
    #[serde(rename = "IPISendRoutineRequestsPersec")]
    pub ipisend_routine_requests_persec: Option<u32>,

/// 
    #[serde(rename = "IPISendSoftwareInterruptsPersec")]
    pub ipisend_software_interrupts_persec: Option<u32>,

/// 
    #[serde(rename = "SpinlockAcquiresPersec")]
    pub spinlock_acquires_persec: Option<u32>,

/// 
    #[serde(rename = "SpinlockContentionsPersec")]
    pub spinlock_contentions_persec: Option<u32>,

/// 
    #[serde(rename = "SpinlockSpinsPersec")]
    pub spinlock_spins_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_SynchronizationNuma {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            exec_resource_acquires_acq_excl_lite_persec: None,
            exec_resource_acquires_acq_shrd_lite_persec: None,
            exec_resource_acquires_acq_shrd_starve_excl_persec: None,
            exec_resource_acquires_acq_shrd_wait_for_excl_persec: None,
            exec_resource_attempts_acq_excl_lite_persec: None,
            exec_resource_attempts_acq_shrd_lite_persec: None,
            exec_resource_attempts_acq_shrd_starve_excl_persec: None,
            exec_resource_attempts_acq_shrd_wait_for_excl_persec: None,
            exec_resource_boost_excl_owner_persec: None,
            exec_resource_boost_shared_owners_persec: None,
            exec_resource_contention_acq_excl_lite_persec: None,
            exec_resource_contention_acq_shrd_lite_persec: None,
            exec_resource_contention_acq_shrd_starve_excl_persec: None,
            exec_resource_contention_acq_shrd_wait_for_excl_persec: None,
            exec_resourceno_waits_acq_excl_lite_persec: None,
            exec_resourceno_waits_acq_shrd_lite_persec: None,
            exec_resourceno_waits_acq_shrd_starve_excl_persec: None,
            exec_resourceno_waits_acq_shrd_wait_for_excl_persec: None,
            exec_resource_recursive_excl_acquires_acq_excl_lite_persec: None,
            exec_resource_recursive_excl_acquires_acq_shrd_lite_persec: None,
            exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec: None,
            exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec: None,
            exec_resource_recursive_sh_acquires_acq_shrd_lite_persec: None,
            exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec: None,
            exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec: None,
            exec_resource_set_owner_pointer_exclusive_persec: None,
            exec_resource_set_owner_pointer_shared_existing_owner_persec: None,
            exec_resource_set_owner_pointer_shared_new_owner_persec: None,
            exec_resource_total_acquires_persec: None,
            exec_resource_total_contentions_persec: None,
            exec_resource_total_conv_exclusive_to_shared_persec: None,
            exec_resource_total_delete_persec: None,
            exec_resource_total_exclusive_releases_persec: None,
            exec_resource_total_initialize_persec: None,
            exec_resource_total_re_initialize_persec: None,
            exec_resource_total_shared_releases_persec: None,
            ipisend_broadcast_requests_persec: None,
            ipisend_routine_requests_persec: None,
            ipisend_software_interrupts_persec: None,
            spinlock_acquires_persec: None,
            spinlock_contentions_persec: None,
            spinlock_spins_persec: None,
        }
    }


    /// Sets the value of ExecResourceAcquiresAcqExclLitePersec
    pub fn set_exec_resource_acquires_acq_excl_lite_persec(&mut self, value: u32) {
        self.exec_resource_acquires_acq_excl_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceAcquiresAcqExclLitePersec
    pub fn get_exec_resource_acquires_acq_excl_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_acquires_acq_excl_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceAcquiresAcqShrdLitePersec
    pub fn set_exec_resource_acquires_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resource_acquires_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceAcquiresAcqShrdLitePersec
    pub fn get_exec_resource_acquires_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_acquires_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceAcquiresAcqShrdStarveExclPersec
    pub fn set_exec_resource_acquires_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resource_acquires_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceAcquiresAcqShrdStarveExclPersec
    pub fn get_exec_resource_acquires_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_acquires_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceAcquiresAcqShrdWaitForExclPersec
    pub fn set_exec_resource_acquires_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resource_acquires_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceAcquiresAcqShrdWaitForExclPersec
    pub fn get_exec_resource_acquires_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_acquires_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceAttemptsAcqExclLitePersec
    pub fn set_exec_resource_attempts_acq_excl_lite_persec(&mut self, value: u32) {
        self.exec_resource_attempts_acq_excl_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceAttemptsAcqExclLitePersec
    pub fn get_exec_resource_attempts_acq_excl_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_attempts_acq_excl_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceAttemptsAcqShrdLitePersec
    pub fn set_exec_resource_attempts_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resource_attempts_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceAttemptsAcqShrdLitePersec
    pub fn get_exec_resource_attempts_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_attempts_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceAttemptsAcqShrdStarveExclPersec
    pub fn set_exec_resource_attempts_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resource_attempts_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceAttemptsAcqShrdStarveExclPersec
    pub fn get_exec_resource_attempts_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_attempts_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceAttemptsAcqShrdWaitForExclPersec
    pub fn set_exec_resource_attempts_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resource_attempts_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceAttemptsAcqShrdWaitForExclPersec
    pub fn get_exec_resource_attempts_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_attempts_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceBoostExclOwnerPersec
    pub fn set_exec_resource_boost_excl_owner_persec(&mut self, value: u32) {
        self.exec_resource_boost_excl_owner_persec = Some(value);
    }

    /// Gets the value of ExecResourceBoostExclOwnerPersec
    pub fn get_exec_resource_boost_excl_owner_persec(&self) -> Option<&u32> {
        self.exec_resource_boost_excl_owner_persec.as_ref()
    }

    /// Sets the value of ExecResourceBoostSharedOwnersPersec
    pub fn set_exec_resource_boost_shared_owners_persec(&mut self, value: u32) {
        self.exec_resource_boost_shared_owners_persec = Some(value);
    }

    /// Gets the value of ExecResourceBoostSharedOwnersPersec
    pub fn get_exec_resource_boost_shared_owners_persec(&self) -> Option<&u32> {
        self.exec_resource_boost_shared_owners_persec.as_ref()
    }

    /// Sets the value of ExecResourceContentionAcqExclLitePersec
    pub fn set_exec_resource_contention_acq_excl_lite_persec(&mut self, value: u32) {
        self.exec_resource_contention_acq_excl_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceContentionAcqExclLitePersec
    pub fn get_exec_resource_contention_acq_excl_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_contention_acq_excl_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceContentionAcqShrdLitePersec
    pub fn set_exec_resource_contention_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resource_contention_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceContentionAcqShrdLitePersec
    pub fn get_exec_resource_contention_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_contention_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceContentionAcqShrdStarveExclPersec
    pub fn set_exec_resource_contention_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resource_contention_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceContentionAcqShrdStarveExclPersec
    pub fn get_exec_resource_contention_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_contention_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceContentionAcqShrdWaitForExclPersec
    pub fn set_exec_resource_contention_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resource_contention_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceContentionAcqShrdWaitForExclPersec
    pub fn get_exec_resource_contention_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_contention_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourcenoWaitsAcqExclLitePersec
    pub fn set_exec_resourceno_waits_acq_excl_lite_persec(&mut self, value: u32) {
        self.exec_resourceno_waits_acq_excl_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourcenoWaitsAcqExclLitePersec
    pub fn get_exec_resourceno_waits_acq_excl_lite_persec(&self) -> Option<&u32> {
        self.exec_resourceno_waits_acq_excl_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourcenoWaitsAcqShrdLitePersec
    pub fn set_exec_resourceno_waits_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resourceno_waits_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourcenoWaitsAcqShrdLitePersec
    pub fn get_exec_resourceno_waits_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resourceno_waits_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourcenoWaitsAcqShrdStarveExclPersec
    pub fn set_exec_resourceno_waits_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resourceno_waits_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourcenoWaitsAcqShrdStarveExclPersec
    pub fn get_exec_resourceno_waits_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resourceno_waits_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourcenoWaitsAcqShrdWaitForExclPersec
    pub fn set_exec_resourceno_waits_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resourceno_waits_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourcenoWaitsAcqShrdWaitForExclPersec
    pub fn get_exec_resourceno_waits_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resourceno_waits_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveExclAcquiresAcqExclLitePersec
    pub fn set_exec_resource_recursive_excl_acquires_acq_excl_lite_persec(&mut self, value: u32) {
        self.exec_resource_recursive_excl_acquires_acq_excl_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveExclAcquiresAcqExclLitePersec
    pub fn get_exec_resource_recursive_excl_acquires_acq_excl_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_excl_acquires_acq_excl_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveExclAcquiresAcqShrdLitePersec
    pub fn set_exec_resource_recursive_excl_acquires_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resource_recursive_excl_acquires_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveExclAcquiresAcqShrdLitePersec
    pub fn get_exec_resource_recursive_excl_acquires_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_excl_acquires_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveExclAcquiresAcqShrdStarveExclPersec
    pub fn set_exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveExclAcquiresAcqShrdStarveExclPersec
    pub fn get_exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_excl_acquires_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveExclAcquiresAcqShrdWaitForExclPersec
    pub fn set_exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveExclAcquiresAcqShrdWaitForExclPersec
    pub fn get_exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_excl_acquires_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveShAcquiresAcqShrdLitePersec
    pub fn set_exec_resource_recursive_sh_acquires_acq_shrd_lite_persec(&mut self, value: u32) {
        self.exec_resource_recursive_sh_acquires_acq_shrd_lite_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveShAcquiresAcqShrdLitePersec
    pub fn get_exec_resource_recursive_sh_acquires_acq_shrd_lite_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_sh_acquires_acq_shrd_lite_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveShAcquiresAcqShrdStarveExclPersec
    pub fn set_exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec(&mut self, value: u32) {
        self.exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveShAcquiresAcqShrdStarveExclPersec
    pub fn get_exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_sh_acquires_acq_shrd_starve_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceRecursiveShAcquiresAcqShrdWaitForExclPersec
    pub fn set_exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec(&mut self, value: u32) {
        self.exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec = Some(value);
    }

    /// Gets the value of ExecResourceRecursiveShAcquiresAcqShrdWaitForExclPersec
    pub fn get_exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec(&self) -> Option<&u32> {
        self.exec_resource_recursive_sh_acquires_acq_shrd_wait_for_excl_persec.as_ref()
    }

    /// Sets the value of ExecResourceSetOwnerPointerExclusivePersec
    pub fn set_exec_resource_set_owner_pointer_exclusive_persec(&mut self, value: u32) {
        self.exec_resource_set_owner_pointer_exclusive_persec = Some(value);
    }

    /// Gets the value of ExecResourceSetOwnerPointerExclusivePersec
    pub fn get_exec_resource_set_owner_pointer_exclusive_persec(&self) -> Option<&u32> {
        self.exec_resource_set_owner_pointer_exclusive_persec.as_ref()
    }

    /// Sets the value of ExecResourceSetOwnerPointerSharedExistingOwnerPersec
    pub fn set_exec_resource_set_owner_pointer_shared_existing_owner_persec(&mut self, value: u32) {
        self.exec_resource_set_owner_pointer_shared_existing_owner_persec = Some(value);
    }

    /// Gets the value of ExecResourceSetOwnerPointerSharedExistingOwnerPersec
    pub fn get_exec_resource_set_owner_pointer_shared_existing_owner_persec(&self) -> Option<&u32> {
        self.exec_resource_set_owner_pointer_shared_existing_owner_persec.as_ref()
    }

    /// Sets the value of ExecResourceSetOwnerPointerSharedNewOwnerPersec
    pub fn set_exec_resource_set_owner_pointer_shared_new_owner_persec(&mut self, value: u32) {
        self.exec_resource_set_owner_pointer_shared_new_owner_persec = Some(value);
    }

    /// Gets the value of ExecResourceSetOwnerPointerSharedNewOwnerPersec
    pub fn get_exec_resource_set_owner_pointer_shared_new_owner_persec(&self) -> Option<&u32> {
        self.exec_resource_set_owner_pointer_shared_new_owner_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalAcquiresPersec
    pub fn set_exec_resource_total_acquires_persec(&mut self, value: u32) {
        self.exec_resource_total_acquires_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalAcquiresPersec
    pub fn get_exec_resource_total_acquires_persec(&self) -> Option<&u32> {
        self.exec_resource_total_acquires_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalContentionsPersec
    pub fn set_exec_resource_total_contentions_persec(&mut self, value: u32) {
        self.exec_resource_total_contentions_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalContentionsPersec
    pub fn get_exec_resource_total_contentions_persec(&self) -> Option<&u32> {
        self.exec_resource_total_contentions_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalConvExclusiveToSharedPersec
    pub fn set_exec_resource_total_conv_exclusive_to_shared_persec(&mut self, value: u32) {
        self.exec_resource_total_conv_exclusive_to_shared_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalConvExclusiveToSharedPersec
    pub fn get_exec_resource_total_conv_exclusive_to_shared_persec(&self) -> Option<&u32> {
        self.exec_resource_total_conv_exclusive_to_shared_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalDeletePersec
    pub fn set_exec_resource_total_delete_persec(&mut self, value: u32) {
        self.exec_resource_total_delete_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalDeletePersec
    pub fn get_exec_resource_total_delete_persec(&self) -> Option<&u32> {
        self.exec_resource_total_delete_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalExclusiveReleasesPersec
    pub fn set_exec_resource_total_exclusive_releases_persec(&mut self, value: u32) {
        self.exec_resource_total_exclusive_releases_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalExclusiveReleasesPersec
    pub fn get_exec_resource_total_exclusive_releases_persec(&self) -> Option<&u32> {
        self.exec_resource_total_exclusive_releases_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalInitializePersec
    pub fn set_exec_resource_total_initialize_persec(&mut self, value: u32) {
        self.exec_resource_total_initialize_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalInitializePersec
    pub fn get_exec_resource_total_initialize_persec(&self) -> Option<&u32> {
        self.exec_resource_total_initialize_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalReInitializePersec
    pub fn set_exec_resource_total_re_initialize_persec(&mut self, value: u32) {
        self.exec_resource_total_re_initialize_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalReInitializePersec
    pub fn get_exec_resource_total_re_initialize_persec(&self) -> Option<&u32> {
        self.exec_resource_total_re_initialize_persec.as_ref()
    }

    /// Sets the value of ExecResourceTotalSharedReleasesPersec
    pub fn set_exec_resource_total_shared_releases_persec(&mut self, value: u32) {
        self.exec_resource_total_shared_releases_persec = Some(value);
    }

    /// Gets the value of ExecResourceTotalSharedReleasesPersec
    pub fn get_exec_resource_total_shared_releases_persec(&self) -> Option<&u32> {
        self.exec_resource_total_shared_releases_persec.as_ref()
    }

    /// Sets the value of IPISendBroadcastRequestsPersec
    pub fn set_ipisend_broadcast_requests_persec(&mut self, value: u32) {
        self.ipisend_broadcast_requests_persec = Some(value);
    }

    /// Gets the value of IPISendBroadcastRequestsPersec
    pub fn get_ipisend_broadcast_requests_persec(&self) -> Option<&u32> {
        self.ipisend_broadcast_requests_persec.as_ref()
    }

    /// Sets the value of IPISendRoutineRequestsPersec
    pub fn set_ipisend_routine_requests_persec(&mut self, value: u32) {
        self.ipisend_routine_requests_persec = Some(value);
    }

    /// Gets the value of IPISendRoutineRequestsPersec
    pub fn get_ipisend_routine_requests_persec(&self) -> Option<&u32> {
        self.ipisend_routine_requests_persec.as_ref()
    }

    /// Sets the value of IPISendSoftwareInterruptsPersec
    pub fn set_ipisend_software_interrupts_persec(&mut self, value: u32) {
        self.ipisend_software_interrupts_persec = Some(value);
    }

    /// Gets the value of IPISendSoftwareInterruptsPersec
    pub fn get_ipisend_software_interrupts_persec(&self) -> Option<&u32> {
        self.ipisend_software_interrupts_persec.as_ref()
    }

    /// Sets the value of SpinlockAcquiresPersec
    pub fn set_spinlock_acquires_persec(&mut self, value: u32) {
        self.spinlock_acquires_persec = Some(value);
    }

    /// Gets the value of SpinlockAcquiresPersec
    pub fn get_spinlock_acquires_persec(&self) -> Option<&u32> {
        self.spinlock_acquires_persec.as_ref()
    }

    /// Sets the value of SpinlockContentionsPersec
    pub fn set_spinlock_contentions_persec(&mut self, value: u32) {
        self.spinlock_contentions_persec = Some(value);
    }

    /// Gets the value of SpinlockContentionsPersec
    pub fn get_spinlock_contentions_persec(&self) -> Option<&u32> {
        self.spinlock_contentions_persec.as_ref()
    }

    /// Sets the value of SpinlockSpinsPersec
    pub fn set_spinlock_spins_persec(&mut self, value: u32) {
        self.spinlock_spins_persec = Some(value);
    }

    /// Gets the value of SpinlockSpinsPersec
    pub fn get_spinlock_spins_persec(&self) -> Option<&u32> {
        self.spinlock_spins_persec.as_ref()
    }
}


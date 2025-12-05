// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Synchronized struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Synchronized {

/// CopyMethodology specifies what copy methodology the copy engine uses to create and/or maintain the target element. Values are: 
/// 0 - 'Not Specified': The method of maintaining the copy is not specified. 
/// 3 - 'Full Copy': This indicates that a full copy of the source object is (or will be) generated. 
/// 4 - 'Incremental-Copy': Only changed data from source element is copied to target element. 
/// 5 - 'Differential-Copy': Only the new writes to the source element are copied to the target element. 
/// 6 - 'Copy-On-Write': Affected data is copied on the first write to the source or to the target elements. 
/// 7 - 'Copy-On-Access': Affected data is copied on the first access to the source element. 
/// 8 - 'Delta-Update': Difference based replication where after the initial copy, only updates to source are copied to target. 
/// 9 - 'Snap-And-Clone': The service creates a snapshot of the source element first, then uses the snapshot as the source of the copy operation to the target element.
    #[serde(rename = "CopyMethodology")]
    pub copy_methodology: Option<Synchronized_CopyMethodology>,

/// CopyPriority allows the priority of background copy engine I/O to be managed relative to host I/O operations during a sequential background copy operation. Values are: 
/// 1 - 'Low': copy engine I/O lower priority than host I/O. 
/// 2 - 'Same': copy engine I/O has the same priority as host I/O. 
/// 3 - 'High': copy engine I/O has higher priority than host I/O.
    #[serde(rename = "CopyPriority")]
    pub copy_priority: Option<Synchronized_CopyPriority>,

/// CopyState describes the state of the association with respect to replication activity. Values are: 
/// 2 - 'Initialized': The link to enable replication is established and source/replica elements are associated, but the copy operation has not started. 
/// 3 - 'Unsynchronized': Not all the source element data has been copied to the target element. 
/// 4 - 'Synchronized': For the Mirror, Snapshot, or Clone replication, the target represents a copy of the source. 
/// 5 - 'Broken': The relationship is non-functional due to errors in the source, the target, the path between the two or space constraints. 
/// 6 - 'Fractured': Target is split from the source. 
/// 7 - 'Split': The target element was gracefully (or systematically) split from its source element -- consistency is guaranteed. 
/// 8 - 'Inactive': Copy operation has stopped, writes to source element will not be sent to target element. 
/// 9 - 'Suspended': Data flow between the source and target elements has stopped. Writes to source element are held until the association is resumed. 
/// 10 - 'Failedover': Reads and writes to/from the target element. Source element is not reachable. 
/// 11 - 'Prepared': Initialization completed and the copy operation started; however, the data flow has not started. 
/// 12 - 'Aborted': The copy operation is aborted with the Abort operation. Use the Resync Replica operation to restart the copy operation. 
/// 13 - 'Skewed': The target has been modified and is no longer synchronized with the source element or the point-in-time view. 
/// 14 - 'Mixed': Applies to the CopyState of GroupSynchronized. It indicates the StorageSynchronized associations of the elements in the groups have different CopyState values.
    #[serde(rename = "CopyState")]
    pub copy_state: Option<Synchronized_CopyState>,

/// CopyType describes the Replication Policy. Values are: 
/// 2 - 'Async': create and maintain an asynchronous copy of the source. 
/// 3 - 'Sync': create and maintain a synchronized copy of the source. 
/// 4 - 'UnSyncAssoc': create an unsynchronized copy and maintain an association to the source. 
/// 5 - 'UnSyncUnAssoc': create an unsynchronized copy with a temporary association that is deleted upon completion of the copy operation.
    #[serde(rename = "CopyType")]
    pub copy_type: Option<Synchronized_CopyType>,

/// Specifies the percent of the work completed to reach synchronization. Must be set to NULL if implementation is not capable of providing this information.
    #[serde(rename = "PercentSynced")]
    pub percent_synced: Option<u16>,

/// ProgressStatus describes the status of the association with respect to Replication activity. Values are: 
/// 2 - 'Completed': The request is completed. Copy operation is idle. 
/// 3 - 'Dormant': Indicates that the copy operation is inactive suspended or quiesced. 
/// 4 - 'Initializing': In the process of establishing source/replica association and the copy operation has not started. 
/// 5 - 'Preparing': preparation-in-progress. 
/// 6 - 'Synchronizing': sync-in-progress. 
/// 7 - 'Resyncing': resync-in-progress. 
/// 8 - 'Restoring': restore-in-progress. 
/// 9 - 'Fracturing': fracture-in-progress. 
/// 10 - 'Splitting': split-in-progress. 
/// 11 - 'Failing over': in the process of switching source and target. 
/// 12 - 'Failing back': Undoing the result of failover. 
/// 13 - 'Detaching': detach-in-progress. 
/// 14 - 'Aborting': abort-in-progress. 
/// 15 - 'Mixed': Applies to groups with element pairs with different statuses. Generally, the individual statuses need to be examined. 
/// 16 - 'Suspending': The copy operation is in the process of being suspended. 
/// 17 - 'Requires fracture': The requested operation has completed, however, the synchronization relationship needs to be fractured before further copy operations can be issued. 
/// 18 - 'Requires resync': The requested operation has completed, however, the synchronization relationship needs to be resynced before further copy operations can be issued. 
/// 19 - 'Requires activate': The requested operation has completed, however, the synchronization relationship needs to be activated before further copy operations can be issued. 
/// 20 - 'Pending': The flow of data has stopped momentarily due to limited bandwidth or busy system.
    #[serde(rename = "ProgressStatus")]
    pub progress_status: Option<Synchronized_ProgressStatus>,

/// Recovery Point Objective indicates the maximum interval in which data might be lost. For synchronous copy operations, RPO is 0. For asynchronous copy operations RPO represents the interval since the most recent transmission of data to the target element.
    #[serde(rename = "RecoveryPointObjective")]
    pub recovery_point_objective: Vec<u16>,

/// ReplicaType provides information on how the Replica is being maintained. Values are: 
/// 2 - 'Full Copy': This indicates that a full copy of the source object is (or will be) generated . 
/// 3 - 'Before Delta': This indicates that the source object will be maintained as a delta data from the replica. 
/// 4 - 'After Delta': This indicates that the replica will be maintained as delta data from the source object. 
/// 5 - 'Log': This indicates that the replica object is being maintained as a log of changes to the source. 
/// 0 - 'Not Specified': The method of maintaining the copy is not specified.
    #[serde(rename = "ReplicaType")]
    pub replica_type: Option<Synchronized_ReplicaType>,

/// RequestedCopyState is an integer enumeration that indicates the last requested or desired state for the association. The actual state of the association is represented by CopyState. Note that when CopyState reaches the requested state, this property will be set to 'Not Applicable.
    #[serde(rename = "RequestedCopyState")]
    pub requested_copy_state: Option<u16>,

/// Boolean indicating whether synchronization is maintained.
    #[serde(rename = "SyncMaintained")]
    pub sync_maintained: Option<bool>,

/// Mode describes whether the target elements will be updated synchronously or asynchronously. If NULL, implementation decides the mode.
    #[serde(rename = "SyncMode")]
    pub sync_mode: Option<Synchronized_SyncMode>,

/// SyncState describes the state of the association with respect to Replication activity. Values are: 
/// 2 - 'Initialized': The link to enable replication is established and source/replica elements are associated, but the Copy engine has not started. 
/// 3 - 'PrepareInProgress': Preparation for Replication is in progress and the Copy engine has started. 
/// 4 - 'Prepared': All necessary preparation has completed. 
/// 5 - 'ResyncInProgress': Synchronization or Resynchronization is in progress. This may be the initial 'copy' or subsequent changes being copied. 
/// 6 - 'Synchronized': An Async or Sync replication is currently synchronized. When this value is set, SyncMaintained will be true. 
/// 7 - 'FractureInProgress': An operation to fracture an Async or Sync replication is in progress. 
/// 8 - 'QuiesceInProgress': A quiesce operation is in progress. 
/// 9 - 'Quiesced': The replication has been quiesced and is ready for a change. 
/// 10 - 'RestoreInProgress': An operation is in progress to copy the Synced object to the System object. 
/// 11 - 'Idle': The 'normal' state for an UnSyncAssoc replica. 
/// 12 - 'Broken': The relationship is non-functional due to errors in the source, the target, the path between the two or space constraints. 
/// 13 - 'Fractured': An Async or Sync replication is fractured. 
/// 14 - 'Frozen': All blocks copied from source to an UnSyncAssoc replica and the copy engine is stopped. 
/// 15 - 'CopyInProgress': A deferred background copy operation is in progress to copy the source to the replica target for an UnSyncAssoc association. 
/// 
    #[serde(rename = "SyncState")]
    pub sync_state: Option<Synchronized_SyncState>,

/// The point in time that the virtual disks were synchronized.
    #[serde(rename = "SyncTime")]
    pub sync_time: Option<String>,

/// SyncType describes the intended outcome of the replication. Values are: 
/// 6 - 'Mirror': create and maintain a copy of the source. 
/// 7 - 'Snapshot': create a point-in-time, virtual copy of the source. 
/// 8 - 'Clone': create a point-in-time, full copy the source.
    #[serde(rename = "SyncType")]
    pub sync_type: Option<Synchronized_SyncType>,
}

impl MSFT_Synchronized {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            copy_methodology: None,
            copy_priority: None,
            copy_state: None,
            copy_type: None,
            percent_synced: None,
            progress_status: None,
            recovery_point_objective: Vec::new(),
            replica_type: None,
            requested_copy_state: None,
            sync_maintained: None,
            sync_mode: None,
            sync_state: None,
            sync_time: None,
            sync_type: None,
        }
    }


    /// Sets the value of CopyMethodology
    pub fn set_copy_methodology(&mut self, value: Synchronized_CopyMethodology) {
        self.copy_methodology = Some(value);
    }

    /// Gets the value of CopyMethodology
    pub fn get_copy_methodology(&self) -> Option<&Synchronized_CopyMethodology> {
        self.copy_methodology.as_ref()
    }

    /// Sets the value of CopyPriority
    pub fn set_copy_priority(&mut self, value: Synchronized_CopyPriority) {
        self.copy_priority = Some(value);
    }

    /// Gets the value of CopyPriority
    pub fn get_copy_priority(&self) -> Option<&Synchronized_CopyPriority> {
        self.copy_priority.as_ref()
    }

    /// Sets the value of CopyState
    pub fn set_copy_state(&mut self, value: Synchronized_CopyState) {
        self.copy_state = Some(value);
    }

    /// Gets the value of CopyState
    pub fn get_copy_state(&self) -> Option<&Synchronized_CopyState> {
        self.copy_state.as_ref()
    }

    /// Sets the value of CopyType
    pub fn set_copy_type(&mut self, value: Synchronized_CopyType) {
        self.copy_type = Some(value);
    }

    /// Gets the value of CopyType
    pub fn get_copy_type(&self) -> Option<&Synchronized_CopyType> {
        self.copy_type.as_ref()
    }

    /// Sets the value of PercentSynced
    pub fn set_percent_synced(&mut self, value: u16) {
        self.percent_synced = Some(value);
    }

    /// Gets the value of PercentSynced
    pub fn get_percent_synced(&self) -> Option<&u16> {
        self.percent_synced.as_ref()
    }

    /// Sets the value of ProgressStatus
    pub fn set_progress_status(&mut self, value: Synchronized_ProgressStatus) {
        self.progress_status = Some(value);
    }

    /// Gets the value of ProgressStatus
    pub fn get_progress_status(&self) -> Option<&Synchronized_ProgressStatus> {
        self.progress_status.as_ref()
    }

    /// Sets the value of RecoveryPointObjective
    pub fn set_recovery_point_objective(&mut self, value: Vec<u16>) {
        self.recovery_point_objective = value;
    }

    /// Gets the value of RecoveryPointObjective
    pub fn get_recovery_point_objective(&self) -> &Vec<u16> {
        &self.recovery_point_objective
    }

    /// Sets the value of ReplicaType
    pub fn set_replica_type(&mut self, value: Synchronized_ReplicaType) {
        self.replica_type = Some(value);
    }

    /// Gets the value of ReplicaType
    pub fn get_replica_type(&self) -> Option<&Synchronized_ReplicaType> {
        self.replica_type.as_ref()
    }

    /// Sets the value of RequestedCopyState
    pub fn set_requested_copy_state(&mut self, value: u16) {
        self.requested_copy_state = Some(value);
    }

    /// Gets the value of RequestedCopyState
    pub fn get_requested_copy_state(&self) -> Option<&u16> {
        self.requested_copy_state.as_ref()
    }

    /// Sets the value of SyncMaintained
    pub fn set_sync_maintained(&mut self, value: bool) {
        self.sync_maintained = Some(value);
    }

    /// Gets the value of SyncMaintained
    pub fn get_sync_maintained(&self) -> Option<&bool> {
        self.sync_maintained.as_ref()
    }

    /// Sets the value of SyncMode
    pub fn set_sync_mode(&mut self, value: Synchronized_SyncMode) {
        self.sync_mode = Some(value);
    }

    /// Gets the value of SyncMode
    pub fn get_sync_mode(&self) -> Option<&Synchronized_SyncMode> {
        self.sync_mode.as_ref()
    }

    /// Sets the value of SyncState
    pub fn set_sync_state(&mut self, value: Synchronized_SyncState) {
        self.sync_state = Some(value);
    }

    /// Gets the value of SyncState
    pub fn get_sync_state(&self) -> Option<&Synchronized_SyncState> {
        self.sync_state.as_ref()
    }

    /// Sets the value of SyncTime
    pub fn set_sync_time(&mut self, value: String) {
        self.sync_time = Some(value);
    }

    /// Gets the value of SyncTime
    pub fn get_sync_time(&self) -> Option<&String> {
        self.sync_time.as_ref()
    }

    /// Sets the value of SyncType
    pub fn set_sync_type(&mut self, value: Synchronized_SyncType) {
        self.sync_type = Some(value);
    }

    /// Gets the value of SyncType
    pub fn get_sync_type(&self) -> Option<&Synchronized_SyncType> {
        self.sync_type.as_ref()
    }
}


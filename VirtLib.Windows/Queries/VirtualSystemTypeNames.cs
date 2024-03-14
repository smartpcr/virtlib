// -----------------------------------------------------------------------
// <copyright file="VirtualSystemTypeNames.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

public static class VirtualSystemTypeNames
{
    public const string RealizedVM = "Microsoft:Hyper-V:System:Realized";
    public const string PlannedVM = "Microsoft:Hyper-V:System:Planned";
    public const string RealizedSnapshot = "Microsoft:Hyper-V:Snapshot:Realized";
    public const string RecoverySnapshot = "Microsoft:Hyper-V:Snapshot:Recovery";
    public const string PlannedSnapshot = "Microsoft:Hyper-V:Snapshot:Planned";
    public const string MissingSnapshot = "Microsoft:Hyper-V:Snapshot:Missing";
    public const string ReplicaStandardRecoverySnapshot = "Microsoft:Hyper-V:Snapshot:Replica:Standard";
    public const string ReplicaApplicationConsistentRecoverySnapshot = "Microsoft:Hyper-V:Snapshot:Replica:ApplicationConsistent";
    public const string ReplicaPlannedRecoverySnapshot = "Microsoft:Hyper-V:Snapshot:Replica:PlannedFailover";
    public const string ReplicaSettings = "Microsoft:Hyper-V:Replica";
}
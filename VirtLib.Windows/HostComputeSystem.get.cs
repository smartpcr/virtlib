// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.get.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Linq;
using System.Management;
using Models;
using Newtonsoft.Json;
using Queries;

public partial class HostComputeSystem
{
    public OSInfo? GetOsInfo()
    {
        using var mc = new ManagementClass(Win32OperatingSystem);
        using var moc = mc.GetInstances();
        using var os = moc.OfType<ManagementObject>().FirstOrDefault();
        return os == null ? null : new OSInfo(this._serviceProvider, os);
    }

    public HyperVHostInfo? GetHyperVHost()
    {
        ObjectQuery query = new ObjectQuery($"SELECT * FROM {MsvmVirtualSystemManagementService}");
        using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
        using var collection = searcher.Get();
        using var managementObject = collection.OfType<ManagementObject>().FirstOrDefault();
        return managementObject == null ? null : new HyperVHostInfo(this._serviceProvider, managementObject);
    }

    public SwitchInfo? GetVSwitch(string switchName)
    {
        var query = new ObjectQuery(string.Format(VSwitchQueries.GetVSwitchByName, switchName));
        using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
        using var collection = searcher.Get();
        using var instance = collection.OfType<ManagementObject>().FirstOrDefault();
        return instance == null ? null : new SwitchInfo(this._serviceProvider, instance);
    }

    public bool IsVirtualSwitchExist(string virtualSwitchName)
    {
        var vswitch = GetVSwitch(virtualSwitchName);
        return vswitch != null;
    }

    public VirtualMachine? GetVirtualMachine(string vmName)
    {
        var query = new ObjectQuery(string.Format(VMQueries.GetVMByName, vmName));
        using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
        using var collection = searcher.Get();
        using var instance = collection.OfType<ManagementObject>().FirstOrDefault();
        var vm = instance == null ? null : new VirtualMachine(this._serviceProvider, instance);
        if (vm != null)
        {
            var json = JsonConvert.SerializeObject(vm, this._serializerSetting);
            this._logger.LogVM(vmName, json);
        }

        return vm;
    }

    public bool IsVirtualMachineExist(string vmName)
    {
        var query = new ObjectQuery(string.Format(VMQueries.GetVMByName, vmName));
        using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
        using var collection = searcher.Get();
        return collection.Count > 0;
    }
}
// -----------------------------------------------------------------------
// <copyright file="VMCreator.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace Generation2VM;

using System;
using System.Management;

public static class VMCreator
{
    internal static void CreateGeneration2VM(string serverName, string vmName)
    {
        var classPath = new ManagementPath
        {
            Server = serverName,
            NamespacePath = @"\root\virtualization\v2",
            ClassName = "Msvm_VirtualSystemSettingData"
        };

        using var virtualSystemSettingClass = new ManagementClass(classPath);
        var scope = new ManagementScope($@"\\{serverName}\root\virtualization\v2", null);
        virtualSystemSettingClass.Scope = scope;
        using var virtualSystemSetting = virtualSystemSettingClass.CreateInstance();
        virtualSystemSetting["ElementName"] = vmName;
        virtualSystemSetting["VirtualSystemSubType"] = "Microsoft:Hyper-V:SubType:2";
        virtualSystemSetting["ConfigurationDataRoot"] = "C:\\ProgramData\\Microsoft\\Windows\\Hyper-V\\";

        string embeddedInstance = virtualSystemSetting.GetText(TextFormat.CimDtd20);
        using var service = WmiUtilities.GetVirtualMachineManagementService(scope);
        using var inParams = service.GetMethodParameters("DefineSystem");
        inParams["SystemSettings"] = embeddedInstance;

        using var outParams = service.InvokeMethod("DefineSystem", inParams, null);
        Console.WriteLine($"create vm {vmName} result. {outParams["ReturnValue"]}");
    }
}
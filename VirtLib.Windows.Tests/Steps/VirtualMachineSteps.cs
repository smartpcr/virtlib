namespace VirtLib.Windows.Tests.Steps;

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using FluentAssertions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;
using TechTalk.SpecFlow;
using Windows.Models;
using Definitions;
using Xunit.Abstractions;
using ScsiController = Definitions.ScsiController;

[Binding]
public class VirtualMachineSteps
{
    private readonly ScenarioContext _context;
    private readonly ITestOutputHelper _outputHelper;

    public VirtualMachineSteps(ScenarioContext context, ITestOutputHelper outputHelper)
    {
        _context = context;
        _outputHelper = outputHelper;
    }

    [When(@"I get vm with name ""(.*)""")]
    public void WhenIGetVmWithName(string vmName)
    {
        var hcs = _context.Get<HostComputeSystem>();
        var vm = hcs.GetVirtualMachine(vmName);
        _context.Set(vm);
    }

    [Then(@"vm should exist with name ""(.*)""")]
    public void ThenVmShouldExistWithName(string vmName)
    {
        var vm = _context.Get<VirtualMachine>();
        vm.Should().NotBeNull();
        var serializerSetting = new JsonSerializerSettings
        {
            Formatting = Formatting.Indented,
            ContractResolver = new CamelCasePropertyNamesContractResolver(),
            Converters = new List<JsonConverter> { new StringEnumConverter() }
        };
        var json = JsonConvert.SerializeObject(vm, serializerSetting);
        _outputHelper.WriteLine(json);
        vm.Name.Should().Be(vmName);
    }

    [Given(@"ubuntu image ""(.*)"" is downloaded from url ""(.*)""")]
    public async Task GivenUbuntuImageIsDownloadedFromUrl(string isoFileName, string isoUrl)
    {
        string? userFolder = RuntimeInformation.IsOSPlatform(OSPlatform.Windows)
            ? Environment.GetEnvironmentVariable("USERPROFILE")
            : Environment.GetEnvironmentVariable("HOME");
        userFolder.Should().NotBeNull();
        Directory.Exists(userFolder).Should().BeTrue();
        var hyperVFolder = Path.Combine(userFolder!, "Hyper-V");
        if (!Directory.Exists(hyperVFolder))
        {
            Directory.CreateDirectory(hyperVFolder);
        }

        var isoFilePath = Path.Combine(hyperVFolder, isoFileName);
        _context.Set(hyperVFolder, "hyper-v");
        _context.Set(isoFilePath, "iso-file");
        if (File.Exists(isoFilePath))
        {
            return;
        }

        using var httpClient = new HttpClient();
        using var response = await httpClient.GetAsync(isoUrl, HttpCompletionOption.ResponseHeadersRead);
        response.EnsureSuccessStatusCode();
        await using var streamToReadFrom = await response.Content.ReadAsStreamAsync();
        await using var streamToWriteTo = File.Open(isoFilePath, FileMode.Create);
        await streamToReadFrom.CopyToAsync(streamToWriteTo);
    }

    [When(@"the following cloud-init settings")]
    public void WhenTheFollowingCloudInitSettings()
    {
        ScenarioContext.StepIsPending();
    }

    [When(@"I create gen (\d) vm with name ""(.*)""")]
    public void WhenICreateGenVmWithName(int generation, string vmName, Table table)
    {
        var hcs = _context.Get<HostComputeSystem>();
        var hyperVFolder = _context.Get<string>("hyper-v");
        Directory.Exists(hyperVFolder).Should().BeTrue();
        var isoFile = _context.Get<string>("iso-file");
        File.Exists(isoFile).Should().BeTrue();

        var createVmRequest = new VirtualMachineDefinition(vmName, hyperVFolder)
        {
            Generation = generation == 1? Generation.Gen1 : Generation.Gen2
        };

        // processor
        var cpu = uint.Parse(table.Rows.First(r => r["Name"] == "CpuCount")["Value"]);
        createVmRequest.Processor.Quantity = cpu;

        // memory
        var memory = ulong.Parse(table.Rows.First(r => r["Name"] == "MemoryInMB")["Value"]);
        createVmRequest.Memory.Minimum = 32;
        createVmRequest.Memory.Maximum = memory;

        // network adapter
        var vswitchName = table.Rows.First(r => r["Name"] == "SwitchName")["Value"];
        createVmRequest.NetworkAdapters.Add(new NetworkAdapter
        {
            VirtualSwitch = vswitchName
        });

        // dvd drive
        var scsiController = new Definitions.ScsiController();
        var scsiDrive = new Definitions.VirtualDvdDrive
        {
            VirtualDvdDisk = new VirtualDvdDisk(isoFile)
        };
        scsiController.Drives = new IScsiDrive[] { scsiDrive };
        createVmRequest.ScsiControllers.Add(scsiController);

        // hard drive
        var vhdxFileSize = ulong.Parse(table.Rows.First(r => r["Name"] == "HardDiskSizeInGB")["Value"]);
        var hardDrive = new Definitions.VirtualHardDrive(
            new VirtualHardDisk(
            VirtualHardDiskFormat.Vhdx,
            VirtualHardDiskType.FixedSize,
            vhdxFileSize,
            Path.Combine(hyperVFolder, $"{vmName}.vhdx")));
        var hardDriveController = new Definitions.ScsiController
        {
            Drives = new IScsiDrive[]{hardDrive}
        };
        createVmRequest.ScsiControllers.Add(hardDriveController);

        var vm = hcs.CreateVirtualMachine(createVmRequest);
        _context.Set(vm);
    }
}
namespace VirtLib.Windows.Tests.Steps;

using System.Collections.Generic;
using FluentAssertions;
using Hooks;
using Newtonsoft.Json;
using TechTalk.SpecFlow;
using Windows.Models;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;
using Xunit.Abstractions;

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

    [Given(@"ubuntu image is downloaded from url ""(.*)""")]
    public void GivenUbuntuImageIsDownloadedFromUrl(string p0)
    {
        ScenarioContext.StepIsPending();
    }

    [When(@"I create vm with name ""(.*)""")]
    public void WhenICreateVmWithName(string p0)
    {
        ScenarioContext.StepIsPending();
    }

    [When(@"the following cloud-init settings")]
    public void WhenTheFollowingCloudInitSettings()
    {
        ScenarioContext.StepIsPending();
    }
}
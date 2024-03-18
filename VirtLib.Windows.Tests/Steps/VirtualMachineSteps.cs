namespace VirtLib.Windows.Tests.Steps;

using FluentAssertions;
using Hooks;
using Newtonsoft.Json;
using TechTalk.SpecFlow;
using Windows.Models;
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
        var json = JsonConvert.SerializeObject(vm);
        _outputHelper.WriteLine(json);
        vm.Name.Should().Be(vmName);
    }
}
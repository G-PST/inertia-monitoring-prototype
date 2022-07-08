using PowerSystems
using PowerSimulations
using PowerSystemCaseBuilder
using HiGHS
using Logging

solver = optimizer_with_attributes(
    HiGHS.Optimizer,
    "mip_rel_gap" => 0.1,
    "presolve" => "on",
    "mip_feasibility_tolerance" => 1e-04,
    "primal_feasibility_tolerance" => 1e-04,
)

da_sys = build_system(PSITestSystems, "modified_RTS_GMLC_DA_sys")
rt_sys = build_system(PSITestSystems, "modified_RTS_GMLC_RT_sys")

template_da = template_unit_commitment()
set_device_model!(template_da, ThermalStandard, ThermalBasicUnitCommitment)

template_rt = template_economic_dispatch()
set_device_model!(template_rt, ThermalStandard, ThermalBasicUnitCommitment)

# ## simulation
models = SimulationModels(
    decision_models = [
        DecisionModel(
            template_da,
            da_sys,
            optimizer = solver,
            name = "DA",
            optimizer_solve_log_print = false,
            initialize_model = true,
        ),
        DecisionModel(
            template_rt,
            rt_sys,
            optimizer = solver,
            name = "RT",
            optimizer_solve_log_print = false,
            initialize_model = true,
        ),
    ],
)


feedforward = Dict(
    "RT" => [
        SemiContinuousFeedforward(
            component_type = ThermalStandard,
            source = OnVariable,
            affected_values = [ActivePowerVariable],
        ),
    ],
)


sequence = SimulationSequence(
    models = models,
    ini_cond_chronology = InterProblemChronology(),
    #feedforwards = feedforward,
)

sim = Simulation(
    name = "demo-rts-sim",
    steps = 3,
    models = models,
    sequence = sequence,
    simulation_folder = ".",
)

build!(sim, serialize = false, console_level = Logging.Info)

exports = Dict(
    "models" => [
        Dict("name" => "DA"),
        Dict("name" => "RT", "variables" => ["OnVariable__ThermalStandard"]),
    ],
    "path" => ".",
    "optimizer_stats" => false,
)
execute!(sim, enable_progress_bar = true, exports = exports)

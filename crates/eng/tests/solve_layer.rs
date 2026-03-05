use eng::devices::NozzleFlowBranch;
use eng::solve::{
    NozzleShockWorkflowRequest, bisect_by_sign_change, find_roots_by_scan_bisection, rk4_step_2,
    run_nozzle_normal_shock_workflow,
};

#[test]
fn numeric_root_wrappers_converge() {
    let out = bisect_by_sign_change(1.0, 2.0, 1e-12, 200, |x| Ok::<_, ()>(x * x - 2.0))
        .expect("bisection");
    assert!((out.root - 2.0_f64.sqrt()).abs() < 1e-10);

    let (roots, meta) = find_roots_by_scan_bisection(-4.0, 4.0, 200, 1e-12, 1e-7, |x| {
        Ok::<_, ()>((x - 1.0) * (x + 2.0))
    })
    .expect("scan");
    assert_eq!(meta.roots, 2);
    assert!(roots.iter().any(|r| (*r - 1.0).abs() < 1e-6));
    assert!(roots.iter().any(|r| (*r + 2.0).abs() < 1e-6));
}

#[test]
fn ode_wrapper_tracks_simple_oscillator() {
    let mut x = 0.0_f64;
    let mut y1 = 1.0_f64;
    let mut y2 = 0.0_f64;
    for _ in 0..100 {
        let (n1, n2) =
            rk4_step_2(x, y1, y2, 0.01, |_, a, b| Ok::<_, ()>((b, -a))).expect("rk4 step");
        x += 0.01;
        y1 = n1;
        y2 = n2;
    }
    assert!((y1 - x.cos()).abs() < 5e-7);
    assert!((y2 + x.sin()).abs() < 5e-7);
}

#[test]
fn workflow_chain_nozzle_to_normal_shock_is_provenanced() {
    let out = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {
        gamma: 1.4,
        area_ratio: 2.0,
        nozzle_branch: NozzleFlowBranch::Supersonic,
    })
    .expect("workflow chain");
    assert!(out.pre_shock_mach > 1.0);
    assert!(out.post_shock_mach < out.pre_shock_mach);
    assert!(out.shock_pressure_ratio > 1.0);
    assert!(out.path_text().contains("device.nozzle_flow_calc"));
    assert!(out.path_text().contains("device.normal_shock_calc"));
    assert!(
        out.workflow
            .station("s2_post_shock")
            .and_then(|s| s.quantities.get("shock_pressure_ratio"))
            .is_some()
    );
}

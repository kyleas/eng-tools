{
    let result = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Colebrook)
        .given_rho("1000 kg/m^3")
        .given_mu("1 cP")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve()?;

    let _dp = result.delta_p();
    let _re = result.reynolds_number().unwrap_or_default();
}

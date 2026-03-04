{
    let _dp = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Fixed(0.02))
        .given_rho("1000 kg/m3")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .solve_delta_p()?;
}

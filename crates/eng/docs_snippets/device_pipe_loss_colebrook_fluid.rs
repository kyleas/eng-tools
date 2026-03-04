{
    let _dp = eng::devices::pipe_loss()
        .friction_model(eng::devices::PipeFrictionModel::Colebrook)
        .fluid(eng::fluids::water().state_tp("300 K", "1 atm")?)
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve_delta_p()?;
}

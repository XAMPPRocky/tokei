// 142 lines 123 code 17 comments 2 blanks
// Example code from https://github.com/TheEconomist/us-potus-model/blob/85be55ae7b0bc68cb155a9ca975e155837eb4851/scripts/model/poll_model_2020.stan
data{
  int N_national_polls;    // Number of polls
  int N_state_polls;    // Number of polls
  int T;    // Number of days
  int S;    // Number of states (for which at least 1 poll is available) + 1
  int P;    // Number of pollsters
  int M;    // Number of poll modes
  int Pop;    // Number of poll populations
  int<lower = 1, upper = S + 1> state[N_state_polls]; // State index
  int<lower = 1, upper = T> day_state[N_state_polls];   // Day index
  int<lower = 1, upper = T> day_national[N_national_polls];   // Day index
  int<lower = 1, upper = P> poll_state[N_state_polls];  // Pollster index
  int<lower = 1, upper = P> poll_national[N_national_polls];  // Pollster index
  int<lower = 1, upper = M> poll_mode_state[N_state_polls];  // Poll mode index
  int<lower = 1, upper = M> poll_mode_national[N_national_polls];  // Poll mode index
  int<lower = 1, upper = Pop> poll_pop_state[N_state_polls];  // Poll mode index
  int<lower = 1, upper = Pop> poll_pop_national[N_national_polls];  // Poll mode index
  int n_democrat_national[N_national_polls];
  int n_two_share_national[N_national_polls];
  int n_democrat_state[N_state_polls];
  int n_two_share_state[N_state_polls];
  vector<lower = 0, upper = 1.0>[N_national_polls] unadjusted_national;
  vector<lower = 0, upper = 1.0>[N_state_polls] unadjusted_state;
  // cov_matrix[S] ss_cov_mu_b_walk;
  // cov_matrix[S] ss_cov_mu_b_T;
  // cov_matrix[S] ss_cov_poll_bias;
  //*** prior input
  vector[S] mu_b_prior;
  vector[S] state_weights;
  real sigma_c;
  real sigma_m;
  real sigma_pop;
  real sigma_measure_noise_national;
  real sigma_measure_noise_state;
  real sigma_e_bias;
  // covariance matrix and scales
  cov_matrix[S] state_covariance_0;
  real random_walk_scale;
  real mu_b_T_scale;
  real polling_bias_scale;
}
transformed data {
  real national_cov_matrix_error_sd = sqrt(transpose(state_weights) * state_covariance_0 * state_weights);
  cholesky_factor_cov[S] cholesky_ss_cov_poll_bias;
  cholesky_factor_cov[S] cholesky_ss_cov_mu_b_T;
  cholesky_factor_cov[S] cholesky_ss_cov_mu_b_walk;
  // scale covariance
  matrix[S, S] ss_cov_poll_bias = state_covariance_0 * square(polling_bias_scale/national_cov_matrix_error_sd);
  matrix[S, S] ss_cov_mu_b_T = state_covariance_0 * square(mu_b_T_scale/national_cov_matrix_error_sd);
  matrix[S, S] ss_cov_mu_b_walk = state_covariance_0 * square(random_walk_scale/national_cov_matrix_error_sd);
  // transformation
  cholesky_ss_cov_poll_bias = cholesky_decompose(ss_cov_poll_bias);
  cholesky_ss_cov_mu_b_T = cholesky_decompose(ss_cov_mu_b_T);
  cholesky_ss_cov_mu_b_walk = cholesky_decompose(ss_cov_mu_b_walk);
}
parameters {
  vector[S] raw_mu_b_T;
  matrix[S, T] raw_mu_b;
  vector[P] raw_mu_c;
  vector[M] raw_mu_m;
  vector[Pop] raw_mu_pop;
  real<offset=0, multiplier=0.02> mu_e_bias;
  real<lower = 0, upper = 1> rho_e_bias;
  vector[T] raw_e_bias;
  vector[N_national_polls] raw_measure_noise_national;
  vector[N_state_polls] raw_measure_noise_state;
  vector[S] raw_polling_bias;
  real mu_b_T_model_estimation_error;
}
transformed parameters {
  //*** parameters
  matrix[S, T] mu_b;
  vector[P] mu_c;
  vector[M] mu_m;
  vector[Pop] mu_pop;
  vector[T] e_bias;
  vector[S] polling_bias = cholesky_ss_cov_poll_bias * raw_polling_bias;
  vector[T] national_mu_b_average;
  real national_polling_bias_average = transpose(polling_bias) * state_weights;
  real sigma_rho;
  //*** containers
  vector[N_state_polls] logit_pi_democrat_state;
  vector[N_national_polls] logit_pi_democrat_national;
  //*** construct parameters
  mu_b[:,T] = cholesky_ss_cov_mu_b_T * raw_mu_b_T + mu_b_prior;  // * mu_b_T_model_estimation_error
  for (i in 1:(T-1)) mu_b[:, T - i] = cholesky_ss_cov_mu_b_walk * raw_mu_b[:, T - i] + mu_b[:, T + 1 - i];
  national_mu_b_average = transpose(mu_b) * state_weights;
  mu_c = raw_mu_c * sigma_c;
  mu_m = raw_mu_m * sigma_m;
  mu_pop = raw_mu_pop * sigma_pop;
  e_bias[1] = raw_e_bias[1] * sigma_e_bias;
  sigma_rho = sqrt(1-square(rho_e_bias)) * sigma_e_bias;
  for (t in 2:T) e_bias[t] = mu_e_bias + rho_e_bias * (e_bias[t - 1] - mu_e_bias) + raw_e_bias[t] * sigma_rho;
  //*** fill pi_democrat
  for (i in 1:N_state_polls){
    logit_pi_democrat_state[i] =
      mu_b[state[i], day_state[i]] +
      mu_c[poll_state[i]] +
      mu_m[poll_mode_state[i]] +
      mu_pop[poll_pop_state[i]] +
      unadjusted_state[i] * e_bias[day_state[i]] +
      raw_measure_noise_state[i] * sigma_measure_noise_state +
      polling_bias[state[i]];
  }
  logit_pi_democrat_national =
    national_mu_b_average[day_national] +
    mu_c[poll_national] +
    mu_m[poll_mode_national] +
    mu_pop[poll_pop_national] +
    unadjusted_national .* e_bias[day_national] +
    raw_measure_noise_national * sigma_measure_noise_national +
    national_polling_bias_average;
}

model {
  //*** priors
  raw_mu_b_T ~ std_normal();
  //mu_b_T_model_estimation_error ~ scaled_inv_chi_square(7, 1);
  to_vector(raw_mu_b) ~ std_normal();
  raw_mu_c ~ std_normal();
  raw_mu_m ~ std_normal();
  raw_mu_pop ~ std_normal();
  mu_e_bias ~ normal(0, 0.02);
  rho_e_bias ~ normal(0.7, 0.1);
  raw_e_bias ~ std_normal();
  raw_measure_noise_national ~ std_normal();
  raw_measure_noise_state ~ std_normal();
  raw_polling_bias ~ std_normal();
  //*** likelihood
  n_democrat_state ~ binomial_logit(n_two_share_state, logit_pi_democrat_state);
  n_democrat_national ~ binomial_logit(n_two_share_national, logit_pi_democrat_national);
}

generated quantities {
  matrix[T, S] predicted_score;
  for (s in 1:S){
    //predicted_score[1:T, s] = inv_logit(mu_a[1:T] + to_vector(mu_b[s, 1:T]));
    predicted_score[1:T, s] = inv_logit(to_vector(mu_b[s, 1:T]));
  }
}

#[derive(Debug, Clone)]
pub struct AR2Model {
  pub phi1: f64,      // AR(1) 系数
  pub phi2: f64,      // AR(2) 系数
  pub constant: f64,  // 常数项
  pub residuals: Vec<f64>,  // 残差
}

impl AR2Model {
  /// 从数据拟合 AR(2) 模型
  pub fn fit(data: &[f64]) -> Self {
    let n = data.len();

    if n < 3 {
      return Self {
        phi1: 0.0,
        phi2: 0.0,
        constant: data.first().copied().unwrap_or(0.0),
        residuals: Vec::new(),
      };
    }

    // 使用 Yule-Walker 方程估计参数（比 OLS 更稳定）
    let (phi1, phi2, constant) = estimate_ar2_yule_walker(data);

    // 计算残差
    let residuals = calculate_residuals(data, phi1, phi2, constant);

    Self {
      phi1,
      phi2,
      constant,
      residuals,
    }
  }

  /// 进行预测
  pub fn predict(&self, last_two_values: (f64, f64), steps: usize) -> Vec<f64> {
    let mut predictions = Vec::with_capacity(steps);
    let (mut x_t_minus_1, mut x_t_minus_2) = last_two_values;

    for _ in 0..steps {
      let forecast = self.constant + self.phi1 * x_t_minus_1 + self.phi2 * x_t_minus_2;
      let rounded = forecast.round();

      predictions.push(rounded);

      // 更新滞后值用于下一步预测
      x_t_minus_2 = x_t_minus_1;
      x_t_minus_1 = rounded;
    }

    predictions
  }

  // 获取模型参数的标准误
  // pub fn standard_errors(&self, data: &[f64]) -> (f64, f64) {
  //   if data.len() < 3 {
  //     return (0.0, 0.0);
  //   }
  //
  //   let n = data.len() as f64;
  //   let residual_variance = self.residuals.iter().map(|&x| x * x).sum::<f64>() / (n - 3.0);
  //
  //   // 简化计算
  //   let se_phi1 = (residual_variance / n).sqrt();
  //   let se_phi2 = (residual_variance / n).sqrt();
  //
  //   (se_phi1, se_phi2)
  // }
}

/// 使用 Yule-Walker 方程估计 AR(2) 参数
fn estimate_ar2_yule_walker(data: &[f64]) -> (f64, f64, f64) {
  let n = data.len() as f64;

  // 计算均值
  let mean = data.iter().sum::<f64>() / n;

  // 计算自协方差
  let gamma0 = data.iter()
      .map(|&x| (x - mean).powi(2))
      .sum::<f64>() / n;

  let gamma1 = (0..data.len() - 1)
      .map(|i| (data[i] - mean) * (data[i + 1] - mean))
      .sum::<f64>() / (n - 1.0);

  let gamma2 = (0..data.len() - 2)
      .map(|i| (data[i] - mean) * (data[i + 2] - mean))
      .sum::<f64>() / (n - 2.0);

  // 解 Yule-Walker 方程
  // φ1 = (γ1 * γ0 - γ2 * γ1) / (γ0^2 - γ1^2)
  // φ2 = (γ2 * γ0 - γ1^2) / (γ0^2 - γ1^2)
  let denominator = gamma0.powi(2) - gamma1.powi(2);

  let phi1 = if denominator.abs() > 1e-10 {
    (gamma1 * gamma0 - gamma2 * gamma1) / denominator
  } else {
    0.0
  };

  let phi2 = if denominator.abs() > 1e-10 {
    (gamma2 * gamma0 - gamma1.powi(2)) / denominator
  } else {
    0.0
  };

  // 常数项 c = μ * (1 - φ1 - φ2)
  let constant = mean * (1.0 - phi1 - phi2);

  (phi1, phi2, constant)
}

/// 计算残差
fn calculate_residuals(data: &[f64], phi1: f64, phi2: f64, constant: f64) -> Vec<f64> {
  let mut residuals = Vec::new();

  for i in 2..data.len() {
    let predicted = constant + phi1 * data[i - 1] + phi2 * data[i - 2];
    let residual = data[i] - predicted;
    residuals.push(residual);
  }

  residuals
}

/// 主要的预测函数
pub fn forecast_ar2_sg(data: &[f64], forecast_count: usize) -> Vec<i64> {
  // 过滤掉零值（与 Python 代码一致）
  let filtered_data: Vec<f64> = data.iter()
      .filter(|&&x| x != 0.0)
      .copied()
      .collect();

  if filtered_data.len() < 3 {
    let last_value = filtered_data.last().copied().unwrap_or(0.0) as i64;
    return vec![last_value; forecast_count];
  }

  // 拟合 AR(2) 模型
  let model = AR2Model::fit(&filtered_data);

  // 获取最后两个值用于预测
  let n = filtered_data.len();
  let last_two = (filtered_data[n - 1], filtered_data[n - 2]);

  // 进行预测
  let predictions = model.predict(last_two, forecast_count);

  // 转换为整数
  predictions.into_iter().map(|x| x as i64).collect()
}
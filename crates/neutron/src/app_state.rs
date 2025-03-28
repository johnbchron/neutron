#[derive(Clone)]
pub struct AppState {
  pub shutdown: bool,
  pub counter:  usize,
}

#[allow(clippy::derivable_impls)]
impl Default for AppState {
  fn default() -> Self {
    AppState {
      shutdown: false,
      counter:  0,
    }
  }
}

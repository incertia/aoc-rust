#[macro_export]
macro_rules! impl_solver {
  ($id:ident, $day:literal) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new::<()>($day, None, None, None, None);
  };
  ($id:ident, $day:literal, $p:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, None, Some($p), None, None);
  };
  ($id:ident, $day:literal, $p:ident, $a:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, None, Some($p), Some($a), None);
  };
  ($id:ident, $day:literal, $p:ident, $a:ident, $b:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, None, Some($p), Some($a), Some($b));
  };
}

#[macro_export]
macro_rules! impl_solver_named {
  ($id:ident, $day:literal, $name: literal) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new::<()>($day, Some($name), None, None, None);
  };
  ($id:ident, $day:literal, $name: literal, $p:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, Some($name), Some($p), None, None);
  };
  ($id:ident, $day:literal, $name: literal, $p:ident, $a:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, Some($name), Some($p), Some($a), None);
  };
  ($id:ident, $day:literal, $name: literal, $p:ident, $a:ident, $b:ident) => {
    pub const $id: $crate::AdventSolver =
      $crate::AdventSolver::new($day, Some($name), Some($p), Some($a), Some($b));
  };
}

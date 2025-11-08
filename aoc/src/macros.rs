#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
  ($_t:tt $sub:expr) => {
    $sub
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! count_tts {
  ($($tts:tt)*) => {<[()]>::len(&[$($crate::replace_expr!($tts ())),*])};
}

#[macro_export]
macro_rules! impl_solver {
  ($day:literal) => {
    pub const SOLVER: $crate::AdventSolver =
      $crate::AdventSolver::new::<()>($day, None, None, None);
  };
  ($day:literal, $p:ident) => {
    pub const SOLVER: $crate::AdventSolver = $crate::AdventSolver::new($day, Some($p), None, None);
  };
  ($day:literal, $p:ident, $a:ident) => {
    pub const SOLVER: $crate::AdventSolver =
      $crate::AdventSolver::new($day, Some($p), Some($a), None);
  };
  ($day:literal, $p:ident, $a:ident, $b:ident) => {
    pub const SOLVER: $crate::AdventSolver =
      $crate::AdventSolver::new($day, Some($p), Some($a), Some($b));
  };
}

#[macro_export]
macro_rules! impl_all_solvers {
  ($($c:ident),*) => {
    pub const SOLVERS: [$crate::AdventSolver; $crate::count_tts!($($c)*)] = [$($c::SOLVER),*];
  };
}

#[macro_export]
macro_rules! impl_year {
  ($y:literal, $($c:ident),*) => {
    $crate::impl_all_solvers!($($c),*);
    pub const SOLVER: $crate::AdventYearSolver = $crate::AdventYearSolver::new($y, &SOLVERS);
  };
}

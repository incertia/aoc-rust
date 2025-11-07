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
    pub const SOLVER: $crate::AdventSolver = $crate::AdventSolver::new_empty($day);
  };
  ($day:literal, $p:ident) => {
    pub const SOLVER: $crate::AdventSolver = $crate::AdventSolver::new::<$p>($day, None, None);
  };
  ($day:literal, $p:ident, $a:expr) => {
    pub const SOLVER: $crate::AdventSolver = $crate::AdventSolver::new::<$p>($day, Some($a), None);
  };
  ($day:literal, $p:ident, $a:expr, $b:expr) => {
    pub const SOLVER: $crate::AdventSolver =
      $crate::AdventSolver::new::<$p>($day, Some($a), Some($b));
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

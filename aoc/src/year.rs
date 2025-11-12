use crate::{AdventRuntimeError, AdventSolver};

use brunch::Benches;
use reqwest::blocking::Client;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct AdventYearSolver {
  pub year: i64,
  pub solvers: &'static [AdventSolver],
}

fn load_input(year: i64, day: i64) -> Result<Vec<u8>, AdventRuntimeError> {
  let cache: std::path::PathBuf = format!(".aoc-cache/{}/{}", year, day).into();
  Ok(if cache.exists() {
    std::fs::read(cache)?
  } else {
    println!(
      "cache not found at {}, trying to download input instead",
      cache.display(),
    );

    let mut session = String::with_capacity(256);
    let mut f = match std::fs::File::open(".session") {
      Ok(f) => Ok(f),
      Err(e) => {
        println!("unable to open session cookie at .session");
        Err(e)
      }
    }?;
    f.read_to_string(&mut session)?;
    drop(f);

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let buf: Vec<u8> = Client::new()
      .get(url)
      .header(
        reqwest::header::COOKIE,
        format!("session={}", session.trim()),
      )
      .send()?
      .bytes()?
      // i swear to god if this allocates i will do something unspeakable
      .into();

    println!("got {} bytes... saving to cache", buf.len());

    // serialize to cache
    // SAFETY: cache/{}/{} definitely has a parent when formatted with ints
    std::fs::create_dir_all(unsafe { cache.parent().unwrap_unchecked() })?;
    let mut f = std::fs::File::create_new(cache)?;
    f.write_all(&buf)?;
    drop(f);

    buf
  })
}

impl AdventYearSolver {
  pub const fn new(year: i64, solvers: &'static [AdventSolver]) -> Self {
    Self { year, solvers }
  }

  pub fn run(&self, day: i64) -> Result<(), AdventRuntimeError> {
    let input = load_input(self.year, day)?;

    let mut ran = 0;
    for s in self.solvers {
      if s.day() == day {
        s.run(&input);
        ran += 1;
      }
    }

    println!("ran {} solver{}", ran, if ran == 1 { "" } else { "s" });

    Ok(())
  }

  pub fn bench(
    &self,
    day: i64,
    samples: Option<u32>,
    benches: &mut Benches,
  ) -> Result<(), AdventRuntimeError> {
    let input = load_input(self.year, day)?;

    for s in self.solvers {
      if s.day() == day {
        s.bench(&input, samples, benches);
      }
    }

    Ok(())
  }
}

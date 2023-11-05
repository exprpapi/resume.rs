use tectonic::{
  config::PersistentConfig,
  digest::DigestData,
  driver::{
    ProcessingSession,
    ProcessingSessionBuilder,
  },
  io::{
    InputHandle,
    IoProvider,
    OpenResult,
  },
  status::{
    ChatterLevel,
    StatusBackend,
    termcolor::TermcolorStatusBackend,
  },
};
use tectonic_bundles::{
  dir::DirBundle, Bundle
};

use notify::{
  event::{Event, EventKind, AccessKind, AccessMode},
  recommended_watcher,
  RecursiveMode,
  Watcher,
};

use std::{
  path::Path,
  sync::mpsc::channel,
};

use crate::resume::Resume;
use crate::Error;

pub struct Session<'a> {
  src: &'a str,
  session: ProcessingSession,
  status: TermcolorStatusBackend,
}

impl<'a> Session<'a> {
  pub fn new(src: &'a str) -> Self {
    let _config = PersistentConfig::default();
    let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);
    let bundle = Box::<TestBundle>::default();
    let tex_input_name = "resume.tex";
    let primary_input_path = {
      let mut path = std::env::current_dir().unwrap();
      path.push(tex_input_name);
      path
    };
    let format_name = "plain";
    let format_cache_path = std::env::current_dir().unwrap();

    let mut pbuilder = ProcessingSessionBuilder::default();
    pbuilder
      .primary_input_path(primary_input_path)
      .tex_input_name(tex_input_name)
      .format_name(format_name)
      .format_cache_path(format_cache_path)
      .bundle(bundle);

    let session = pbuilder
      .create(&mut status)
      .expect("couldn't create processing session");

    Self {
      src,
      session,
      status,
    }
  }

  fn run(&mut self) -> Error {
    let status = &mut self.status;
    let session = &mut self.session;
    session.run(status)?;
    Ok(())
  }

  pub fn build(&mut self) -> Error {
    let src = std::fs::read_to_string(self.src)?;
    let resume = serde_yaml::from_str::<Resume>(&src)?;
    let dst_tex = swap_stem(self.src, ".yaml", ".tex");
    // let dst_pdf = swap_stem(self.src, ".yaml", ".pdf");
    let tex = resume.to_tex();
    println!("Writing tex to {dst_tex}");
    std::fs::write(dst_tex, tex)?;
    // println!("Writing pdf to {dst_pdf}");
    // std::fs::write(dst_pdf, &pdf)?;
    self.run()?;
    println!("Done.");
    Ok(())
  }

  pub fn watch(&mut self) -> Error {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher.watch(Path::new(self.src), RecursiveMode::NonRecursive).unwrap();
    println!("Watching for changes in: {}", self.src);

    for r in rx.into_iter().flatten() {
      if let Event{kind: EventKind::Access(AccessKind::Close(AccessMode::Write)), .. } = r {
        println!("File modified. Rebuilding...");
        self.build()?;
      }
    }
    Ok(())
  }  
}

struct TestBundle(DirBundle);

impl Default for TestBundle {
  fn default() -> Self {
    TestBundle(DirBundle::new("assets"))
  }
}

impl IoProvider for TestBundle {
    // All other functions can default to NotAvailable/error.
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.0.input_open_name(name, status)
    }
}

impl Bundle for TestBundle {
  fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> anyhow::Result<DigestData> {
    Ok(DigestData::zeros())
  }

  fn all_files(&mut self, status: &mut dyn StatusBackend) -> anyhow::Result<Vec<String>> {
    self.0.all_files(status)
  }
}

fn swap_stem(file: &str, from: &str, to: &str) -> String {
  if !file.ends_with(from) {
    panic!("file does not have the expected stem");
  }
  let stem = Path::new(file).file_stem().and_then(|stem| stem.to_str()).unwrap_or_default();
  format!("{stem}{to}")
}

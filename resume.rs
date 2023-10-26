#!/usr/bin/env -S cargo +nightly -Zunstable-options -Zscript

//! ```cargo
//! cargo-features = ["edition2024"]
//! package.edition = "2024"
//! [dependencies]
//! clap = { version = "4.4.6", features = ["derive"] }
//! indoc = "2.0.4"
//! regex = "1.10"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! tectonic = "0.14"
//! ```

use clap::{Parser, Subcommand};
use indoc::{formatdoc, indoc};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  match Cli::parse().command {
    Some(Commands::Pdf{src}) => compile(&src, true),
    Some(Commands::Tex{src}) => compile(&src, false),
    None => compile("resume.yaml", true),
  }?;
  Ok(())
}

fn compile(src: &str, emit_pdf: bool) -> Result<(), Box<dyn std::error::Error>> {
  let resume = serde_yaml::from_str::<Resume>(&std::fs::read_to_string(src)?)?;
  let tex = resume.to_tex();
  let dst_tex = swap_stem(src, ".yaml", ".tex");
  std::fs::write(dst_tex, &tex)?;
  if emit_pdf {
    let dst_pdf = swap_stem(src, ".yaml", ".pdf");
    std::fs::write(dst_pdf, Resume::to_pdf(&tex))?;
  }
  Ok(())
}

fn swap_stem(file: &str, from: &str, to: &str) -> String {
  if !file.ends_with(from) {
    panic!("file does not have the expected stem");
  }
  let stem = Path::new(file).file_stem().and_then(|stem| stem.to_str()).unwrap_or_default();
  format!("{stem}{to}")
}

#[derive(Debug, Parser)]
#[command(about = "resume generator", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  Pdf { src: String },
  Tex { src: String },
}

#[derive(Deserialize)]
struct Resume {
  contact: Contact,
  education: Educations,
  experience: Experiences,
  projects: Projects,
  skills: Skills,
}

#[derive(Deserialize)]
struct Contact {
  name: String,
  email: String,
  github: String,
}

#[derive(Deserialize)]
struct Educations(Vec<Education>);

#[derive(Deserialize)]
struct Education {
  program: String,
  institution: String,
  graduation: String,
  description: Vec<String>,
}

#[derive(Deserialize)]
struct Experiences(Vec<Experience>);

#[derive(Deserialize)]
struct Experience {
  position: String,
  company: String,
  begin: String,
  end: String,
  description: Vec<String>,
}

#[derive(Deserialize)]
struct Projects(Vec<Project>);

#[derive(Deserialize)]
struct Project {
  title: String,
  category: String,
  github: String,
  description: Vec<String>,
}

#[derive(Deserialize)]
struct Skills(Vec<Skill>);

#[derive(Deserialize)]
struct Skill {
  area: String,
  description: String,
}

impl Resume {
  fn to_pdf(tex: &str) -> Vec<u8> {
    tectonic::latex_to_pdf(tex).expect("processing resume to pdf failed")
  }

  fn to_tex(&self) -> String {
    let contact = &self.contact.to_tex();
    let experience = &self.experience.to_tex();
    let education = &self.education.to_tex();
    let projects = &self.projects.to_tex();
    let skills = &self.skills.to_tex();
    let preamble = indoc!(r"
      \documentclass[12pt,a4paper]{article}
      \pagestyle{empty}
      \usepackage[utf8]{inputenc}
      \usepackage[parfill]{parskip}
      \usepackage[margin=1cm]{geometry}
      \usepackage{hyperref, graphicx, enumitem, titlesec, fontspec}
      \usepackage[sfdefault]{inter}
      \begin{document}
      \titleformat{\section}{\large\bf}{}{0cm}{}[\titlerule\vspace{-0.5em}]
    ");

    let postamble = indoc!(r"
      \end{document}
    ");

    formatdoc!(r"
      {preamble}
      {contact}
      {education}
      {experience}
      {projects}
      {skills}
      {postamble}
    ")
  } 
}

impl Contact {
  fn to_tex(&self) -> String {
    let name = tex_sanitize(&self.name.trim());
    let email = tex_sanitize(&self.email.trim());
    let github = tex_sanitize(&self.github.trim());
    formatdoc!(r"
      \begin{{center}}
      {{\Huge\textbf{{{name}}}}} \\[.8em]
      {email} \hspace{{2em}} github.com/{github}
      \end{{center}}")
  }
}

impl Educations {
  fn to_tex(&self) -> String {
    tex_section("Education", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Education {
  fn to_tex(&self) -> String {
    let program = tex_sanitize(&self.program.trim());
    let institution = tex_sanitize(&self.institution.trim());
    let graduation = tex_sanitize(&self.graduation.trim());
    let description = tex_sanitize_vec(&self.description);
    tex_role(&program, &institution, &graduation, &description)
  }
}

impl Experiences {
  fn to_tex(&self) -> String {
    tex_section("Experience", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Experience {
  fn to_tex(&self) -> String {
    let begin = tex_sanitize(&self.begin.trim());
    let end = tex_sanitize(&self.end.trim());
    let endash = r"--";
    let time = format!("{begin} {endash} {end}");
    let position = tex_sanitize(&self.position.trim());
    let company = tex_sanitize(&self.company.trim());
    let description = tex_sanitize_vec(&self.description);
    tex_role(&position, &company, &time, &description)
  }
}

impl Projects {
  fn to_tex(&self) -> String {
    tex_section("Projects", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Project {
  fn to_tex(&self) -> String {
    let title = tex_sanitize(&self.title.trim());
    let category = tex_sanitize(&self.category.trim());
    let github = tex_sanitize(&self.github.trim());
    let description = tex_sanitize_vec(&self.description);
    tex_role(&title, &category, &github, &description)
  }
}

impl Skills {
  fn to_tex(&self) -> String {
    tex_section("Skills", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Skill {
  fn to_tex(&self) -> String {
    let area = tex_sanitize(&self.area.trim());
    let description = tex_sanitize(&self.description.trim());
    formatdoc!(r"\textbf{{{area}}}:\ {{{description}}}")
  }
}

fn tex_section(name: &str, items: Vec<String>) -> String {
  let name = tex_sanitize(&name);
  let leftmargin = r"0cm";
  let itemsep = r"-0.0em";
  let label = r"{}";
  let items = tex_itemize(items, leftmargin, itemsep, label);
  formatdoc!(r"

    \section{{{name}}}
    {items}")
}

fn tex_role(t1: &str, t2: &str, t3: &str, items: &[String]) -> String {
  let leftmargin = r"*";
  let itemsep = r"-0.7em";
  let label = r"\textbullet";
  let items = tex_itemize(items.to_vec(), leftmargin, itemsep, label);
  formatdoc!(r"
    \textbf{{{t1}}}, {t2} \hfill {t3}
    {items}")
}

fn tex_itemize(mut items: Vec<String>, leftmargin: &str, itemsep: &str, label: &str) -> String {
  for s in items.iter_mut() {
    s.insert_str(0, r"\item ");
  }
  let items = items.join("\n");
  formatdoc!(r"
    \begin{{itemize}}[leftmargin={leftmargin}, topsep=-2em, itemsep={itemsep}, label={label}]
    {items}
    \end{{itemize}}")
}

fn tex_sanitize(input: &str) -> String {
  let mut result = String::from(input);
  let backslash_regex = Regex::new(r#"\\"#).unwrap();
  let tilde_regex = Regex::new(r#"\~"#).unwrap();
  let caret_regex = Regex::new(r#"\^"#).unwrap();
  let special_chars_regex = Regex::new(r#"([#%&_^{}~])"#).unwrap();
  let dollar_regex = Regex::new(r#"\$"#).unwrap();
  result = backslash_regex.replace_all(&result, r"\textbackslash ").into();
  result = tilde_regex.replace_all(&result, r"\textasciitilde").into();
  result = caret_regex.replace_all(&result, r"\textasciicircum").into();
  result = special_chars_regex.replace_all(&result, |caps: &regex::Captures| {
    format!("\\{}", &caps[1])
  }).into();
  result = dollar_regex.replace_all(&result, r"\$").into();
  result.trim().into()
}

fn tex_sanitize_vec(input: &[String]) -> Vec<String> {
  input.iter().map(|s| tex_sanitize(&s.trim())).collect::<Vec<_>>()
}

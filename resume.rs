#!/usr/bin/env -S cargo +nightly -Zunstable-options -Zscript

//! ```cargo
//! cargo-features = ["edition2024"]
//! package.edition = "2024"
//! [dependencies]
//! serde = {version = "1.0", features = ["derive"]}
//! serde_yaml = "0.9"
//! tectonic = "0.14"
//! indoc = "2.0.4"
//! regex = "1.10"
//! ```

use serde::Deserialize;
use indoc::formatdoc;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let src = format!("resume.yaml");
  let dst = format!("resume.pdf");
  let resume = serde_yaml::from_str::<Resume>(&std::fs::read_to_string(src)?)?;
  let pdf = resume.to_pdf();
  std::fs::write(dst, &pdf)?;
  Ok(())
}

#[derive(Deserialize)]
struct Resume {
  contact: Contact,
  education: Vec<Education>,
  experience: Vec<Experience>,
  projects: Vec<Project>,
  skillset: Vec<Skillset>,
}

impl Resume {
  pub fn to_pdf(&self) -> Vec<u8> {
    tectonic::latex_to_pdf(&self.to_string()).expect("processing resume to pdf failed")
  }
}

impl ToString for Resume {
  fn to_string(&self) -> String {
    let contact = &self.contact.to_string();
    let experience = tex_section("Experience", self.experience.iter().map(ToString::to_string).collect());
    let education = tex_section("Education", self.education.iter().map(ToString::to_string).collect());
    let projects = tex_section("Projects", self.projects.iter().map(ToString::to_string).collect());
    let skillset = tex_section("Skillset", self.skillset.iter().map(ToString::to_string).collect());
    formatdoc!(r"
      \documentclass[11pt,a4paper]{{article}}
      \pagestyle{{empty}}
      \usepackage[utf8]{{inputenc}}
      \usepackage[parfill]{{parskip}}
      \usepackage[margin=1cm, top=2cm]{{geometry}}
      \usepackage{{fontawesome, hyperref, fontspec, graphicx}}
      \usepackage[sfdefault]{{inter}}
      \begin{{document}}
        {contact} \\[2.5em]
        {experience} \\[2.5em]
        {education} \\[2.5em]
        {projects} \\[2.5em]
        {skillset}
      \end{{document}}
    ")
  } 
}

#[derive(Deserialize)]
struct Contact {
  name: String,
  email: String,
  github: String,
}

impl ToString for Contact {
  fn to_string(&self) -> String {
    let name = &self.name;
    let email = &self.email;
    let github = &self.github;
    formatdoc!(r"
      \begin{{minipage}}[t]{{10\textwidth}}
        {{\bf\LARGE\MakeUppercase{{{name}}}}}\qquad\ {email}\qquad github.com/{github}
      \end{{minipage}}
    ")
  }
}

#[derive(Deserialize)]
struct Education {
  program: String,
  institution: String,
  location: String,
  time: String,
  description: String,
}

impl ToString for Education {
  fn to_string(&self) -> String {
    tex_role(
      &self.program,
      &self.institution,
      &self.time,
      &self.description,
    )
  }
}

#[derive(Deserialize)]
struct Experience {
  position: String,
  company: String,
  location: String,
  time: String,
  description: String,
}

impl ToString for Experience {
  fn to_string(&self) -> String {
    tex_role(
      &self.position,
      &self.company,
      &self.time,
      &self.description,
    )
  }
}

#[derive(Deserialize)]
struct Project {
  title: String,
  github: String,
  description: String,
}

impl ToString for Project {
  fn to_string(&self) -> String {
    let title = &self.title;
    let github = &self.github;
    let description = &self.description;
    formatdoc!(r"\textbf{{{title}}}{{{description}}}")
  }
}

#[derive(Deserialize)]
struct Skillset {
  area: String,
  description: String,
}

impl ToString for Skillset {
  fn to_string(&self) -> String {
    let area = &self.area;
    let description = &self.description;
    formatdoc!(r"\textbf{{{area}}}{{{description}}}")
  }
}

fn tex_role(a: &str, b: &str, c: &str, d: &str) -> String {
  let a = sanitize_tex(&a);
  let b = sanitize_tex(&b);
  let c = sanitize_tex(&c);
  let d = sanitize_tex(&d);
  formatdoc!(r"\textbf{{{a}}}, {b} \hfill {c} \\ {d}")
}

fn tex_section(name: &str, items: Vec<String>) -> String {
  let name = sanitize_tex(&name);
  // let items = items.iter().map(|s| sanitize_tex(&s)).collect::<Vec<_>>();
  let items = items.join(r"\\[.4em]");
  formatdoc!(r"
    \MakeUppercase{{\bf{{ {name} }} }} \\[-.7em]
    \rule{{\textwidth}}{{.15ex}} \\[.1em]
    {items}")
}

fn sanitize_tex(input: &str) -> String {
  let mut result = String::from(input);
  let backslash_regex = Regex::new(r#"\\"#).unwrap();
  let tilde_regex = Regex::new(r#"\~"#).unwrap();
  let caret_regex = Regex::new(r#"\^"#).unwrap();
  let special_chars_regex = Regex::new(r#"([#%&_^{}~])"#).unwrap();
  let dollar_regex = Regex::new(r#"\$"#).unwrap();
  result = backslash_regex.replace_all(&result, r"\textbackslash ").to_string();
  result = tilde_regex.replace_all(&result, r"\textasciitilde").to_string();
  result = caret_regex.replace_all(&result, r"\textasciicircum").to_string();
  result = special_chars_regex.replace_all(&result, |caps: &regex::Captures| {
    format!("\\{}", &caps[1])
  }).into();
  result = dollar_regex.replace_all(&result, r"\$").to_string();
  result
}

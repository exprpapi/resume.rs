#!/usr/bin/env -S cargo +nightly -Zunstable-options -Zscript

//! ```cargo
//! cargo-features = ["edition2024"]
//! package.edition = "2024"
//! [dependencies]
//! indoc = "2.0.4"
//! regex = "1.10"
//! serde = {version = "1.0", features = ["derive"]}
//! serde_yaml = "0.9"
//! tectonic = "0.14"
//! ```

use indoc::{formatdoc, indoc};
use regex::Regex;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let src = format!("resume.yaml");
  let dst = format!("resume.pdf");
  let resume = serde_yaml::from_str::<Resume>(&std::fs::read_to_string(src)?)?;
  let tex = resume.to_tex();
  std::fs::write("resume.tex", &tex)?;
  let pdf = Resume::to_pdf(&tex);
  std::fs::write(dst, &pdf)?;
  Ok(())
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
  time: String,
  description: Vec<String>,
}

#[derive(Deserialize)]
struct Experiences(Vec<Experience>);

#[derive(Deserialize)]
struct Experience {
  position: String,
  company: String,
  time: String,
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
  fn to_pdf(tex: &String) -> Vec<u8> {
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
      \usepackage[sfdefault]{inter}
      \usepackage{hyperref, graphicx, enumitem, titlesec}
      \begin{document}
      \titleformat{\section}
        {\vspace{0cm}\large\bf}
        {}
        {0cm}
        {\MakeUppercase}
        [\vspace{-0.2em}\titlerule\vspace{-0.5em}]
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
      {postamble}")
  } 
}

impl Contact {
  fn to_tex(&self) -> String {
    let name = &self.name;
    let email = &self.email;
    let github = &self.github;
    formatdoc!(r"
      \begin{{center}}
      {{\bf\LARGE\MakeUppercase{{{name}}}}} \\[.8em]
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
    tex_role(
      &self.program,
      &self.institution,
      &self.time,
      self.description.clone(),
    )
  }
}

impl Experiences {
  fn to_tex(&self) -> String {
    tex_section("Experience", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Experience {
  fn to_tex(&self) -> String {
    tex_role(
      &self.position,
      &self.company,
      &self.time,
      self.description.clone(),
    )
  }
}

impl Projects {
  fn to_tex(&self) -> String {
    tex_section("Projects", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Project {
  fn to_tex(&self) -> String {
    tex_role(
      &self.title,
      &self.category,
      &self.github,
      self.description.clone(),
    )
  }
}

impl Skills {
  fn to_tex(&self) -> String {
    tex_section("Skills", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Skill {
  fn to_tex(&self) -> String {
    let area = &self.area;
    let description = &self.description.trim();
    formatdoc!(r"\textbf{{{area}}}:\ {{{description}}}")
  }
}

fn tex_section(name: &str, items: Vec<String>) -> String {
  let name = tex_sanitize(&name);
  let leftmargin = r"0cm";
  let itemsep = r"-0.3em";
  let label = r"{}";
  let items = tex_itemize(items, leftmargin, itemsep, label);
  formatdoc!(r"

    \section{{{name}}}
    {items}")
}

fn tex_role(t1: &str, t2: &str, t3: &str, items: Vec<String>) -> String {
  let t1 = tex_sanitize(&t1);
  let t2 = tex_sanitize(&t2);
  let t3 = tex_sanitize(&t3);
  let items = items.iter().map(|s| tex_sanitize(&s)).collect::<Vec<_>>();
  let leftmargin = r"*";
  let itemsep = r"-0.5em";
  let label = r"\textbullet";
  let items = tex_itemize(items, leftmargin, itemsep, label);
  formatdoc!(r"
    \textbf{{{t1}}}, {t2} \hfill {t3}
    {items}")
}

fn tex_itemize(mut items: Vec<String>, leftmargin: &str, itemsep: &str, label: &str) -> String {
  for s in items.iter_mut() {
    s.insert_str(0, r"\item ");
  }
  let items = items.join("\n");
  let itemize = formatdoc!(r"
    \begin{{itemize}}[leftmargin={leftmargin}, topsep=-0.5em, itemsep={itemsep}, label={label}]
    {items}
    \end{{itemize}}");
  itemize
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

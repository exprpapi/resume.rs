use indoc::{formatdoc, indoc};
use serde::Deserialize;
use crate::tex;

#[derive(Deserialize)]
pub struct Resume {
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
  pub fn to_tex(&self) -> String {
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
    let name = tex::sanitize(self.name.trim());
    let email = tex::sanitize(self.email.trim());
    let github = tex::sanitize(self.github.trim());
    formatdoc!(r"
      \begin{{center}}
      {{\Huge\textbf{{{name}}}}} \\[.8em]
      {email} \hspace{{2em}} github.com/{github}
      \end{{center}}")
  }
}

impl Educations {
  fn to_tex(&self) -> String {
    tex::section("Education", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Education {
  fn to_tex(&self) -> String {
    let program = tex::sanitize(self.program.trim());
    let institution = tex::sanitize(self.institution.trim());
    let graduation = tex::sanitize(self.graduation.trim());
    let description = tex::sanitize_vec(&self.description);
    tex::role(&program, &institution, &graduation, &description)
  }
}

impl Experiences {
  fn to_tex(&self) -> String {
    tex::section("Experience", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Experience {
  fn to_tex(&self) -> String {
    let begin = tex::sanitize(self.begin.trim());
    let end = tex::sanitize(self.end.trim());
    let endash = r"--";
    let time = format!("{begin} {endash} {end}");
    let position = tex::sanitize(self.position.trim());
    let company = tex::sanitize(self.company.trim());
    let description = tex::sanitize_vec(&self.description);
    tex::role(&position, &company, &time, &description)
  }
}

impl Projects {
  fn to_tex(&self) -> String {
    tex::section("Projects", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Project {
  fn to_tex(&self) -> String {
    let title = tex::sanitize(self.title.trim());
    let category = tex::sanitize(self.category.trim());
    let github = tex::sanitize(self.github.trim());
    let description = tex::sanitize_vec(&self.description);
    tex::role(&title, &category, &github, &description)
  }
}

impl Skills {
  fn to_tex(&self) -> String {
    tex::section("Skills", self.0.iter().map(|s| s.to_tex()).collect())
  }
}

impl Skill {
  fn to_tex(&self) -> String {
    let area = tex::sanitize(self.area.trim());
    let description = tex::sanitize(self.description.trim());
    formatdoc!(r"\textbf{{{area}}}:\ {{{description}}}")
  }
}

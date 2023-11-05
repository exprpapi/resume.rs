use indoc::formatdoc;
use regex::Regex;

pub fn section(name: &str, items: Vec<String>) -> String {
  let name = sanitize(name);
  let leftmargin = r"0cm";
  let itemsep = r"-0.0em";
  let label = r"{}";
  let items = itemize(items, leftmargin, itemsep, label);
  formatdoc!(r"

    \section{{{name}}}
    {items}")
}

pub fn role(t1: &str, t2: &str, t3: &str, items: &[String]) -> String {
  let leftmargin = r"*";
  let itemsep = r"-0.7em";
  let label = r"\textbullet";
  let items = itemize(items.to_vec(), leftmargin, itemsep, label);
  formatdoc!(r"
    \textbf{{{t1}}}, {t2} \hfill {t3}
    {items}")
}

pub fn itemize(mut items: Vec<String>, leftmargin: &str, itemsep: &str, label: &str) -> String {
  for s in items.iter_mut() {
    s.insert_str(0, r"\item ");
  }
  let items = items.join("\n");
  formatdoc!(r"
    \begin{{itemize}}[leftmargin={leftmargin}, topsep=-2em, itemsep={itemsep}, label={label}]
    {items}
    \end{{itemize}}")
}

pub fn sanitize(input: &str) -> String {
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

pub fn sanitize_vec(input: &[String]) -> Vec<String> {
  input.iter().map(|s| sanitize(s.trim())).collect::<Vec<_>>()
}

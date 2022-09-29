use crate::types::basic_types;

pub struct Snippet {
    name: String,
    prefix: String,
    body: Vec<String>,
}

impl Snippet {
    pub fn new(name: impl ToString, prefix: impl ToString, body: Vec<impl ToString>) -> Self {
        Snippet {
            name: name.to_string(),
            prefix: prefix.to_string(),
            body: body.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn json(&self) -> String {
        let mut body = self.body.clone();

        if !(body
            .iter()
            .map(|s| s.contains("$0"))
            .reduce(|f, e| f || e)
            .unwrap())
        {
            body.push("$0".to_string());
        }

        format!(
            r#""{}":{{"prefix":"{}","body":[{}]}}"#,
            self.name,
            self.prefix,
            body.iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Default)]
pub struct SnippetManager {
    snippets: Vec<Snippet>,
}

fn gen_spacers(min: usize, max: usize) -> Vec<String> {
    (min..=max)
        .into_iter()
        .map(|n| format!("${}", n))
        .collect::<Vec<String>>()
}

impl SnippetManager {
    pub fn gen(&mut self, cin_max_len: usize, cout_max_len: usize) {
        let basic_types = basic_types();

        // 標準入力から基本型受け取る系
        for ty in &basic_types {
            for i in 1..=cin_max_len {
                let name = format!("cin_{}_{}", ty, i);
                let prefix = format!(
                    "cin{}{}",
                    ty,
                    if i > 1 { i.to_string() } else { String::new() }
                );
                let spacers = gen_spacers(1, i);
                let var_statement = format!("{} {};", ty, spacers.join(", "));
                let cin_statement = format!("cin >> {};", spacers.join(" >> "));
                let snippet = Snippet::new(name, prefix, vec![var_statement, cin_statement]);

                self.snippets.push(snippet);
            }
        }

        // 標準入力からvector受け取る系
        for ty in &basic_types {
            for i in 1..=cin_max_len {
                let name = format!("cin_vec_{}_{}", ty, i);
                let prefix = format!(
                    "cinv{}{}",
                    ty,
                    if i > 1 { i.to_string() } else { String::new() }
                );
                let spacers = gen_spacers(1, i);
                let var_statement = format!(
                    "vector<{}> {};",
                    ty,
                    spacers
                        .iter()
                        .map(|s| format!("{}(${})", s, i + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                let cin_statement = format!(
                    "rep(i, ${}) cin >> {};",
                    i + 1,
                    spacers
                        .iter()
                        .map(|s| format!("{}[i]", s))
                        .collect::<Vec<String>>()
                        .join(" >> ")
                );
                let snippet = Snippet::new(name, prefix, vec![var_statement, cin_statement]);

                self.snippets.push(snippet);
            }
        }

        // 標準入力からvecvec受け取る系
        for ty in &basic_types {
            let name = format!("cin_vec_vec_{}", ty);
            let prefix = format!("cinvv{}", ty,);
            let var_statement = format!("vector<vector<{}>> $1($2, vector<{}>($3));", ty, ty);
            let cin_statement = format!("rep(i, $2) rep(j, $3) cin >> $1[i][j];",);
            let snippet = Snippet::new(name, prefix, vec![var_statement, cin_statement]);

            self.snippets.push(snippet);
        }

        // 標準出力
        for i in 1..=cout_max_len {
            let name = format!("cout_{}", i);
            let prefix = format!("co{}", if i > 1 { i.to_string() } else { String::new() });
            let spacers = gen_spacers(1, i);
            let cout_statement = format!(
                r#"cout{} << \"\\n\";"#,
                spacers
                    .iter()
                    .map(|s| format!(" << {}", s))
                    .collect::<Vec<String>>()
                    .join("")
            );
            let snippet = Snippet::new(name, prefix, vec![cout_statement]);

            self.snippets.push(snippet);
        }

        // スペース区切りの標準出力
        for i in 1..=cout_max_len {
            let name = format!("cout_{}_with_spaces", i);
            let prefix = format!("cs{}", if i > 1 { i.to_string() } else { String::new() });
            let spacers = gen_spacers(1, i);
            let cout_statement = format!(
                r#"cout << {}{} << \"\\n\";"#,
                spacers.iter().nth(0).unwrap(),
                spacers
                    .iter()
                    .skip(1)
                    .map(|s| format!(r#" << \" \" << {}"#, s))
                    .collect::<Vec<String>>()
                    .join("")
            );
            let snippet = Snippet::new(name, prefix, vec![cout_statement]);

            self.snippets.push(snippet);
        }

        // yesno
        self.snippets.push(Snippet::new(
            "yesno",
            "yesno",
            vec![
                r#"if ($1) cout << \"${2:Yes}\" << \"\\n\";"#,
                r#"else cout << \"${3:No}\" << \"\\n\";"#,
            ],
        ));

        // grid_vector
        self.snippets.push(Snippet::new(
            "grid_vector",
            "gridvector",
            vec![
                "const ll dx[4] = {1, 0, -1, 0};",
                "const ll dy[4] = {0, -1, 0, 1};",
            ],
        ));

        // grid_next
        self.snippets.push(Snippet::new(
            "grid_next",
            "gridnext",
            vec![
                "rep(${1:x}, ${2:W}) rep(${3:y}, ${4:H}) rep(i, 4) {",
                "  ll nx = ${1:x} + dx[i], ny = ${2:y} + dy[i];",
                "",
                "  if (nx < 0 || nx >= ${3:W} || ny < 0 || ny >= ${4:H}) continue;",
                "  $0",
                "}",
            ],
        ))
    }

    pub fn json(&self) -> String {
        format!(
            "{{{}}}",
            self.snippets
                .iter()
                .map(|s| s.json())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

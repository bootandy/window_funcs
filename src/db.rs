use deadpool_postgres::Client;

macro_rules! regex {
    ($e:expr) => {
        regex::Regex::new($e).unwrap()
    };
}

fn _format_type<T: ToString>(t: Option<T>) -> String {
    match t {
        None => "Null".to_string(),
        Some(x) => x.to_string(),
    }
}

pub async fn run_sql(client: &Client, sql_command: &str) -> Vec<Vec<String>> {
    let mut result = vec![];
    let query_result = client.query(sql_command, &[]).await;

    match query_result {
        Ok(query_results) => {
            if let Some(row) = query_results.first() {
                let cols = row.columns().iter();
                result.push(cols.map(|c| c.name().to_string()).collect());

                for row in &query_results {
                    let result_row = row
                        .columns()
                        .iter()
                        .enumerate()
                        .map(|(i, col)| match col.type_().name() {
                            "int8" => {
                                let temp: Option<i64> = row.get(i);
                                _format_type(temp)
                            }
                            "int4" => {
                                let temp: Option<i32> = row.get(i);
                                _format_type(temp)
                            }
                            "float8" => {
                                let temp: Option<f64> = row.get(i);
                                match temp {
                                    None => "Null".to_string(),
                                    Some(x) => format!("{:.1}", x),
                                }
                            }
                            "varchar" | "text" => {
                                let temp: Option<String> = row.get(i);
                                _format_type(temp)
                            }
                            "_varchar" => {
                                let temp: Option<Vec<String>> = row.get(i);
                                match temp {
                                    None => "Null".to_string(),
                                    Some(x) => x.join(","),
                                }
                            }
                            x => {
                                println!("Got unknown type: {:?}", x);
                                format!("Add conversion for {:?}", x)
                            }
                        })
                        .collect();
                    result.push(result_row);
                }
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
            result.push(vec![error.to_string()])
        }
    }
    result
}

pub async fn verify_then_run_sql(client: &Client, s: &str) -> Vec<Vec<String>> {
    if regex!(r###"[^\w]pg_"###).is_match(s) {
        vec![vec!["Do not use pg_".into()]]
    } else if regex!(r###"[^\w]statement_timeout"###).is_match(s) {
        vec![vec!["Do not use statement_timeout".into()]]
    } else if regex!(r###"[^\w]version[^\w]"###).is_match(s) {
        vec![vec!["Do not use version".into()]]
    } else {
        run_sql(client, s).await
    }
}

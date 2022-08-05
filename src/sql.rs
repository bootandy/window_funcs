use std::collections::HashMap;

static INTRO_0_SQL: &str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12 order by age";
static INTRO_0_HELP: &str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-group-by-transact-sql#d-use-a-group-by-clause-with-a-having-clause";

static OVER_0_SQL: &str = "select name, sum(weight)
over (order by name) as running_total_weight
from cats order by name";
static OVER_0_HELP: &str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static OVER_1_SQL: &str = "
select name, breed,
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";
static OVER_1_HELP: &str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static OVER_2_SQL: &str = "
select name, weight,
avg(weight) over (order by weight ROWS between 1 preceding and 1 following) as average_weight
from cats order by weight";
static OVER_2_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#arguments";

static OVER_3_SQL: &str = "
select name,
sum(weight) over (order by weight DESC ROWS between unbounded preceding and current row) as running_total_weight
from cats order by running_total_weight";
static OVER_3_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#arguments";

static RANKINGS_0_SQL: &str = "
select row_number() over (order by color,name) as unique_number,
name, color
from cats";
static RANKINGS_0_HELP: &str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/row-number-transact-sql#b-returning-the-row-number-for-salespeople";

static RANKINGS_1_SQL: &str = "
select
rank() over (order by weight desc) as ranking,
weight, name
 from cats
 order by ranking, name";
static RANKINGS_1_HELP: &str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/rank-transact-sql#a-ranking-rows-within-a-partition";

static RANKINGS_2_SQL: &str = "
select
dense_rank() over (order by age DESC) as r, name,age
 from cats order by r, name";
static RANKINGS_2_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/dense-rank-transact-sql#examples";

static RANKINGS_3_SQL: &str = "
select name, weight,
percent_rank() over (order by weight) * 100 as percent
from cats order by weight";
static RANKINGS_3_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/percent-rank-transact-sql";

static RANKINGS_4_SQL: &str = "
select name, weight,
cast(cume_dist() over (order by weight) * 100 as integer) as percent
from cats order by weight";
static RANKINGS_4_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/cume-dist-transact-sql";

static GROUPINGS_0_SQL: &str = "
select
 name, weight, ntile(4) over ( order by weight) as weight_quartile
from  cats
order by weight
";
static GROUPINGS_0_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/ntile-transact-sql#examples";

static GROUPINGS_1_SQL: &str = "
select name, weight,
   coalesce(weight - lag(weight, 1) over (order by weight), 0) as weight_to_lose
FROM cats order by weight";
static GROUPINGS_1_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static GROUPINGS_2_SQL: &str = "
    select name, breed, weight,
coalesce(weight - lag(weight, 1) over (partition by breed order by weight), 0) as weight_to_lose
from cats order by weight, name ";
static GROUPINGS_2_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static GROUPINGS_3_SQL: &str = "
select name, color,
first_value(weight) over (partition by color order by weight) as weight_by_color
from cats order by color, name";
static GROUPINGS_3_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/first-value-transact-sql#examples";

static GROUPINGS_4_SQL: &str = "
select name, weight, breed,
coalesce(cast(lead(weight, 1) over (partition by breed order by weight) as varchar), 'fattest cat') as next_heaviest
from cats order by weight";
static GROUPINGS_4_HELP: &str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lead-transact-sql#examples";

static GROUPINGS_5_SQL: &str = "
select name, weight,
coalesce(nth_value(weight, 4) over (order by weight), 99.9) as imagined_weight
from cats order by weight";
static GROUPINGS_5_HELP: &str =
    "https://docs.oracle.com/cloud/latest/db112/SQLRF/functions114.htm#SQLRF30031";

static GROUPINGS_6_SQL: &str = "
select distinct(breed),
nth_value(weight, 2) over (
    partition by breed order by weight RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING
) as imagined_weight
from cats order by breed;";
static GROUPINGS_6_HELP: &str =
    "https://docs.oracle.com/cloud/latest/db112/SQLRF/functions114.htm#SQLRF30031";

static OTHER_0_SQL: &str = "
select name, weight,
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              window ntile_window AS
                       ( ORDER BY weight)
     order by weight, name";
static OTHER_0_HELP: &str = "http://dcx.sap.com/1200/en/dbreference/window-statement.html";

static OTHER_1_SQL: &str = "
select color,
array_agg(name) as names
from cats group by color
order by color desc";
static OTHER_1_HELP: &str = "https://lorenstewart.me/2017/12/03/postgresqls-array_agg-function/";

static OTHER_2_SQL: &str = "
select breed,
avg(weight) as average_weight,
avg(weight) filter (where age > 1) average_old_weight
from cats group by breed order by breed";
static OTHER_2_HELP: &str = "https://modern-sql.com/feature/filter";

static INTRO_0_TITLE: &str = "Refresher on Aggregates";
static OVER_0_TITLE: &str = "Running Totals";
static OVER_1_TITLE: &str = "Partitioned Running Totals";
static OVER_2_TITLE: &str = "Examining nearby rows";
static OVER_3_TITLE: &str = "Correct Running Total";
static RANKINGS_0_TITLE: &str = "Unique Numbers";
static RANKINGS_1_TITLE: &str = "Ordering";
static RANKINGS_2_TITLE: &str = "Further Ordering";
static RANKINGS_3_TITLE: &str = "Percentages";
static RANKINGS_4_TITLE: &str = "Percentiles";
static GROUPINGS_0_TITLE: &str = "Quartiles";
static GROUPINGS_1_TITLE: &str = "Compare to Row";
static GROUPINGS_2_TITLE: &str = "Compare to Row Part 2";
static GROUPINGS_3_TITLE: &str = "First of each Group";
static GROUPINGS_4_TITLE: &str = "More Row Comparisons";
static GROUPINGS_5_TITLE: &str = "Special Case Grouping";
static GROUPINGS_6_TITLE: &str = "More Grouping";
static OTHER_0_TITLE: &str = "Using Window Clause";
static OTHER_1_TITLE: &str = "Aggregating data";
static OTHER_2_TITLE: &str = "Limiting Large Results";

// Concept:
// pub struct QuestionData {
//     sql: &'static str,
//     help: &'static str,
//     title: &'static str,
//     keywords: Vec<&'static str>,
// }

pub fn get_question_data_map(
) -> HashMap<&'static str, Vec<(&'static str, &'static str, &'static str, Vec<&'static str>)>> {
    HashMap::from([
        (
            "intro",
            vec![(INTRO_0_SQL, INTRO_0_HELP, INTRO_0_TITLE, vec!["group by"])],
        ),
        (
            "over",
            vec![
                (OVER_0_SQL, OVER_0_HELP, OVER_0_TITLE, vec!["over"]),
                (OVER_1_SQL, OVER_1_HELP, OVER_1_TITLE, vec!["partition by"]),
                (
                    OVER_2_SQL,
                    OVER_2_HELP,
                    OVER_2_TITLE,
                    vec!["preceding", "following"],
                ),
                (
                    OVER_3_SQL,
                    OVER_3_HELP,
                    OVER_3_TITLE,
                    vec!["unbounded preceding"],
                ),
            ],
        ),
        (
            "ranking",
            vec![
                (
                    RANKINGS_0_SQL,
                    RANKINGS_0_HELP,
                    RANKINGS_0_TITLE,
                    vec!["row_number"],
                ),
                (
                    RANKINGS_1_SQL,
                    RANKINGS_1_HELP,
                    RANKINGS_1_TITLE,
                    vec!["rank"],
                ),
                (
                    RANKINGS_2_SQL,
                    RANKINGS_2_HELP,
                    RANKINGS_2_TITLE,
                    vec!["dense_rank"],
                ),
                (
                    RANKINGS_3_SQL,
                    RANKINGS_3_HELP,
                    RANKINGS_3_TITLE,
                    vec!["percent_rank"],
                ),
                (
                    RANKINGS_4_SQL,
                    RANKINGS_4_HELP,
                    RANKINGS_4_TITLE,
                    vec!["cume_dist"],
                ),
            ],
        ),
        (
            "grouping",
            vec![
                (
                    GROUPINGS_0_SQL,
                    GROUPINGS_0_HELP,
                    GROUPINGS_0_TITLE,
                    vec!["ntile"],
                ),
                (
                    GROUPINGS_1_SQL,
                    GROUPINGS_1_HELP,
                    GROUPINGS_1_TITLE,
                    vec!["lag", "lead", "min"],
                ),
                (
                    GROUPINGS_2_SQL,
                    GROUPINGS_2_HELP,
                    GROUPINGS_2_TITLE,
                    vec!["lag", "lead", "min"],
                ),
                (
                    GROUPINGS_3_SQL,
                    GROUPINGS_3_HELP,
                    GROUPINGS_3_TITLE,
                    vec!["first_value", "nth_value", "min"],
                ),
                (
                    GROUPINGS_4_SQL,
                    GROUPINGS_4_HELP,
                    GROUPINGS_4_TITLE,
                    vec!["lead"],
                ),
                (
                    GROUPINGS_5_SQL,
                    GROUPINGS_5_HELP,
                    GROUPINGS_5_TITLE,
                    vec!["nth_value"],
                ),
                (
                    GROUPINGS_6_SQL,
                    GROUPINGS_6_HELP,
                    GROUPINGS_6_TITLE,
                    vec!["nth_value"],
                ),
            ],
        ),
        (
            "other",
            vec![
                (OTHER_0_SQL, OTHER_0_HELP, OTHER_0_TITLE, vec!["window"]),
                (OTHER_1_SQL, OTHER_1_HELP, OTHER_1_TITLE, vec!["array_agg"]),
                (OTHER_2_SQL, OTHER_2_HELP, OTHER_2_TITLE, vec!["filter"]),
            ],
        ),
    ])
}

static CATEGORY_ORDER: &[&str] = &["intro", "over", "ranking", "grouping", "other"];

pub fn get_sql_for_q<'a>(
    data_mp: &'a HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>,
    category: &str,
    number: &str,
) -> Option<&'a (&'a str, &'a str, &'a str, Vec<&'a str>)> {
    let questions_by_type = data_mp.get(category);
    match questions_by_type {
        Some(q_list) => {
            let tmp = number.parse::<usize>().unwrap_or(0);
            if tmp >= q_list.len() {
                None
            } else {
                Some(&q_list[tmp])
            }
        }
        None => None,
    }
}

pub fn get_next_page(
    data_mp: &HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>,
    category: &str,
    sid: &str,
) -> String {
    let id = (sid.parse::<i32>().unwrap_or(-1) + 1).to_string();

    if get_sql_for_q(data_mp, category, &id).is_some() {
        format!("{}/{}", category, id)
    } else {
        let idx = CATEGORY_ORDER.iter().position(|&a| a == category);
        match idx {
            Some(i) => {
                // Handles case of the last page
                if i + 1 >= CATEGORY_ORDER.len() {
                    "".to_string()
                } else {
                    CATEGORY_ORDER.get(i + 1).unwrap_or(&"intro").to_string() + "/"
                }
            }
            None => CATEGORY_ORDER[0].to_string() + "/",
        }
    }
}

pub fn get_prev_page(
    data_mp: &HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>,
    category: &str,
    sid: &str,
) -> String {
    let id = sid.parse::<i32>().unwrap_or(-1) - 1;

    // If there was no number -> Then the previous page will be the largest of the
    // previous category
    if id == -2 {
        let idx = CATEGORY_ORDER
            .iter()
            .position(|&a| a == category)
            .unwrap_or(1);
        match idx {
            // Handles the case of the first page
            0 => "".to_owned(),
            _ => {
                let prev_cat = CATEGORY_ORDER[idx - 1];
                let max_id = data_mp.get(prev_cat).unwrap_or(&vec![]).len() - 1;
                prev_cat.to_string() + "/" + &max_id.to_string()
            }
        }
    } else if id > 0 && get_sql_for_q(data_mp, category, &id.to_string()).is_some() {
        format!("{}/{}", category, id)
    // If the number was 0 the previous will be the category instructions page
    } else {
        let c = if CATEGORY_ORDER.contains(&category) {
            category
        } else {
            CATEGORY_ORDER[0]
        };
        c.to_string() + "/"
    }
}

#[test]
fn test_get_prev() {
    let mp = &get_question_data_map();
    assert_eq!(get_prev_page(mp, "grouping", "6"), "grouping/5");
    assert_eq!(get_prev_page(mp, "grouping", ""), "ranking/4");
    assert_eq!(get_prev_page(mp, "grouping", "0"), "grouping/");
    assert_eq!(get_prev_page(mp, "rubbish", "0"), "intro/");
}

#[test]
fn test_get_next() {
    let mp = &get_question_data_map();
    assert_eq!(get_next_page(mp, "grouping", "2"), "grouping/3");
    assert_eq!(get_next_page(mp, "grouping", "6"), "other/");
    assert_eq!(get_next_page(mp, "grouping", ""), "grouping/0");
    assert_eq!(get_next_page(mp, "rubbish", "0"), "intro/");
}

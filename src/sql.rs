static INTRO_0_SQL: &'static str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12 order by age";
static INTRO_0_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-group-by-transact-sql#d-use-a-group-by-clause-with-a-having-clause";

static OVER_0_SQL: &'static str = "select name, sum(weight)
over (order by name) as running_total_weight
from cats order by name";
static OVER_0_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static OVER_1_SQL: &'static str = "
select name, breed,
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";
static OVER_1_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static OVER_2_SQL: &'static str = "
select name, weight,
avg(weight) over (order by weight ROWS between 1 preceding and 1 following) as average_weight
from cats order by weight";
static OVER_2_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#arguments";

static OVER_3_SQL: &'static str = "
select name,
sum(weight) over (order by weight DESC ROWS between unbounded preceding and current row) as running_total_weight
from cats order by running_total_weight";
static OVER_3_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#arguments";

static RANKINGS_0_SQL: &'static str = "
select row_number() over (order by color,name) as unique_number,
name, color
from cats";
static RANKINGS_0_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/row-number-transact-sql#b-returning-the-row-number-for-salespeople";

static RANKINGS_1_SQL: &'static str = "
select
rank() over (order by weight desc) as ranking,
weight, name
 from cats
 order by ranking, name";
static RANKINGS_1_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/rank-transact-sql#a-ranking-rows-within-a-partition";

static RANKINGS_2_SQL: &'static str = "
select
dense_rank() over (order by age DESC) as r, name,age
 from cats order by r, name";
static RANKINGS_2_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/dense-rank-transact-sql#examples";

static RANKINGS_3_SQL: &'static str = "
select name, weight,
percent_rank() over (order by weight) * 100 as percent
from cats order by weight";
static RANKINGS_3_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/percent-rank-transact-sql";

static RANKINGS_4_SQL: &'static str = "
select name, weight,
cast(cume_dist() over (order by weight) * 100 as integer) as percent
from cats order by weight";
static RANKINGS_4_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/cume-dist-transact-sql";

static GROUPINGS_0_SQL: &'static str = "
select
 name, weight, ntile(4) over ( order by weight) as weight_quartile
from  cats
order by weight
";
static GROUPINGS_0_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/ntile-transact-sql#examples";

static GROUPINGS_1_SQL: &'static str = "
select name, weight,
   coalesce(weight - lag(weight, 1) over (order by weight), 0) as weight_to_lose
FROM cats order by weight";
static GROUPINGS_1_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static GROUPINGS_2_SQL: &'static str = "
    select name, breed, weight,
coalesce(weight - lag(weight, 1) over (partition by breed order by weight), 0) as weight_to_lose
from cats order by weight, name ";
static GROUPINGS_2_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static GROUPINGS_3_SQL: &'static str = "
select name, color,
first_value(weight) over (partition by color order by weight) as weight_by_color
from cats order by color, name";
static GROUPINGS_3_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/first-value-transact-sql#examples";

static GROUPINGS_4_SQL: &'static str = "
select name, weight, breed,
coalesce(cast(lead(weight, 1) over (partition by breed order by weight) as varchar), 'fattest cat') as next_heaviest
from cats order by weight";
static GROUPINGS_4_HELP: &'static str =
    "https://docs.microsoft.com/en-us/sql/t-sql/functions/lead-transact-sql#examples";

static GROUPINGS_5_SQL: &'static str = "
select name, weight,
coalesce(nth_value(weight, 4) over (order by weight), 99.9) as imagined_weight
from cats order by weight";
static GROUPINGS_5_HELP: &'static str =
    "https://docs.oracle.com/cloud/latest/db112/SQLRF/functions114.htm#SQLRF30031";

static GROUPINGS_6_SQL: &'static str = "
select distinct(breed),
nth_value(weight, 2) over (
    partition by breed order by weight RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING
) as imagined_weight
from cats order by breed;";
static GROUPINGS_6_HELP: &'static str =
    "https://docs.oracle.com/cloud/latest/db112/SQLRF/functions114.htm#SQLRF30031";

static OTHER_0_SQL: &'static str = "
select name, weight,
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              window ntile_window AS
                       ( ORDER BY weight)
     order by weight, name";
static OTHER_0_HELP: &'static str = "http://dcx.sap.com/1200/en/dbreference/window-statement.html";

static OTHER_1_SQL: &'static str = "
select color,
array_agg(name) as names
from cats group by color
order by color desc";
static OTHER_1_HELP: &'static str =
    "https://lorenstewart.me/2017/12/03/postgresqls-array_agg-function/";

static OTHER_2_SQL: &'static str = "
select breed,
avg(weight) as average_weight,
avg(weight) filter (where age > 1) average_old_weight
from cats group by breed order by breed";
static OTHER_2_HELP: &'static str = "https://modern-sql.com/feature/filter";


static INTRO_0_TITLE: &'static str = "Refresher on Aggregates";
static OVER_0_TITLE: &'static str = "Running Totals";
static OVER_1_TITLE: &'static str = "Partitioned Running Totals";
static OVER_2_TITLE: &'static str = "Examining nearby rows";
static OVER_3_TITLE: &'static str = "Correct Running Total";
static RANKINGS_0_TITLE: &'static str = "Unique Numbers";
static RANKINGS_1_TITLE: &'static str = "Orderring";
static RANKINGS_2_TITLE: &'static str = "Further Orderring";
static RANKINGS_3_TITLE: &'static str = "Percentages";
static RANKINGS_4_TITLE: &'static str = "Percentiles";
static GROUPINGS_0_TITLE: &'static str = "Quartiles";
static GROUPINGS_1_TITLE: &'static str = "Compare to Row";
static GROUPINGS_2_TITLE: &'static str = "Compare to Row Part 2";
static GROUPINGS_3_TITLE: &'static str = "First of each Group";
static GROUPINGS_4_TITLE: &'static str = "More Row Comparisons";
static GROUPINGS_5_TITLE: &'static str = "Special Case Grouping";
static GROUPINGS_6_TITLE: &'static str = "More Grouping";
static OTHER_0_TITLE: &'static str = "Using Window Clause";
static OTHER_1_TITLE: &'static str = "Aggregating data";
static OTHER_2_TITLE: &'static str = "Limiting Large Results";


pub fn get_sql_for_q<'a>(
    folder: &'a str,
    q: &'a str,
) -> Option<(&'a str, &'a str, &'a str, Vec<&'a str>)> {
    match (folder, q) {
        ("intro", "0") => Some((INTRO_0_SQL, INTRO_0_HELP, INTRO_0_TITLE, vec!["group by"])),
        ("over", "0") => Some((OVER_0_SQL, OVER_0_HELP, OVER_0_TITLE, vec!["over"])),
        ("over", "1") => Some((OVER_1_SQL, OVER_1_HELP, OVER_1_TITLE, vec!["partition by"])),
        ("over", "2") => Some((
            OVER_2_SQL,
            OVER_2_HELP,
            OVER_2_TITLE,
            vec!["preceding", "following"],
        )),
        ("over", "3") => Some((
            OVER_3_SQL,
            OVER_3_HELP,
            OVER_3_TITLE,
            vec!["unbounded preceding"],
        )),
        ("ranking", "0") => Some((
            RANKINGS_0_SQL,
            RANKINGS_0_HELP,
            RANKINGS_0_TITLE,
            vec!["row_number"],
        )),
        ("ranking", "1") => Some((
            RANKINGS_1_SQL,
            RANKINGS_1_HELP,
            RANKINGS_1_TITLE,
            vec!["rank"],
        )),
        ("ranking", "2") => Some((
            RANKINGS_2_SQL,
            RANKINGS_2_HELP,
            RANKINGS_2_TITLE,
            vec!["dense_rank"],
        )),
        ("ranking", "3") => Some((
            RANKINGS_3_SQL,
            RANKINGS_3_HELP,
            RANKINGS_3_TITLE,
            vec!["percent_rank"],
        )),
        ("ranking", "4") => Some((
            RANKINGS_4_SQL,
            RANKINGS_4_HELP,
            RANKINGS_4_TITLE,
            vec!["cume_dist"],
        )),
        ("grouping", "0") => Some((
            GROUPINGS_0_SQL,
            GROUPINGS_0_HELP,
            GROUPINGS_0_TITLE,
            vec!["ntile"],
        )),
        ("grouping", "1") => Some((
            GROUPINGS_1_SQL,
            GROUPINGS_1_HELP,
            GROUPINGS_1_TITLE,
            vec!["lag", "lead", "min"],
        )),
        ("grouping", "2") => Some((
            GROUPINGS_2_SQL,
            GROUPINGS_2_HELP,
            GROUPINGS_2_TITLE,
            vec!["lag", "lead", "min"],
        )),
        ("grouping", "3") => Some((
            GROUPINGS_3_SQL,
            GROUPINGS_3_HELP,
            GROUPINGS_3_TITLE,
            vec!["first_value", "nth_value", "min"],
        )),
        ("grouping", "4") => Some((
            GROUPINGS_4_SQL,
            GROUPINGS_4_HELP,
            GROUPINGS_4_TITLE,
            vec!["lead"],
        )),
        ("grouping", "5") => Some((
            GROUPINGS_5_SQL,
            GROUPINGS_5_HELP,
            GROUPINGS_5_TITLE,
            vec!["nth_value"],
        )),
        ("grouping", "6") => Some((
            GROUPINGS_6_SQL,
            GROUPINGS_6_HELP,
            GROUPINGS_6_TITLE,
            vec!["nth_value" ],
        )),
        ("other", "0") => Some((OTHER_0_SQL, OTHER_0_HELP, OTHER_0_TITLE, vec!["window"])),
        ("other", "1") => Some((OTHER_1_SQL, OTHER_1_HELP, OTHER_1_TITLE, vec!["array_agg"])),
        ("other", "2") => Some((OTHER_2_SQL, OTHER_2_HELP, OTHER_2_TITLE, vec!["filter"])),
        (_, _) => None,
    }
}

static CATEGORIES: &'static [&'static str] = &["intro", "over", "ranking", "grouping", "other"];

pub fn get_prev(s: &str) -> &str {
    _get_next_or_prev(s, -1)
}
pub fn get_next(s: &str) -> &str {
    _get_next_or_prev(s, 1)
}
fn _get_next_or_prev(s: &str, index: i32) -> &str {
    match CATEGORIES.iter().position(|&a| a == s) {
        Some(i) => {
            let ii = i as i32 + index;
            if ii < 0 {
                ""
            } else {
                let res = CATEGORIES.get(ii as usize);
                res.unwrap_or_else(|| &CATEGORIES[0])
            }
        }
        None => CATEGORIES[0],
    }
}

pub fn check_category(s: &str) -> &str {
    if CATEGORIES.contains(&s) {
        s
    } else {
        CATEGORIES[0]
    }
}

pub fn get_titles_for(s: &str) -> Vec<&str> {
    match s {
        "intro" => [INTRO_0_TITLE].to_vec(),
        "over" => [OVER_0_TITLE, OVER_1_TITLE, OVER_2_TITLE, OVER_3_TITLE].to_vec(),
        "ranking" => [
            RANKINGS_0_TITLE,
            RANKINGS_1_TITLE,
            RANKINGS_2_TITLE,
            RANKINGS_3_TITLE,
            RANKINGS_4_TITLE,
        ].to_vec(),
        "grouping" => [
            GROUPINGS_0_TITLE,
            GROUPINGS_1_TITLE,
            GROUPINGS_2_TITLE,
            GROUPINGS_3_TITLE,
            GROUPINGS_4_TITLE,
            GROUPINGS_5_TITLE,
        ].to_vec(),
        "other" => [OTHER_0_TITLE, OTHER_1_TITLE, OVER_2_TITLE].to_vec(),
        _ => [].to_vec(),
    }
}

#[test]
fn test_get_prev() {
    assert_eq!(get_prev("grouping"), "ranking");
    assert_eq!(get_prev("rubbish"), "intro");
}

#[test]
fn test_get_next() {
    assert_eq!(get_next("grouping"), "other");
    assert_eq!(get_next("rubbish"), "intro");
}

#[test]
fn test_check_category() {
    assert_eq!(check_category("grouping"), "grouping");
    assert_eq!(check_category("rubbish"), "intro");
}

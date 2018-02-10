static Q0_SQL: &'static str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12 order by age DESC";
static Q0_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-group-by-transact-sql#d-use-a-group-by-clause-with-a-having-clause";

static Q1_SQL: &'static str = "select name, sum(weight)
over (order by name) as running_total_weight
from cats order by name";
static Q1_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static Q2_SQL: &'static str = "
select name, breed,
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";
static Q2_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/queries/select-over-clause-transact-sql#b-using-the-over-clause-with-aggregate-functions";

static Q3_SQL: &'static str = "
select
row_number() over (order by color,name) as unique_number,
name, color
from cats";
static Q3_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/row-number-transact-sql#b-returning-the-row-number-for-salespeople";

static Q4_SQL: &'static str = "
select
rank() over (order by weight desc) as ranking,
weight, name
 from cats
 order by ranking, name DESC";
static Q4_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/rank-transact-sql#a-ranking-rows-within-a-partition";

static Q5_SQL: &'static str = "
select
 name, weight, ntile(4) over ( order by weight) as weight_quartile
from  cats
order by weight_quartile, name
";
static Q5_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/ntile-transact-sql#examples";

static Q6_SQL: &'static str = "
select
dense_rank() over (order by age DESC) as r, name,age
 from cats order by r, name";
static Q6_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/dense-rank-transact-sql#examples";

static Q7_SQL: &'static str = "
select name, weight,
   coalesce(weight - lag(weight, 1) over (order by weight), 0) as weight_to_lose
FROM cats order by weight";
static Q7_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static Q8_SQL: &'static str = "
    select name, breed, weight,
coalesce(weight - lag(weight, 1) over (partition by breed order by weight), 0) as weight_to_lose
from cats order by weight, name ";
static Q8_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/lag-transact-sql#examples";

static Q9_SQL: &'static str = "
select name, color,
first_value(weight) over (partition by color order by weight) as lowest_weight_by_color
from cats order by color, name";
static Q9_HELP: &'static str = "https://docs.microsoft.com/en-us/sql/t-sql/functions/first-value-transact-sql#examples";

static Q10_SQL: &'static str = "
select name, weight,
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              window ntile_window AS
                       ( ORDER BY weight)
     order by weight, name";
static Q10_HELP: &'static str ="http://dcx.sap.com/1200/en/dbreference/window-statement.html";

pub fn get_sql_for_q(s: &str) -> Option<(&str, &str, Vec<&str>)> {
    match s {
        "0" => Some((Q0_SQL, Q0_HELP, vec!["group by"])),
        "1" => Some((Q1_SQL, Q1_HELP, vec!["over"])),
        "2" => Some((Q2_SQL, Q2_HELP, vec!["partition by"])),
        "3" => Some((Q3_SQL, Q3_HELP, vec!["row_number"])),
        "4" => Some((Q4_SQL, Q4_HELP, vec!["rank"])),
        "5" => Some((Q5_SQL, Q5_HELP, vec!["ntile"])),
        "6" => Some((Q6_SQL, Q6_HELP, vec!["dense_rank"])),
        "7" => Some((Q7_SQL, Q7_HELP, vec!["lag", "lead", "min"])),
        "8" => Some((Q8_SQL, Q8_HELP, vec!["lag", "lead", "min"])),
        "9" => Some((Q9_SQL, Q9_HELP, vec!["first_value", "nth_value", "min"])),
        "10" => Some((Q10_SQL, Q10_HELP, vec!["window"])),
        _ => None,
    }
}

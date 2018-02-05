
static Q0_SQL :&'static str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12;";

static Q1_SQL :&'static str = "select name, sum(weight) 
over (order by name) as running_total_weight 
from cats order by name";

static Q2_SQL :&'static str = "
select name, breed, 
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";

static Q3_SQL :&'static str = "
select 
row_number() over (order by color,name) as unique,
name, color 
from cats";

static Q4_SQL :&'static str = "
select 
rank() over (partition by breed order by weight DESC) as ranking,
name, breed, weight
from cats order by ranking, weight DESC";

static Q5_SQL :&'static str = "
select
 name, weight, ntile(4) over ( order by weight) as weight_quartile
       from  cats 
       ";

static Q6_SQL :&'static str = "
select 
dense_rank() over (order by age DESC) as r, name,age
 from cats order by r";

static Q7_SQL :&'static str = "
select name, weight, 
      weight - lag(weight, 1) over (order by weight) as target_weight
      from cats order by weight";

static Q8_SQL :&'static str = "
    select name, breed, weight,
weight - lag(weight, 1) over (partition by breed order by weight) as target_weight
from cats order by weight ";

static Q9_SQL :&'static str = "
select name, color,
first_value(weight) over (partition by color order by weight) as lowest_weight_by_color
from cats ";

static Q10_SQL :&'static str = "
select name, weight, 
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              window ntile_window AS
                       ( ORDER BY weight)
     order by weight";


pub fn get_sql_for_q(s: &str) -> (&str, &str) {
    match s {
        "0" => (Q0_SQL, "group by"),
        "1" => (Q1_SQL, "over"),
        "2" => (Q2_SQL, "partition by"),
        "3" => (Q3_SQL, "row_number"),
        "4" => (Q4_SQL, "rank"),
        "5" => (Q5_SQL, "ntile"),
        "6" => (Q6_SQL, "dense_rank"),
        "7" => (Q7_SQL, "lag"),
        "8" => (Q8_SQL, "lag"),
        "9" => (Q9_SQL, "first_value"),
        "10" => (Q10_SQL, "window"),
        _ => ("select 1 from cats", "")
    }
}


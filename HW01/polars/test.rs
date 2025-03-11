use glob::glob;
use polars::prelude::*;

fn main() {
    let infos_csv = "HW01/Data/Students_info_*.csv";
    let mut infos_vec = vec![];

    for path in glob(infos_csv).unwrap() {
        if let Ok(csv) = path {
            
            let lf = LazyCsvReader::new(csv)
                .finish()
                .unwrap();
            infos_vec.push(lf);
        }
    }
        
    let marks_csv = "HW01/Data/Students_marks_*.csv";
    let mut marks_vec = vec![];

    for path in glob(marks_csv).unwrap() {
        if let Ok(csv) = path {
            let lf = LazyCsvReader::new(csv)
                .finish()
                .unwrap();
            marks_vec.push(lf);
        }
    }

    let df_info = infos_vec.into_iter()
                .reduce(
                    |acc, lf| concat(
                        [acc.clone(), lf.clone()], 
                        UnionArgs::default()
                    ).unwrap())
                .unwrap();

    let df_marks = marks_vec.into_iter()
                .reduce(
                    |acc, lf| concat(
                        [acc.clone(), lf.clone()], 
                        UnionArgs::default()
                    ).unwrap())
                .unwrap();

    let df = df_info.join(
                        df_marks, 
                        [col("index")],
                        [col("index")],
                        JoinArgs::default(),
                    )
                    .collect()
                    .unwrap();

                // если у нас больше 3 столбцов, то считать mean вручную тупо, поэтому можно так
// (украл у deepseek, так как документация polars rust просто ужасна)

let score_columns = df
.clone()
.schema()
.iter_names_cloned()
.filter(|name| name.ends_with("score"))
.collect::<Vec<_>>();

let sum_expr = score_columns
.iter()
.fold(lit(0.0), |acc, col_name| acc + col(PlSmallStr::from_str(col_name)).cast(DataType::Float64));


let count = score_columns.len() as f64;
let average_expr = (sum_expr / lit(count)).alias("average");

let df = df.lazy()
                    .with_column(average_expr.clone())
                    .with_column(
                        when(col("average").lt(50.0))
                            .then(lit("F"))
                            .when(col("average").lt(60.0))
                            .then(lit("E"))
                            .when(col("average").lt(70.0))
                            .then(lit("D"))
                            .when(col("average").lt(80.0))
                            .then(lit("C"))
                            .when(col("average").lt(90.0))
                            .then(lit("B"))
                            .otherwise(lit("A"))
                            .alias("Grade")
                    )
                    .drop(["average"])
                    .collect()
                    .unwrap();
}
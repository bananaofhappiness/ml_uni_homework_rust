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
}
#[derive(Debug)]
struct DemoStats {
    count: i64,
    min: i64,
    max: i64,
    sum: i64,
}
impl DemoStats {
    fn new() -> DemoStats {
        DemoStats {
            count: 0,
            min: 0,
            max: 0,
            sum: 0,
        }
    }
}

// #[derive(Debug)]
// struct Demo {
//     x: i32,
//     y: i32,
// }
// impl Demo {
//     fn new() -> Demo {
//         Demo { x: 0, y: 0 }
//     }
// }

#[get("/fold")]
pub fn fold() -> String {
    let stats: Vec<DemoStats> = vec![
        DemoStats {
            count: 43,
            min: 43,
            max: 543,
            sum: 4234,
        },
        DemoStats {
            count: 43,
            min: 83,
            max: 643,
            sum: 43253,
        },
        DemoStats {
            count: 43,
            min: 43,
            max: 593,
            sum: 4234,
        },
        DemoStats {
            count: 43,
            min: 430,
            max: 5403,
            sum: 5435,
        },
    ];
    let mut minimum = i64::MAX;
    let mut maximum = i64::MIN;
    let final_stats = stats.iter().fold(DemoStats::new(), |acc, x| {
        // The below commented code will give 0 as min value if there are no negative entries
        // if acc.min < x.min {
        //     minimum = acc.min;
        // } else {
        //     minimum = x.min;
        // }
        if minimum > x.min {
            minimum = x.min;
        }
        if acc.max > x.max {
            maximum = acc.max;
        } else {
            maximum = x.max;
        }
        DemoStats {
            count: acc.count + x.count,
            min: minimum,
            max: maximum,
            sum: acc.sum + x.sum,
        }
    });

    // let num = vec![1, 2, 3, 3, 4, 5];
    // let stat = num.iter().fold(0, |acc, &x| acc + x);
    // println!("{}", stat);

    // let num1 = vec![
    //     Demo { x: 1, y: 2 },
    //     Demo { x: 34, y: 534 },
    //     Demo { x: 76, y: 43 },
    //     Demo { x: 54, y: 76 },
    // ];
    // let res = num1.iter().fold(Demo::new(), |acc, a| Demo {
    //     x: acc.x + a.x,
    //     y: acc.y + a.y,
    // });
    // println!("{:?}", res);

    format!("executed fold! {:?}", final_stats)
}

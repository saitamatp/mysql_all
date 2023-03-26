use mysql::*;
use mysql::prelude::*;

pub fn period_end(url:String)-> String{
    let opts = Opts::from_url(&url).expect("Unable to parse URL");
    let pool = Pool::new(opts).expect("Unable to parse URL");
    let mut conn = pool.get_conn().expect("Unable to connect to MYSQL server");

    "use ledger;call sp_period_end_cal;"
    .run(&mut conn).expect("Unable to trigger SP");

    return String::from("Ran period end SP");

}
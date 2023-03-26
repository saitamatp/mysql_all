use mysql::*;
use mysql::prelude::*;

pub fn clear_tmp(url:String)-> String{

    let opts = Opts::from_url(&url).expect("Unable to parse URL");
    let pool = Pool::new(opts).expect("Unable to parse URL");
    let mut conn = pool.get_conn().expect("Unable to connect to MYSQL server");

    conn.query_drop(r"delete from ledger.tmp_expense_detail where row_stat in ('DELETED','IMPORTED','VALUE_EXISTS');")
                    .expect("Unable to clear temporay tables");
    return String::from("Cleared temporary tables");

}
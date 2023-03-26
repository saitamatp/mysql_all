use mysql::*;
use mysql::prelude::*;
use mysql_all::RecordMisc;
use mysql_all::LedgerMisc;
use mysql_all::dealloc_misc;

pub fn credit_load(url:String,path: String)-> String{

    let opts = Opts::from_url(&url).expect("Unable to parse URL");
    let pool = Pool::new(opts).expect("Unable to parse URL");
    let mut conn = pool.get_conn().expect("Unable to connect to MYSQL server");

    let mut j : Vec<LedgerMisc> = Vec::new();
    let mut rdr=csv::Reader::from_path(path).expect("Unable to locate file");
    for result in rdr.deserialize() {
    let record: RecordMisc = result.expect("Failed to load to Ledger Struct");
    j.push(
           LedgerMisc{
            ID: record.ID,
            From: record.From,
            Period_ID: record.Period_ID,
            Amount_recieved:record.Amount_recieved,
            Description: record.Description,
            Date: record.Date,
            Expense_ID: record.Expense_ID,
            Paid_ID: record.Paid_ID
           });
    }

conn.exec_batch(
    r"insert into ledger.misc_credit(id,payment_from,period_id,amount_paid,summary,repaid_date,expense_id,paid_id)
    Values (:id,:payment_from,:period_id,:amount_paid,:summary,STR_TO_DATE(:repaid_date,'%d-%c-%Y  %T'),:expense_id,:paid_id)
     ON DUPLICATE KEY UPDATE payment_from=:payment_from,period_id=:period_id,amount_paid=:amount_paid,summary=:summary,repaid_date=STR_TO_DATE(:repaid_date,'%d-%c-%Y  %T'),expense_id=:expense_id,paid_id=:paid_id",
    j.iter().map(|p| params! {
        "id"=>p.ID,
        "payment_from"=>&p.From,
        "period_id" => p.Period_ID,
        "amount_paid" => p.Amount_recieved,
        "summary" => &p.Description,
        "repaid_date"=>&p.Date,
        "expense_id"=>p.Expense_ID,
        "paid_id"=>p.Paid_ID,
        
    })
).expect("Failed to load to base tables(SQL error)");
    
dealloc_misc(j);
    return String::from("Loaded to credit load tables");
}
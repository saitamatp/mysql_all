use mysql_all::Ledger;
use mysql_all::Record;
use mysql::*;
use mysql::prelude::*;
use std::thread;
use std::sync::mpsc;

pub fn csv_read(url:String,path: String)-> String{

    let (tx, rx) = mpsc::channel();
    
    let opts = Opts::from_url(&url).expect("Unable to parse URL");
    let pool = Pool::new(opts).expect("Unable to pool(create maintain) Connection");
    let mut conn = pool.get_conn().expect("Unable to connect to MYSQL server");
    let mut j : Vec<Ledger> = Vec::new();
    
    thread::spawn(move || {
    let mut rdr=csv::Reader::from_path(path).expect("Unable to locate file");
    for result in rdr.deserialize() {
        
        let record: Record = result.expect("Failed to load to Ledger Struct");
       /*Push recods one by one to temp vector*/
        j.push(
           Ledger{
            id: record.Expense_ID,
            row_stat: "IMPORT".to_owned(),
            expense_id: record.Expense_ID,
            expense_date: record.Expense_date,
            paid_to:record.Paid_To,
            summary_p: record.Summary,
            period_id: record.Period_ID,
            paid_id: record.Paid_ID,
            amount_paid: record.Amount_Paid,
            leisure: record.Leisure
           });
    
}
        for val in j {
            tx.send(val).expect("Unable to send the message to main thread");
        }
    });

conn.exec_batch(
    r"insert into ledger.tmp_expense_detail(id,row_stat,expense_id,expense_date,paid_to,summary,period_id,paid_id,amount_paid)
    Values (:id,:row_stat,:expense_id,STR_TO_DATE(:expense_date,'%d-%c-%Y  %T'),:paid_to,:summary,:period_id,:paid_id,:amount_paid)
     ON DUPLICATE KEY UPDATE row_stat='IMPORT',expense_date= STR_TO_DATE(:expense_date,'%d-%c-%Y  %T'),paid_to=:paid_to,summary=:summary,period_id=:period_id,paid_id=:paid_id,amount_paid=:amount_paid",
    rx.iter().map(|p| params! {
        "id"=>p.id,
        "row_stat"=>p.row_stat.to_owned(),
        "expense_id" => p.expense_id,
        "expense_date" => &p.expense_date,
        "paid_to" => &p.paid_to,
        "summary"=>&p.summary_p,
        "period_id"=>p.period_id,
        "paid_id"=>p.paid_id,
        "amount_paid"=>p.amount_paid,
        
        })
        ).expect("Failed to load to temp tables");

        return String::from("Loaded to temp table successfully");
}
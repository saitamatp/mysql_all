use mysql::*;
use mysql::prelude::*;
use mysql_all::LedgerLoad;
use mysql_all::dealloc;

pub fn load_base(url:String)-> String{
    let opts = Opts::from_url(&url).expect("Unable to parse URL");
    let pool = Pool::new(opts).expect("Unable to parse URL");
    let mut conn = pool.get_conn().expect("Unable to connect to MYSQL server");

    //Update the records that are already present
    //conn.query_drop(update()).expect("Unable to update");

   /*let all_values:Vec<LedgerLoad>=conn
    .query_map("select id,row_stat,expense_id,expense_date,paid_to,summary,period_id,paid_id,amount_paid from ledger.tmp_expense_detail where Row_stat in ('DELETE','IMPORT')", 
    |(id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid)| {
            LedgerLoad { id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid}
        },
    ).expect("Unable to read delete records from  MYSQL temp table");

    let del_ledger_upd:Vec<LedgerLoad>;
    del_ledger_upd.push( 
        all_values.iter().filter(|&p|p.row_stat==String::from("DELETE"))
    );
     *///all_values.iter().filter(|&p|p.row_stat==String::from("DELETE"));

/*                                1st Part of the program
--------------------------------Delete base tables code below --------------------------------------------------------------------*/
    let del_ledger:Vec<LedgerLoad>=conn
    .query_map("select id,row_stat,expense_id,expense_date,paid_to,summary,period_id,paid_id,amount_paid from ledger.tmp_expense_detail where Row_stat='DELETE'", 
    |(id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid)| {
            LedgerLoad { id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid}
        },
    ).expect("Unable to read delete records from  MYSQL temp table");

    conn.exec_batch(
    r"delete from ledger.expense_detail where expense_id=:expense_id",
    del_ledger.iter().map(|p| params! {
        "expense_id" => p.expense_id,
    })
).expect("Unable to update the status in temp table");

conn.exec_batch(
    r"update ledger.tmp_expense_detail set Row_stat='DELETED',message='No Issue' where Id=:id",
    del_ledger.iter().map(|p| params! {
        "id" => p.id,
    })
).expect("Unable to update the status in temp table");

dealloc(del_ledger,String::from("DELETE"));

/*--------------------------------Delete base tables code complete --------------------------------------------------------------------*/


/*                                  2nd Part of the program
--------------------------------Update base tables code below --------------------------------------------------------------------*/
let upd_ledger:Vec<LedgerLoad>=conn.
    query_map("select exp.id,exp.row_stat,exp.expense_id,exp.expense_date,exp.paid_to,exp.summary,exp.period_id,exp.paid_id,exp.amount_paid 
    from ledger.tmp_expense_detail exp
    where exp.expense_id in (select b.expense_id from ledger.expense_detail b where b.expense_id=exp.expense_id)
    and exp.Row_stat='IMPORT'", 
    |(id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid)|{
    LedgerLoad { id,row_stat, expense_id, expense_date, paid_to, summary_p, period_id, paid_id, amount_paid }
    },
).expect("Unable to read from mysql temp tables for update");


/*Update records in base table */
conn.exec_batch(
    r"update ledger.expense_detail base
    set base.expense_date=:expense_date,
        base.paid_to=:paid_to,
        base.summary=:summary,
        base.period_id=:period_id,
        base.paid_id=:paid_id,
        base.amount_paid=:amount_paid
    where base.expense_id=:expense_id",
    upd_ledger.iter().map(|p| params! {
        "expense_id" => p.expense_id,
        "expense_date" => &p.expense_date,
        "paid_to" => &p.paid_to,
        "summary"=>&p.summary_p,
        "period_id"=>p.period_id,
        "paid_id"=>p.paid_id,
        "amount_paid"=>p.amount_paid
    })
).expect("Unable to update to Base tables");


conn.exec_batch(
    r"update ledger.tmp_expense_detail set Row_stat='VALUE_EXISTS',message='No Issue and updated' where Id=:id",
    upd_ledger.iter().map(|p| params! {
        "id" => p.id,
    })
).expect("Unable to update the status in temp table-1");

dealloc(upd_ledger, String::from("UPDATE"));



/*--------------------------------Update base tables code Complete --------------------------------------------------------------------*/


/*                              3rd Part of the program
--------------------------------Insert to  base tables code below --------------------------------------------------------------------*/
let ledger:Vec<LedgerLoad> = conn
    .query_map(
        "select id,row_stat,expense_id,expense_date,paid_to,summary,period_id,paid_id,amount_paid from ledger.tmp_expense_detail where Row_stat='IMPORT'",
        |(id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid)| {
            LedgerLoad { id,row_stat,expense_id,expense_date,paid_to,summary_p,period_id,paid_id,amount_paid}
        },
    ).expect("Unable to read from MYSQL temp table for insert");

conn.exec_batch(
    r"insert into ledger.expense_detail(expense_id,expense_date,paid_to,summary,period_id,paid_id,amount_paid)
    Values (:expense_id,:expense_date,:paid_to,:summary,:period_id,:paid_id,:amount_paid)
     ON DUPLICATE KEY UPDATE expense_date= :expense_date,paid_to=:paid_to,summary=:summary,period_id=:period_id,paid_id=:paid_id,amount_paid=:amount_paid",
    ledger.iter().map(|p| params! {
        "expense_id" => p.expense_id,
        "expense_date" => &p.expense_date,
        "paid_to" => &p.paid_to,
        "summary"=>&p.summary_p,
        "period_id"=>p.period_id,
        "paid_id"=>p.paid_id,
        "amount_paid"=>p.amount_paid
    })
).expect("Unable to load to Base tables");

conn.exec_batch(
    r"update ledger.tmp_expense_detail set Row_stat='IMPORTED',message='No Issue' where Id=:id",
    ledger.iter().map(|p| params! {
        "id" => p.id,
    })
).expect("Unable to update the status in temp table");

//Deallocate
dealloc(ledger,String::from("INSERT"));

/*--------------------------------Insert to  base tables code Complete --------------------------------------------------------------------*/

return String::from("Loaded to base tables");

}
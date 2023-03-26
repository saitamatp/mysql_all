use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Record {
    pub Expense_ID: i32,
    pub Expense_date: String,
    pub Paid_To: String,
    pub Summary: String,
    pub Period_ID: i32,
    pub Paid_ID: i32,
    pub Amount_Paid: f32,
    pub Leisure:String
}


/*Store CSV data to vector of stucts */
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Ledger {
    pub id: i32,
    pub row_stat: String,
    pub expense_id: i32,
    pub expense_date: String,
    pub paid_to:String,
    pub summary_p: String,
    pub period_id: i32,
    pub paid_id: i32,
    pub amount_paid: f32,
    pub leisure: String
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LedgerLoad {
    pub id: i32,//required
    pub row_stat: String,//required
    pub expense_id: i32,//required
    pub expense_date: String,//required
    pub paid_to:String,//required
    pub summary_p: Option<String>,//Nullable
    pub period_id: i32,//required
    pub paid_id: i32,//required
    pub amount_paid: f32//required
    //leisure: i32
}

/*Store CSV data in vector of structs */
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RecordMisc {
    pub ID: i32,
    pub From: String,
    pub Period_ID: i32,
    pub Amount_recieved: f32,
    pub Description: String,
    pub Date: String,
    pub Expense_ID: i32,
    pub Paid_ID:i32
}

#[allow(non_snake_case)]
/*Store CSV data to vector of stucts */
#[derive(Debug, PartialEq, PartialOrd)]
pub struct LedgerMisc {
    pub ID: i32,
    pub From: String,
    pub Period_ID: i32,
    pub Amount_recieved: f32,
    pub Description: String,
    pub Date: String,
    pub Expense_ID: i32,
    pub Paid_ID:i32
}

pub fn dealloc(a:Vec<LedgerLoad>,b:String){
    
    if b.eq("DELETE") {
        println!("Deallocating and the number of records that were deleted are-{}",a.len());
    }else if b.eq("INSERT") {
        println!("Deallocating and the number of new records that were inserted-{}",a.len());
    }else if b.eq("UPDATE") {
        println!("Deallocating and the number of records that were updated is -{}",a.len());
    }   
}

pub fn dealloc_misc(a:Vec<LedgerMisc>){
    println!("Deallocating and the number of records that were processed-{}",a.len());
}
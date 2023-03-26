# MYSQL Leadger Load

##Overview

I note down all my expenses in access Db, I needed a way to load all the records to MYSQL from access database which could be controlled by front-end application.Initially  I worte the ([Front-end](https://github.com/saitamatp/MySQLcontrol)) in C# and ([Back-end](https://github.com/saitamatp/mysql_fun)) in rust, but now I found a way to package both the front-end and backend in rust using EGUI.

Main.rs Contains all the egui Code and calls functions from various folders.
lib.rs Contains all the structs necessary for data load. 

### Different Folders contain various functionalities inside SRC folder 
* CSV_to_database-Read CSV(Convert all my ledger enteries to CSV) file and then load to MYSQL temp Tables.
* Load_to_base-Read the temporary table values and load to base tables.
* clear_tmp-Clear temporary tables once base load is complete (Only processed records are removed).
* credit_load-Load credit CSV file to MYSQL database tables.
* period_end_cal-Trigger month end ([Stored Procedure](https://github.com/saitamatp/miscellaneous/blob/main/sp_period_end_cal.sql)) to calculate total expense by different modes of payment and store it in different table.

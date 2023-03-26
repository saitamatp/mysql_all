//egui
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::epaint::Vec2;
use eframe::Theme;

mod prm_reader;
use prm_reader::prm_reader::find_value;
use prm_reader::prm_reader::read;
mod csv_to_database;
use csv_to_database::csv_read::csv_read;
mod load_to_base;
use load_to_base::load_to_base::load_base;
mod clear_tmp;
use clear_tmp::clear_tmp::clear_tmp;
mod credit_load;
use credit_load::credit_load::credit_load;
mod period_end;
use period_end::period_end::period_end;

pub struct Task1App {
    value: String,
}

impl Default for Task1App {
    fn default() -> Self {
        Self {
            value: String::from("Need to start the app")
        }
    }
}

impl Task1App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Task1App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    /*Mysql Parmater defination */
    let values=read();
    let host_name=find_value(&values,"host".to_string());
    let ports=find_value(&values,"port".to_string());
    let user_name=find_value(&values,"username".to_string());
    let passwords=find_value(&values,"password".to_string());
    let url;
    url=format!("mysql://{}:{}@{}:{}",user_name,passwords,host_name,ports);
    
    let path1:String=find_value(&values,"src_file1".to_string());
    let path2:String=find_value(&values,"src_file2".to_string());
    let message:String=format!("The connection URL is -mysql://{}:XXXX@{}:{}",user_name,host_name,ports);
    self.value=message;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", self.value));
                if ui.button("Read from csv and load to temp tables").clicked() {
                    self.value =csv_read(url.clone(),path1);
                }
            });
            //
            ui.horizontal(|ui| {
                if ui.button("Load to base tables from temp tables").clicked() {
                    self.value =load_base(url.clone());
                }
            });
            //
            //
            ui.horizontal(|ui| {
                if ui.button("Clear temporary tables after data load").clicked() {
                    self.value =clear_tmp(url.clone());
                }
            });
            //
            ui.horizontal(|ui| {
                if ui.button("Load Credit data").clicked() {
                    self.value =credit_load(url.clone(),path2);
                }
            });
            //
              //
            ui.horizontal(|ui| {
                if ui.button("Period End, calculate").clicked() {
                    self.value =period_end(url.clone());
                }
            });
            //
        });
    }
}

fn main() {
  
//egui
    let native_options = eframe::NativeOptions{
        default_theme: Theme::Dark,
        initial_window_size: Option::from(
            Vec2::new(400.0, 200.0)
        ),
        resizable: false,
        ..Default::default()
    };

   eframe::run_native(
        "MySQL Ledger Load",
        native_options,
        Box::new(|cc| Box::new( Task1App::new(cc))),
    );

}

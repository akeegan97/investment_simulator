use std::str::FromStr;

use chrono::{NaiveDate, DateTime,Local, Datelike};
use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2};
use super::product_page::{LISTINGS, PACKAGES};
use egui_extras::DatePickerButton;


pub fn ibfl_sim(ctx:&egui::Context,years:&mut f64,selected_ibfl:&mut Option<LISTINGS>,selected_services:&mut Option<PACKAGES>, mort_rate:&mut f64, first_payment:&mut NaiveDate, 
    second_payment:&mut NaiveDate,third_payment:&mut NaiveDate, fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64,
    third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64, prop_mgmt: &mut f64, service_package: &mut f64, app_rate:&mut f64, expense_withholding:&mut f64,
    rent:&mut f64){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
        col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Parameter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.add(egui::Label::new(RichText::new("Parameters").color(Color32::WHITE).font(FontId::proportional(18.0))));
        ui.separator();
        //getting listing selected:
        let picked_listing:f64;
        match *selected_ibfl{
            Some(LISTINGS::First) => picked_listing = 1_000_000.0,
            Some(LISTINGS::Second)=> picked_listing= 1_250_000.0,
            Some(LISTINGS::Third)=> picked_listing=2_000_000.0,
            Some(LISTINGS::Fourth)=>picked_listing=2_500_000.0,
            Some(LISTINGS::Fifth)=>picked_listing=3_000_000.0,
            _=>picked_listing=0.0,
        };
        let name_listing:String;
        match *selected_ibfl{
            Some(LISTINGS::First) => name_listing= "Name #1".to_string(),
            Some(LISTINGS::Second)=> name_listing= "Name #2".to_string(),
            Some(LISTINGS::Third)=> name_listing= "Name #3".to_string(),
            Some(LISTINGS::Fourth)=>name_listing= "Name #4".to_string(),
            Some(LISTINGS::Fifth)=>name_listing= "Name #5".to_string(),
            _=>name_listing= "No Name".to_string(),
        };
        ui.label(RichText::new(format!("Selected Unit: {} ",name_listing)).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(picked_listing))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        ui.add(egui::Slider::new(mort_rate,0.0..=10.0).text(RichText::new("Mortgage Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(app_rate,0.0..=10.0).text(RichText::new("Appreciation Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(expense_withholding,0.0..=30.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent,2_000.0..=10_000.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.horizontal(|ui: &mut egui::Ui|{
            ui.add(egui::Label::new(RichText::new("First Payment").color(Color32::WHITE).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(first_payment).id_source("First Payment"));
        });
        ui.add(egui::Slider::new(first_payment_percent, 0.0..=(100.0 - *second_payment_percent - *third_payment_percent - *fourth_payment_percent - *fifth_payment_percent))
        .clamp_to_range(false).suffix(" %").max_decimals(0));
        ui.horizontal(|ui|{
            ui.add(egui::Label::new(RichText::new("Second Payment").color(Color32::WHITE).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(second_payment).id_source("Second Payment"));
        });
        ui.add(egui::Slider::new(second_payment_percent, 0.0..=(100.0-*first_payment_percent - *third_payment_percent - *fourth_payment_percent - *fifth_payment_percent))
        .clamp_to_range(false).suffix(" %").max_decimals(0));
        ui.horizontal(|ui|{
            ui.add(egui::Label::new(RichText::new("Third Payment").color(Color32::WHITE).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(third_payment).id_source("Third Payment"));
        });
        ui.add(egui::Slider::new(third_payment_percent, 0.0..=(100.0-*first_payment_percent - *second_payment_percent - *fourth_payment_percent - *fifth_payment_percent))
        .clamp_to_range(false).suffix(" %").max_decimals(0));
        ui.horizontal(|ui|{
            ui.add(egui::Label::new(RichText::new("Fourth Payment").color(Color32::WHITE).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(fourth_payment).id_source("Fourth Payment"));
        });
        ui.add(egui::Slider::new(fourth_payment_percent, 0.0..=(100.0-*first_payment_percent - *second_payment_percent - *third_payment_percent - *fifth_payment_percent))
        .clamp_to_range(false).suffix(" %").max_decimals(0));
        ui.horizontal(|ui|{
            ui.add(egui::Label::new(RichText::new("Fifth Payment").color(Color32::WHITE).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(fifth_payment).id_source("Fifth Payment"));
        });
        ui.add(egui::Slider::new(fifth_payment_percent, 0.0..=(100.0-*first_payment_percent - *second_payment_percent - *third_payment_percent - *fourth_payment_percent))
        .clamp_to_range(false).suffix(" %").max_decimals(0));
    });
        match *selected_services{
            Some(PACKAGES::High)=> *service_package = 10_000.0,
            Some(PACKAGES::Med) => *service_package = 7_000.0,
            Some(PACKAGES::Low) => *service_package = 5_000.0,
            None => *service_package = 0.0,
        }
    egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
    });

}
pub fn ibfp_1_precon_sim(years:&mut f64, interest_rate:&mut f64, mort_rate:&mut f64, price_precon:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, rent:&mut f64){
        let y = *years * 12.0;//total months
        let first_payment_amt:f64 = *price_precon * (*first_payment_percent * 0.01);
        let second_payment_amt:f64 = *price_precon *(*second_payment_percent * 0.01);
        let third_payment_amt:f64 = *price_precon *(*third_payment_percent * 0.01);
        let fourth_payment_amt:f64 = *price_precon *(*fourth_payment_percent * 0.01);
        let fifth_payment_amt:f64 = *price_precon *(*fifth_payment_percent * 0.01);
        let first_payment_date:NaiveDate = *first_payment;
        let second_payment_date:NaiveDate = *second_payment;
        let third_payment_date:NaiveDate = *third_payment;
        let fourth_payment_date:NaiveDate = *fourth_payment;
        let fifth_payment_date:NaiveDate = *fifth_payment;
        let local:DateTime<Local>= Local::now();
        let start_months = local.date_naive().num_days_from_ce();
        let p_1 = first_payment_date.num_days_from_ce();
        let p_2 = second_payment_date.num_days_from_ce();
        let p_3 = third_payment_date.num_days_from_ce();
        let p_4 = fourth_payment_date.num_days_from_ce();
        let p_5 = fifth_payment_date.num_days_from_ce();
        let x1 = ((start_months - p_1) as f64)/30.0; // number of months from today until first payment due 
        let x2 = ((start_months - p_2)as f64)/30.0;//number of months from today until second payment
        let x3 =((start_months - p_3)as f64)/30.0;
        let x4 =((start_months - p_4)as f64)/30.0;
        let x5 =((start_months - p_5)as f64)/30.0;
        let mut results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>=Vec::new();
        let mut x = 0.0;
        let mut capex:f64 = 0.0;
        let expense_withholding:f64 = *expense_withholding;
        while x<=y{
            capex = 0.0;
            while x<=x5{
                capex = 0.0;
                if x == x1{
                    capex = first_payment_amt;
                }
                if x == x2{
                    capex = second_payment_amt;
                }
                if x == x3{
                    capex = third_payment_amt;
                }
                if x == x4{
                    capex = fourth_payment_amt;
                }
                if x == x5{
                    capex = fifth_payment_amt
                }
                x+=1.0;
                results.push(([x,capex],[x,0.0],[x,0.0],[x,0.0]));
            }
            let rev = *rent; 
            results.push(([x,capex],[x,0.0],[x,0.0],[x,0.0]));
            x+=1.0;
        }

        



}
pub fn ibfp_mkt_sim(){

}
pub fn ibfp_precon_mkt(){

}

pub fn fund_core(){
    
}

pub fn fund_plus(){

}
pub fn fund_fi(){

}

const MARGIN:Vec2 = Vec2{x: 7.0,y: 7.0};
//utility function for formatting
pub fn format_dollar_amount(value: impl ToString) -> String {
    let value_str = value.to_string();
    let value_f64 = value_str.parse::<f64>().unwrap_or_default();

    let formatted = format!("{:.2}", value_f64);
    let parts: Vec<&str> = formatted.split('.').collect();

    let int_part = parts[0].chars().rev().collect::<String>();
    let mut formatted_with_commas = String::new();
    let mut count = 0;

    for c in int_part.chars() {
        if count == 3 {
            formatted_with_commas.push(',');
            count = 0;
        }
        count += 1;
        formatted_with_commas.push(c);
    }

    formatted_with_commas = formatted_with_commas.chars().rev().collect();
    formatted_with_commas.push('.');
    formatted_with_commas.push_str(&parts[1]);
    formatted_with_commas.insert(0, '$');

    formatted_with_commas
}
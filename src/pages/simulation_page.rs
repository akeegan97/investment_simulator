
use chrono::{NaiveDate, DateTime,Local, Datelike};
use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2, plot::{PlotPoints,Line, Legend, Plot}};
use super::product_page::{LISTINGS, PACKAGES};
use egui_extras::DatePickerButton;



pub fn ibfl_sim(ctx:&egui::Context,years:&mut f64,selected_ibfl:&mut Option<LISTINGS>,selected_services:&mut Option<PACKAGES>, mort_rate:&mut f64, first_payment:&mut NaiveDate, 
    second_payment:&mut NaiveDate,third_payment:&mut NaiveDate, fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64,
    third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64, prop_mgmt: &mut f64, service_pkg_price: &mut f64, app_rate:&mut f64, expense_withholding:&mut f64,
    rent:&mut f64, selected_mgmt:&mut bool,price_precon:&mut f64,debt_ratio:&mut f64, interest_rate:&mut f64){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
        col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Parameter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.add(egui::Label::new(RichText::new("Parameters").color(Color32::WHITE).font(FontId::proportional(18.0))));
        ui.separator();
        //getting listing selected:
        
        match *selected_ibfl{
            Some(LISTINGS::First) => *price_precon = 1_000_000.0,
            Some(LISTINGS::Second)=> *price_precon= 1_250_000.0,
            Some(LISTINGS::Third)=> *price_precon=2_000_000.0,
            Some(LISTINGS::Fourth)=>*price_precon=2_500_000.0,
            Some(LISTINGS::Fifth)=>*price_precon=3_000_000.0,
            _=>*price_precon=0.0,
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
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(*price_precon))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        ui.add(egui::Slider::new(mort_rate,0.0..=10.0).text(RichText::new("Mortgage Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(app_rate,0.0..=10.0).text(RichText::new("Appreciation Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(expense_withholding,0.0..=30.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent,2_000.0..=10_000.0).text(RichText::new("Rent").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(debt_ratio,2_000.0..=10_000.0).text(RichText::new("Rent").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));//wrong
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
            Some(PACKAGES::High)=> *service_pkg_price = 10_000.0,
            Some(PACKAGES::Med) => *service_pkg_price = 7_000.0,
            Some(PACKAGES::Low) => *service_pkg_price = 5_000.0,
            None => *service_pkg_price = 0.0,
        }
        if *selected_mgmt == true{
            *prop_mgmt = 0.35;
        }
    egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
        ui.heading(RichText::new("Graph").color(Color32::WHITE).font(FontId::proportional(20.0)));
        let ans  = ibfl_precon(years, interest_rate, mort_rate, price_precon, 
            first_payment, second_payment, third_payment, fourth_payment, fifth_payment, 
            first_payment_percent, second_payment_percent, third_payment_percent, fourth_payment_percent, 
            fifth_payment_percent, service_pkg_price, prop_mgmt, app_rate, expense_withholding, 
            rent, debt_ratio);
        let cap_vec = split_ans(&ans, 0);
        let income_vec = split_ans(&ans, 1);
        let prop_value_vec = split_ans(&ans, 2);
        let mort_vec = split_ans(&ans, 3);

        let cap_line:Line = Line::new(PlotPoints::new(cap_vec)).color(Color32::LIGHT_RED).name("Capital Requirements");
        let income_line:Line = Line::new(PlotPoints::new(income_vec)).color(Color32::LIGHT_GREEN).name("Income ");
        let value_line:Line = Line::new(PlotPoints::new(prop_value_vec)).color(Color32::WHITE).name("Property Value");
        let mort_line:Line = Line::new(PlotPoints::new(mort_vec)).color(Color32::RED).name("Mortgage Liability");

        Plot::new("Graph")
            .view_aspect(4.0)
            .auto_bounds_y()
            .include_y(0.0)
            .legend(Legend::default().position(egui::plot::Corner::LeftTop))
            .show(ui, |p_ui|{p_ui.line(cap_line);p_ui.line(income_line);p_ui.line(value_line);p_ui.line(mort_line);});

    });

}
fn ibfl_precon(years:&mut f64, _interest_rate:&mut f64, mort_rate:&mut f64, price_precon:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_pgk_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, rent:&mut f64, debt_ratio:&mut f64)->Vec<([f64;2],[f64;2],[f64;2],[f64;2])>{
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
        let mut rent_revenue_updated = *rent;
        let mut prop_value = *price_precon;
        let service_expense = *service_pgk_price;
        while x<=y{
           let mut cap = 0.0;
            while x<=x5{
                cap = 0.0;
                if x == x1{
                    cap = -first_payment_amt;
                }
                if x == x2{
                    cap = -second_payment_amt;
                }
                if x == x3{
                    cap = -third_payment_amt;
                }
                if x == x4{
                    cap = -fourth_payment_amt;
                }
                if x == x5{
                    cap = -fifth_payment_amt
                }
                x+=1.0;
                results.push(([x,cap],[x,0.0],[x,0.0],[x,0.0]));
            }
            cap = 0.0;
            let mortgage_liability:f64 = *price_precon * *debt_ratio;
            let mort_payment:f64 = mortgage_liability * *mort_rate*0.01;
            let mut cap_inject:f64;
            if x == x5+1.0{
                cap_inject = *price_precon * (*debt_ratio*0.01);
            }else{
                cap_inject = 0.0;
            }
            let mut net_income = (rent_revenue_updated * (*prop_mgmt) *(*expense_withholding*0.01)+cap_inject) - (mort_payment);
            if x%12.0 == 0.0{//checks if it is a year to subtract yearly service package cost + increase property value by app_rate and rent by rent_app
                rent_revenue_updated+=(rent_revenue_updated * 0.05);
                prop_value +=prop_value * (*app_rate*0.01);
                net_income = rent_revenue_updated * (*prop_mgmt) *(*expense_withholding*0.01)+cap_inject - service_expense - mort_payment;
            }
            prop_value += prop_value * (*app_rate*0.01);
            results.push(([x,cap],[x,net_income],[x,prop_value],[x,mortgage_liability]));
            x+=1.0;
        }
        return results

        



}
pub fn ibfp_1_precon_sim(){

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
fn split_ans(results:&Vec<([f64;2],[f64;2],[f64;2],[f64;2])>, position:i64)->Vec<[f64;2]>{
    let mut ans:Vec<[f64;2]>=Vec::new();
    if position == 0{
        for(i,_,_,_,) in results{
            ans.push(*i);
        }
    }
    if position == 1{
        for(_,i,_,_,) in results{
            ans.push(*i);
        }
    }
    if position == 2{
        for(_,_,i,_,) in results{
            ans.push(*i);
        }
    }
    if position == 3{
        for(_,_,_,i,) in results{
            ans.push(*i);
        }
    }
    if position == 4{
        for(_,_,_,i) in results{
            ans.push(*i);
        }
    }
    return ans
}
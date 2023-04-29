
use chrono::{NaiveDate, DateTime,Local, Datelike};
use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2, plot::{PlotPoints,Line, Legend, Plot}};
use super::product_page::{LISTINGS, PACKAGES};




pub fn ibfl_sim(ctx:&egui::Context,years:&mut f64,selected_ibfl:&mut Option<LISTINGS>,selected_services:&mut Option<PACKAGES>, mort_rate:&mut f64, first_payment:&mut NaiveDate, 
    second_payment:&mut NaiveDate,third_payment:&mut NaiveDate, fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64,
    third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64, prop_mgmt: &mut f64, service_pkg_price: &mut f64, app_rate:&mut f64, expense_withholding:&mut f64,
    rent:&mut f64, selected_mgmt:&mut bool,price_precon:&mut f64,debt_ratio:&mut f64, interest_rate:&mut f64, rent_app:&mut f64, closing_costs:&mut f64){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
        col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION InBest For Life").color(Color32::WHITE).font(FontId::proportional(24.0))));
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
        ui.add(egui::Slider::new(rent,2_000.0..=30_000.0).text(RichText::new("Rent").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(debt_ratio,0.0..=100.0).text(RichText::new("Leveraged Ratio").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent_app,0.0..=10.0).text(RichText::new("Yearly Rent Increase").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(closing_costs, 0.0..=100_000.0).text(RichText::new("Closing Costs").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
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
        let ans: Vec<([f64; 2], [f64; 2], [f64; 2], [f64; 2])>  = ibfl_precon(years, interest_rate, mort_rate, price_precon, 
            first_payment, second_payment, third_payment, fourth_payment, fifth_payment, 
            first_payment_percent, second_payment_percent, third_payment_percent, fourth_payment_percent, 
            fifth_payment_percent, service_pkg_price, prop_mgmt, app_rate, expense_withholding, 
            rent, debt_ratio,rent_app, closing_costs);
        let cap_vec: Vec<[f64; 2]> = split_ans(&ans, 0);
        let income_vec: Vec<[f64; 2]> = split_ans(&ans, 1);
        let prop_value_vec: Vec<[f64; 2]> = split_ans(&ans, 2);
        let mort_vec: Vec<[f64; 2]> = split_ans(&ans, 3);
        let income_generated:f64;
        if let Some(total_income) = income_vec.last(){
            income_generated = total_income[1];
        }else{
            income_generated = 0.0;
        }
        let mortgage_liab:f64;
        if let Some(mort) = mort_vec.last(){
            mortgage_liab = mort[1];
        }else{
            mortgage_liab = 0.0;
        }
        let prop_value:f64;
        if let Some(value) = prop_value_vec.last(){
            prop_value = value[1];
        }else{
            prop_value = 0.0;
        }
        let unrealized_gains:f64 = prop_value - *price_precon;


        


        let cap_line:Line = Line::new(PlotPoints::new(cap_vec)).color(Color32::LIGHT_RED).name("Capital Requirements");
        let income_line:Line = Line::new(PlotPoints::new(income_vec)).color(Color32::LIGHT_GREEN).name("Income ");
        let value_line:Line = Line::new(PlotPoints::new(prop_value_vec)).color(Color32::WHITE).name("Property Value");
        let mort_line:Line = Line::new(PlotPoints::new(mort_vec)).color(Color32::RED).name("Loan Liability");

        Plot::new("Graph")
            .view_aspect(4.0)
            .auto_bounds_y()
            .include_y(0.0)
            .legend(Legend::default().position(egui::plot::Corner::LeftTop))
            .show(ui, |p_ui: &mut egui::plot::PlotUi|{p_ui.line(cap_line);p_ui.line(income_line);p_ui.line(value_line);p_ui.line(mort_line);});

        ui.heading(RichText::new("Results").color(Color32::WHITE).font(FontId::proportional(20.0)));
        ui.horizontal(|ui|{
            ui.label(RichText::new("Generated Income").color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new(format_dollar_amount(income_generated)).color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new("Mortgage Liability").color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new(format_dollar_amount(mortgage_liab)).color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new("Property Value").color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new(format_dollar_amount(prop_value)).color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new("Unrealized Gains").color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new(format_dollar_amount(unrealized_gains)).color(Color32::BLACK).font(FontId::proportional(20.0)));
        });
        


    });

}
fn ibfl_precon(years:&mut f64, _interest_rate:&mut f64, mort_rate:&mut f64, price_precon:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_pkg_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, rent:&mut f64, debt_ratio:&mut f64, rent_app:&mut f64, closing_costs:&mut f64)->Vec<([f64;2],[f64;2],[f64;2],[f64;2])>{
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
        let start_months: i32 = local.date_naive().num_days_from_ce();
        let p_1: i32 = first_payment_date.num_days_from_ce();
        let p_2: i32 = second_payment_date.num_days_from_ce();
        let p_3: i32 = third_payment_date.num_days_from_ce();
        let p_4: i32 = fourth_payment_date.num_days_from_ce();
        let p_5: i32 = fifth_payment_date.num_days_from_ce();
        let x1: i32 = (p_1- start_months)/30; // number of months from today until first payment due 
        let x2: i32 = ((p_2- start_months))/30;//number of months from today until second payment
        let x3: i32 =((p_3- start_months))/30;
        let x4: i32 =((p_4- start_months))/30;
        let x5: i32 =((p_5- start_months))/30;
        let mut results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>=Vec::new();
        let mut x: f64 = 0.0;
        let mut rent_revenue_updated: f64 = *rent;
        let mut prop_value: f64 = *price_precon;
        let service_expense: f64 = *service_pkg_price;
        let inside_app_rate:f64 = *app_rate / 12.0;//rate divided by 12 since dealing with months not years
        let mut net_income:f64=0.0;
        let inside_debt_ratio:f64 = *debt_ratio * 0.01;
        let inside_expense_withholding:f64 = *expense_withholding * 0.01;
        let inside_mort_rate:f64 = *mort_rate * 0.01;
        let inside_rent_app:f64 = *rent_app * 0.01;
        while x<=y{
           let mut cap:f64;
            while x<=x5 as f64{
                cap = 0.0;
                if x == x1 as f64{
                    cap = -first_payment_amt;
                }
                if x == x2 as f64{
                    cap = -second_payment_amt;
                }
                if x == x3 as f64{
                    cap = -third_payment_amt;
                }
                if x == x4 as f64{
                    cap = -fourth_payment_amt;
                }
                if x == x5 as f64{
                    cap = -fifth_payment_amt - *closing_costs;
                }
                x+=1.0;
                results.push(([x,cap],[x,0.0],[x,0.0],[x,0.0]));
            }
            cap = 0.0;
            let mortgage_liability:f64 = *price_precon * (inside_debt_ratio);
            let mort_payment:f64 = (mortgage_liability * (inside_mort_rate)) / 12.0;
            let cap_inject:f64;
            if x == x5 as f64+1.0{
                cap_inject = *price_precon * (inside_debt_ratio);
            }else{
                cap_inject = 0.0;
            }
            
            net_income += (rent_revenue_updated * (1.0 - *prop_mgmt) *(1.0-*expense_withholding*0.01)+cap_inject) - (mort_payment);
            if x%12.0 == 0.0{//checks if it is a year to subtract yearly service package cost + increase property value by app_rate and rent by rent_app
                rent_revenue_updated+=rent_revenue_updated * inside_rent_app;//increasing the rent once per year
                net_income += rent_revenue_updated * (*prop_mgmt) * inside_expense_withholding + cap_inject - service_expense - mort_payment;
            }
            prop_value += prop_value * (inside_app_rate*0.01);
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
use chrono::{NaiveDate, DateTime,Local, Datelike};
use egui::{Color32, RichText, FontId, Vec2, plot::{PlotPoints,Line, Legend, Plot}};
use egui_extras;
use super::{product_page::PACKAGES};
use crate::pages::simulation_page::{format_dollar_amount,split_ans};
use std::{fs::File, path::{PathBuf, Display}};
use csv::Writer;
const MARGIN:Vec2 = Vec2{x: 7.0,y: 7.0};

pub fn ibfp_sim_precon(ctx:&egui::Context,
                        years:&mut f64,
                        ibfp_unit_str:&mut String,
                        ibfp_unit_price:&mut f64,
                        selected_services:&mut Option<PACKAGES>,
                        service_pkg_price: &mut f64,
                        prop_mgmt: &mut f64,
                        selected_mgmt:&mut bool, 
                        mort_rate:&mut f64, 
                        first_payment:&mut NaiveDate, 
                        second_payment:&mut NaiveDate,
                        third_payment:&mut NaiveDate, 
                        fourth_payment:&mut NaiveDate, 
                        fifth_payment:&mut NaiveDate, 
                        first_payment_percent:&mut f64, 
                        second_payment_percent:&mut f64,
                        third_payment_percent:&mut f64, 
                        fourth_payment_percent:&mut f64, 
                        fifth_payment_percent:&mut f64,
                        app_rate:&mut f64,
                        occupancy:&mut f64,
                        price_per_night:&mut f64,
                        expense_withholding:&mut f64,
                        debt_ratio:&mut f64,
                        rent_app:&mut f64,
                        closing_costs:&mut f64,
                        
                        ){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
        col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION InBest For Profit").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Parameter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.add(egui::Label::new(RichText::new("Parameters").color(Color32::WHITE).font(FontId::proportional(18.0))));
        ui.separator();
        ui.add(egui::TextEdit::singleline(ibfp_unit_str).hint_text(RichText::new("Enter Price of Unit")));
        if let Ok(price) = ibfp_unit_str.parse::<f64>(){
            *ibfp_unit_price = price;
        }
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(*ibfp_unit_price))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        ui.add(egui::Slider::new(mort_rate,0.0..=10.0).text(RichText::new("Mortgage Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(app_rate,0.0..=10.0).text(RichText::new("Appreciation Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(expense_withholding,0.0..=30.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(price_per_night,120.0..=500.0).text(RichText::new("Price Per Night").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(occupancy,25.0..=99.0).text(RichText::new("Yearly Occupancy").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(debt_ratio,0.0..=100.0).text(RichText::new("Leveraged Ratio").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent_app,0.0..=10.0).text(RichText::new("Price Per Night Increase").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
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
        let ans: Vec<([f64; 2], [f64; 2], [f64; 2], [f64; 2])> = ibfp_precon(years, mort_rate, ibfp_unit_price, first_payment, second_payment, 
            third_payment, fourth_payment, fifth_payment, first_payment_percent, second_payment_percent, third_payment_percent, 
            fourth_payment_percent, fifth_payment_percent, service_pkg_price, prop_mgmt, app_rate, expense_withholding, price_per_night, 
            debt_ratio, rent_app, closing_costs,occupancy,selected_mgmt);
        let cap_vec:Vec<[f64;2]> = split_ans(&ans, 0);
        let income_vec:Vec<[f64;2]> = split_ans(&ans, 1);
        let prop_value_vec: Vec<[f64; 2]> = split_ans(&ans, 2);
        let mort: Vec<[f64; 2]> = split_ans(&ans, 3);
        let income_generated:f64;
        if let Some(total_income) = income_vec.last(){
            income_generated = total_income[1];
        }else{
            income_generated = 0.0;
        }
        let mortgage_liab:f64;
        if let Some(mort) = mort.last(){
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
        let unrealized_gains:f64 = prop_value - mortgage_liab + income_generated;

        let cap_line:Line = Line::new(PlotPoints::new(cap_vec)).color(Color32::LIGHT_RED).name("Capital Requirements");
        let income_line:Line = Line::new(PlotPoints::new(income_vec)).color(Color32::LIGHT_GREEN).name("Income ");
        let value_line:Line = Line::new(PlotPoints::new(prop_value_vec)).color(Color32::WHITE).name("Property Value");
        let mort_line:Line = Line::new(PlotPoints::new(mort)).color(Color32::RED).name("Loan Liability");
    egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
            ui.heading(RichText::new("Graph").color(Color32::WHITE).font(FontId::proportional(20.0)));
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
            ui.label(RichText::new("Holding Net Value").color(Color32::BLACK).font(FontId::proportional(20.0)));
            ui.label(RichText::new(format_dollar_amount(unrealized_gains)).color(Color32::BLACK).font(FontId::proportional(20.0)));
        });
        let download = ui.add(egui::Button::new(RichText::new("Click to Download as CSV").font(FontId::proportional(18.0))));
        if download.clicked(){
            write_csv_precon(ans);
        }
    });
}

pub fn ibfp_precon(years:&mut f64, mort_rate:&mut f64, ibfp_unit_price:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_pkg_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, price_per_night:&mut f64, debt_ratio:&mut f64, rent_app:&mut f64, 
    closing_costs:&mut f64,occupancy:&mut f64,selected_mgmt:&mut bool)->Vec<([f64;2],[f64;2],[f64;2],[f64;2])>{
        let y = *years * 12.0;//total months
        let first_payment_amt:f64 = *ibfp_unit_price * (*first_payment_percent * 0.01);
        let second_payment_amt:f64 = *ibfp_unit_price *(*second_payment_percent * 0.01);
        let third_payment_amt:f64 = *ibfp_unit_price *(*third_payment_percent * 0.01);
        let fourth_payment_amt:f64 = *ibfp_unit_price *(*fourth_payment_percent * 0.01);
        let fifth_payment_amt:f64 = *ibfp_unit_price *(*fifth_payment_percent * 0.01);
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
        let mut rent_revenue_updated: f64 = (*price_per_night) * ((*occupancy * 0.01)*30.42);
        let mut prop_value: f64 = *ibfp_unit_price;
        let service_expense: f64 = *service_pkg_price;
        let inside_app_rate:f64 = *app_rate / 12.0;//rate divided by 12 since dealing with months not years
        let mut net_income:f64=0.0;
        let inside_debt_ratio:f64 = *debt_ratio * 0.01;
        let inside_expense_withholding:f64 = *expense_withholding * 0.01;
        let inside_mort_rate:f64 = *mort_rate * 0.01;
        let inside_rent_app:f64 = *rent_app * 0.01;
        if *selected_mgmt == true{
            *prop_mgmt = 0.35;
        }else{
            *prop_mgmt = 0.0;
        }
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
            let mortgage_liability:f64 = *ibfp_unit_price * (inside_debt_ratio);
            let mort_payment:f64 = (mortgage_liability * (inside_mort_rate)) / 12.0;
            let cap_inject:f64;
            if x == x5 as f64+1.0{
                cap_inject = *ibfp_unit_price * (inside_debt_ratio);
            }else{
                cap_inject = 0.0;
            }
            
            net_income += (rent_revenue_updated*(1.0-*prop_mgmt)  *(1.0-*expense_withholding*0.01)+cap_inject) - (mort_payment);
            if x%12.0 == 0.0{//checks if it is a year to subtract yearly service package cost + increase property value by app_rate and rent by rent_app
                rent_revenue_updated+=rent_revenue_updated * inside_rent_app;//increasing the rent once per year
                net_income += rent_revenue_updated * inside_expense_withholding + cap_inject - service_expense - mort_payment;
            }
            prop_value += prop_value * (inside_app_rate*0.01);
            results.push(([x,cap],[x,net_income],[x,prop_value],[x,mortgage_liability]));
            x+=1.0;
            
        }
        
        
        return results
}
pub fn ibfp_sim_mix(ctx:&egui::Context,
                        years:&mut f64,
                        ibfp_unit_str:&mut String,
                        ibfp_unit_price:&mut f64,
                        selected_services:&mut Option<PACKAGES>,
                        service_pkg_price: &mut f64,
                        prop_mgmt: &mut f64,
                        selected_mgmt:&mut bool, 
                        mort_rate:&mut f64, 
                        first_payment:&mut NaiveDate, 
                        second_payment:&mut NaiveDate,
                        third_payment:&mut NaiveDate, 
                        fourth_payment:&mut NaiveDate, 
                        fifth_payment:&mut NaiveDate, 
                        first_payment_percent:&mut f64, 
                        second_payment_percent:&mut f64,
                        third_payment_percent:&mut f64, 
                        fourth_payment_percent:&mut f64, 
                        fifth_payment_percent:&mut f64,
                        app_rate:&mut f64,
                        occupancy:&mut f64,
                        price_per_night:&mut f64,
                        expense_withholding:&mut f64,
                        debt_ratio:&mut f64,
                        rent_app:&mut f64,
                        closing_costs:&mut f64,
                        ){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
            col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION InBest For Profit").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Parameter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.add(egui::Label::new(RichText::new("Parameters").color(Color32::WHITE).font(FontId::proportional(18.0))));
        ui.separator();
        ui.add(egui::TextEdit::singleline(ibfp_unit_str).hint_text(RichText::new("Enter Price of Unit")));
        if let Ok(price) = ibfp_unit_str.parse::<f64>(){
            *ibfp_unit_price = price;
        }
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(*ibfp_unit_price))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        ui.add(egui::Slider::new(mort_rate,0.0..=10.0).text(RichText::new("Mortgage Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(app_rate,0.0..=10.0).text(RichText::new("Appreciation Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(expense_withholding,0.0..=30.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(price_per_night,120.0..=500.0).text(RichText::new("Price Per Night").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(occupancy,25.0..=99.0).text(RichText::new("Yearly Occupancy").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(debt_ratio,0.0..=100.0).text(RichText::new("Leveraged Ratio").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent_app,0.0..=10.0).text(RichText::new("Price Per Night Increase").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
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
    let ans: Vec<([f64; 2], [f64; 2], [f64; 2], [f64; 2])> = ibfp_mix(years, mort_rate, ibfp_unit_price, first_payment, second_payment, 
        third_payment, fourth_payment, fifth_payment, first_payment_percent, second_payment_percent, third_payment_percent, 
        fourth_payment_percent, fifth_payment_percent, service_pkg_price, prop_mgmt, app_rate, expense_withholding, price_per_night, 
        debt_ratio, rent_app, closing_costs,occupancy,selected_mgmt);
    let cap_vec:Vec<[f64;2]> = split_ans(&ans, 0);
    let income_vec:Vec<[f64;2]> = split_ans(&ans, 1);
    let prop_value_vec: Vec<[f64; 2]> = split_ans(&ans, 2);
    let mort: Vec<[f64; 2]> = split_ans(&ans, 3);
    let income_generated:f64;
    if let Some(total_income) = income_vec.last(){
        income_generated = total_income[1];
    }else{
        income_generated = 0.0;
    }
    let mortgage_liab:f64;
    if let Some(mort) = mort.last(){
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
    let unrealized_gains:f64 = prop_value - mortgage_liab + income_generated;

    let cap_line:Line = Line::new(PlotPoints::new(cap_vec)).color(Color32::LIGHT_RED).name("Capital Requirements");
    let income_line:Line = Line::new(PlotPoints::new(income_vec)).color(Color32::LIGHT_GREEN).name("Income ");
    let value_line:Line = Line::new(PlotPoints::new(prop_value_vec)).color(Color32::WHITE).name("Property Value");
    let mort_line:Line = Line::new(PlotPoints::new(mort)).color(Color32::RED).name("Loan Liability");
    egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
        ui.heading(RichText::new("Graph").color(Color32::WHITE).font(FontId::proportional(20.0)));
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
        ui.label(RichText::new("Holding Net Value").color(Color32::BLACK).font(FontId::proportional(20.0)));
        ui.label(RichText::new(format_dollar_amount(unrealized_gains)).color(Color32::BLACK).font(FontId::proportional(20.0)));
    });
    let download = ui.add(egui::Button::new(RichText::new("Click to Download as CSV").font(FontId::proportional(18.0))));
        if download.clicked(){
            write_csv_mix(ans);
        }
});


}



pub fn ibfp_mix(years:&mut f64, mort_rate:&mut f64, ibfp_unit_price:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_pkg_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, price_per_night:&mut f64, debt_ratio:&mut f64, rent_app:&mut f64, 
    closing_costs:&mut f64,occupancy:&mut f64,selected_mgmt:&mut bool)->Vec<([f64;2],[f64;2],[f64;2],[f64;2])>{
        let y = *years * 12.0;//total months
        let mut x:f64 = 1.0;
        let first_payment_amt:f64 = *ibfp_unit_price * (*first_payment_percent * 0.01);
        let second_payment_amt:f64 = *ibfp_unit_price *(*second_payment_percent * 0.01);
        let third_payment_amt:f64 = *ibfp_unit_price *(*third_payment_percent * 0.01);
        let fourth_payment_amt:f64 = *ibfp_unit_price *(*fourth_payment_percent * 0.01);
        let fifth_payment_amt:f64 = *ibfp_unit_price *(*fifth_payment_percent * 0.01);
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
        let mut ppn: f64 = *price_per_night;
        if *selected_mgmt == true{
            *prop_mgmt = 0.35;
        }else{
            *prop_mgmt = 0.0;
        }
        //main sim logic
        let mut mort_liability:f64 = 0.0;
        let mut net_income:f64 = 0.0;
        let mut rev: f64;
        let mut mort_payment:f64;
        let mut prop_value:f64 = *ibfp_unit_price;
        let mut additional_unit_cost:f64 = *ibfp_unit_price + *closing_costs;
        let mut properties:f64 = 1.0;
        while x<=y{
            //first part of the sim while making precon payments
            let mut cap:f64 = 0.0;
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
            additional_unit_cost = additional_unit_cost + ((additional_unit_cost * ((*app_rate *0.01) / 12.0)));
            //starting after precon is bought
            //first take mortgage out on property:
            let mut  cap_inject:f64 = 0.0;
            let mut first_mort:f64 = 0.0;
            if x == x5 as f64 +1.0{
                cap_inject = *ibfp_unit_price * (*debt_ratio * 0.01);
                first_mort = *ibfp_unit_price * (*debt_ratio * 0.01);
            }
            let mut mort_add:f64 = 0.0;
            let mut add_prop_value:f64 = 0.0;
            if net_income >= additional_unit_cost * (1.0- *debt_ratio * 0.01) + *closing_costs + ((additional_unit_cost * (*debt_ratio * 0.01)* *mort_rate)/12.0){
                mort_add = additional_unit_cost * (*debt_ratio * 0.01);
                net_income = net_income - additional_unit_cost * (1.0- *debt_ratio * 0.01);
                add_prop_value = additional_unit_cost;
                properties+=1.0;
            }
            if x%12.0 == 0.0{
                net_income = net_income - (*service_pkg_price * properties);
                ppn = ppn + ppn*(*rent_app * 0.01);
            }
            mort_liability = mort_liability + first_mort + mort_add;
            mort_payment = (mort_liability * (*mort_rate * 0.01))/12.0;
            rev = ((ppn * (*occupancy * 0.01)*30.41)*(1.0- *prop_mgmt)*(1.0- *expense_withholding*0.01) - mort_payment)*properties;
            net_income = net_income + rev + cap_inject;
            prop_value = prop_value + (prop_value * ((*app_rate * 0.01)/12.0)) + add_prop_value;
            results.push(([x,cap],[x,net_income ],[x,prop_value],[x,mort_liability]));
            x+=1.0;
        }

        return results
}



pub fn ibfp_mkt_sim(ctx:&egui::Context,
                        years:&mut f64,
                        ibfp_unit_str:&mut String,
                        ibfp_unit_price:&mut f64,
                        selected_services:&mut Option<PACKAGES>,
                        service_pkg_price: &mut f64,
                        prop_mgmt: &mut f64,
                        selected_mgmt:&mut bool, 
                        mort_rate:&mut f64, 
                        app_rate:&mut f64,
                        occupancy:&mut f64,
                        price_per_night:&mut f64,
                        expense_withholding:&mut f64,
                        debt_ratio:&mut f64,
                        rent_app:&mut f64,
                        closing_costs:&mut f64,
                        ibfp_investment_amount: &mut f64,
                        ibfp_investment_amount_str: &mut String,
                        ){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
            col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION InBest For Profit").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Parameter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.add(egui::Label::new(RichText::new("Parameters").color(Color32::WHITE).font(FontId::proportional(18.0))));
        ui.separator();
        ui.add(egui::TextEdit::singleline(ibfp_unit_str).hint_text(RichText::new("Enter Price of Unit")));
        if let Ok(price) = ibfp_unit_str.parse::<f64>(){
            *ibfp_unit_price = price;
        }
        ui.add(egui::TextEdit::singleline(ibfp_investment_amount_str).hint_text(RichText::new("Enter Investment")));
        if let Ok(initial) = ibfp_investment_amount_str.parse::<f64>(){
            *ibfp_investment_amount = initial;
        }
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(*ibfp_unit_price))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.label(RichText::new(format!("Investment Amount: {}", format_dollar_amount(*ibfp_investment_amount))).color(Color32::WHITE).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        ui.add(egui::Slider::new(mort_rate,0.0..=10.0).text(RichText::new("Mortgage Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(app_rate,0.0..=10.0).text(RichText::new("Appreciation Rate").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(expense_withholding,0.0..=30.0).text(RichText::new("Expense Withholding").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(price_per_night,120.0..=500.0).text(RichText::new("Price Per Night").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(occupancy,25.0..=99.0).text(RichText::new("Yearly Occupancy").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(debt_ratio,0.0..=100.0).text(RichText::new("Leveraged Ratio").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(rent_app,0.0..=10.0).text(RichText::new("Price Per Night Increase").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(1));
        ui.add(egui::Slider::new(closing_costs, 0.0..=100_000.0).text(RichText::new("Closing Costs").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));
        
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
    let ans: Vec<([f64; 2], [f64; 2], [f64; 2], [f64; 2])> = ibfp_mkt(years, mort_rate, ibfp_unit_price, service_pkg_price, prop_mgmt, app_rate, expense_withholding, price_per_night, 
        debt_ratio, rent_app, closing_costs,occupancy,selected_mgmt,ibfp_investment_amount);
    let properties_vec:Vec<[f64;2]> = split_ans(&ans, 0);
    let income_vec:Vec<[f64;2]> = split_ans(&ans, 1);
    let prop_value_vec: Vec<[f64; 2]> = split_ans(&ans, 2);
    let mort: Vec<[f64; 2]> = split_ans(&ans, 3);
    let properties:f64;
    if let Some(props) = properties_vec.last(){
        properties = props[1];
    }else{
        properties = 0.0;
    }
    let income_generated:f64;
    if let Some(total_income) = income_vec.last(){
        income_generated = total_income[1];
    }else{
        income_generated = 0.0;
    }
    let mortgage_liab:f64;
    if let Some(mort) = mort.last(){
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
    let unrealized_gains:f64 = prop_value - mortgage_liab + income_generated;

    let cap_line:Line = Line::new(PlotPoints::new(properties_vec)).color(Color32::LIGHT_RED).name("Properties");
    let income_line:Line = Line::new(PlotPoints::new(income_vec)).color(Color32::LIGHT_GREEN).name("Income ");
    let value_line:Line = Line::new(PlotPoints::new(prop_value_vec)).color(Color32::WHITE).name("Property Value");
    let mort_line:Line = Line::new(PlotPoints::new(mort)).color(Color32::RED).name("Loan Liability");
    egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
        ui.heading(RichText::new("Graph").color(Color32::WHITE).font(FontId::proportional(20.0)));
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
        ui.label(RichText::new("Holding Net Value").color(Color32::BLACK).font(FontId::proportional(20.0)));
        ui.label(RichText::new(format_dollar_amount(unrealized_gains)).color(Color32::BLACK).font(FontId::proportional(20.0)));
    });
    ui.label(RichText::new("Properties").color(Color32::BLACK).font(FontId::proportional(20.0)));
    ui.label(RichText::new(properties.to_string()).color(Color32::BLACK).font(FontId::proportional(20.0)));
    let download = ui.add(egui::Button::new(RichText::new("Click to Download as CSV").font(FontId::proportional(18.0))));
        if download.clicked(){
            write_csv_mkt(ans);
        }
});


}

pub fn ibfp_mkt(years:&mut f64,mort_rate:&mut f64,ibfp_unit_price:&mut f64, service_pkg_price: &mut f64, prop_mgmt: &mut f64, app_rate: &mut f64, expense_withholding:&mut f64, price_per_night:&mut f64,
debt_ratio:&mut f64, rent_app:&mut f64, closing_costs:&mut f64, occupancy:&mut f64, selected_mgmt:&mut bool,ibfp_investment_amount: &mut f64)->Vec<([f64; 2], [f64; 2], [f64; 2], [f64; 2])>{
    let y: f64 = *years * 12.0;//total months
    let mut x:f64 = 1.0;
    let mut results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>=Vec::new();
    let mut ppn: f64 = *price_per_night;
    if *selected_mgmt == true{
        *prop_mgmt = 0.35;
    }else{
        *prop_mgmt = 0.0;
    }
    //main sim logic
    let mut mort_liability:f64 = 0.0;
    let mut net_income:f64 = 0.0;
    let mut rev: f64;
    let mut mort_payment:f64;
    let mut mkt_unit:f64 = *ibfp_unit_price;
    let mut properties:f64 = (*ibfp_investment_amount / (*ibfp_unit_price+*closing_costs)).floor(); // whole number ie 1M investment / 450K unit price = 2
    let excess_cap:f64 = *ibfp_investment_amount % (*ibfp_unit_price + *closing_costs);//remainder after buying first units ie 1M investment / 450K unit price = 100K
    let mut prop_value:f64 = properties * *ibfp_unit_price;
    while x<=y{
        //initializing the interior vars to 0 for each loop;
        let mut mort_add:f64 = 0.0;
        let mut add_prop_value:f64 = 0.0;
        //growing unit price monthly based on the appreciation rate/12;
        mkt_unit = mkt_unit + ((mkt_unit * ((*app_rate *0.01) / 12.0)));
        let mut initial_mort: f64 = 0.0;
        let mut cap_inject:f64 = 0.0;
        if x == 1.0{
            initial_mort = mkt_unit * (*debt_ratio * 0.01);
            cap_inject = mkt_unit * (*debt_ratio * 0.01) + excess_cap;
            
        }
        if net_income >= (mkt_unit * (1.0- *debt_ratio * 0.01) + *closing_costs) + ((mkt_unit * (*debt_ratio * 0.01)* *mort_rate)/12.0){//net income is greater than out of pocket cash + closing costs + first mortgage payment
            mort_add = mkt_unit * (*debt_ratio * 0.01);
            net_income = net_income - mkt_unit * (1.0- *debt_ratio * 0.01);
            add_prop_value = mkt_unit;
            properties+=1.0;
        }
        if x%12.0 == 0.0{
            net_income = net_income - (*service_pkg_price * properties);
            ppn = ppn + ppn*(*rent_app * 0.01);
        }
        mort_liability = mort_liability + mort_add + initial_mort;
        mort_payment = (mort_liability * (*mort_rate * 0.01))/12.0;
        rev = ((ppn * (*occupancy * 0.01)*30.41)*(1.0- *prop_mgmt)*(1.0- *expense_withholding*0.01) - mort_payment)*properties;
        net_income = net_income + rev + cap_inject;
        prop_value = prop_value + (prop_value * ((*app_rate * 0.01)/12.0)) + add_prop_value;
        results.push(([x,properties],[x,net_income ],[x,prop_value],[x,mort_liability]));
        x+=1.0;
    }
    return results
}
fn write_csv_mkt(results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>){
    let mut properties:Vec<[f64;2]> = Vec::new();
    for(i,_,_,_) in &results{
        properties.push(*i);
    }
    let mut income_vec:Vec<[f64;2]> = Vec::new();
    for(_,i,_,_)in &results{
        income_vec.push(*i);
    }
    let mut port_gross_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,i,_) in &results{
        port_gross_vec.push(*i);
    }
    let mut mort_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,_,i) in &results{
        mort_vec.push(*i);
    }
    let properties_total:Vec<String> = properties.iter()
        .map(|&[_,i]| i.to_string())
        .collect();
    let income_earned:Vec<String> = income_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let income_earned_f:Vec<f64> = income_vec.iter()
        .map(|&[_,i]|i)
        .collect();
    let portflio_value:Vec<String> = port_gross_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let portflio_value_f:Vec<f64> = port_gross_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab_f:Vec<f64> = mort_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab:Vec<String> = mort_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let mut net_port_value:Vec<f64> = portflio_value_f.iter()
        .zip(&mort_liab_f)
        .map(|(&i, &j)| i - j)
        .collect();
    net_port_value = net_port_value.iter().zip(&income_earned_f).map(|(&i, &j)| i + j).collect();
    let net_port_out:Vec<String> = net_port_value.iter()
        .map(|&i|format_dollar_amount(i))
        .collect();
    let months:Vec<String> = mort_vec.iter()
        .map(|&[i,_]| i.to_string())
        .collect();

    let desktop: PathBuf = match dirs::desktop_dir(){
        Some(path)=>path,
        None=>{
            println!("No Directory called Desktop found on system");
            return;
        }
    };
    let file_path:PathBuf = [desktop.to_str().unwrap(), "InBest For Profit MKT Results.csv"].iter().collect();
    let display:Display = file_path.display();
    let file = match File::create(&file_path){
        Err(why) => panic!("Couldn't Create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut writer = Writer::from_writer(file);
    let headers:Vec<String> = vec!["Months".to_string(),"Gross Value".to_string(),"Mortgage Value".to_string(),"Net Value".to_string(), "Properties Owned".to_string(),"Income".to_string()];
    let string_data_vec:Vec<Vec<String>> = vec![months, portflio_value,mort_liab,net_port_out,properties_total,income_earned];
    writer.serialize(headers).unwrap();
    
    for i in 0..string_data_vec[0].len(){
        let row :Vec<&str> = string_data_vec.iter().map(|a| a[i].as_str()).collect();
        writer.serialize(row).unwrap();    

    }
    writer.flush().expect("Error Writing");

}

fn write_csv_precon(results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>){
    let mut cap_expense:Vec<[f64;2]> = Vec::new();
    for(i,_,_,_) in &results{
        cap_expense.push(*i);
    }
    let mut income_vec:Vec<[f64;2]> = Vec::new();
    for(_,i,_,_)in &results{
        income_vec.push(*i);
    }
    let mut port_gross_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,i,_) in &results{
        port_gross_vec.push(*i);
    }
    let mut mort_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,_,i) in &results{
        mort_vec.push(*i);
    }
    let cap_exp_vec:Vec<String> = cap_expense.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let income_earned:Vec<String> = income_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let income_earned_f:Vec<f64> = income_vec.iter()
        .map(|&[_,i]|i)
        .collect();
    let portflio_value:Vec<String> = port_gross_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let portflio_value_f:Vec<f64> = port_gross_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab_f:Vec<f64> = mort_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab:Vec<String> = mort_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let mut net_port_value:Vec<f64> = portflio_value_f.iter()
        .zip(&mort_liab_f)
        .map(|(&i, &j)| i - j)
        .collect();
    net_port_value = net_port_value.iter().zip(&income_earned_f).map(|(&i, &j)| i + j).collect();
    let net_port_out:Vec<String> = net_port_value.iter()
        .map(|&i|format_dollar_amount(i))
        .collect();
    let months:Vec<String> = mort_vec.iter()
        .map(|&[i,_]| i.to_string())
        .collect();

    let desktop: PathBuf = match dirs::desktop_dir(){
        Some(path)=>path,
        None=>{
            println!("No Directory called Desktop found on system");
            return;
        }
    };
    let file_path:PathBuf = [desktop.to_str().unwrap(), "InBest For Profit Precon Results.csv"].iter().collect();
    let display:Display = file_path.display();
    let file = match File::create(&file_path){
        Err(why) => panic!("Couldn't Create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut writer = Writer::from_writer(file);
    let headers:Vec<String> = vec!["Months".to_string(),"Gross Value".to_string(),"Mortgage Value".to_string(),"Net Value".to_string(), "Capital Expenditures".to_string(),"Income".to_string()];
    let string_data_vec:Vec<Vec<String>> = vec![months, portflio_value,mort_liab,net_port_out,cap_exp_vec,income_earned];
    writer.serialize(headers).unwrap();
    
    for i in 0..string_data_vec[0].len(){
        let row :Vec<&str> = string_data_vec.iter().map(|a| a[i].as_str()).collect();
        writer.serialize(row).unwrap();    

    }
    writer.flush().expect("Error Writing");

}
fn write_csv_mix(results:Vec<([f64;2],[f64;2],[f64;2],[f64;2])>){
    let mut cap_expense:Vec<[f64;2]> = Vec::new();
    for(i,_,_,_) in &results{
        cap_expense.push(*i);
    }
    let mut income_vec:Vec<[f64;2]> = Vec::new();
    for(_,i,_,_)in &results{
        income_vec.push(*i);
    }
    let mut port_gross_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,i,_) in &results{
        port_gross_vec.push(*i);
    }
    let mut mort_vec:Vec<[f64;2]> = Vec::new();
    for(_,_,_,i) in &results{
        mort_vec.push(*i);
    }
    let cap_exp_vec:Vec<String> = cap_expense.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let income_earned:Vec<String> = income_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let income_earned_f:Vec<f64> = income_vec.iter()
        .map(|&[_,i]|i)
        .collect();
    let portflio_value:Vec<String> = port_gross_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let portflio_value_f:Vec<f64> = port_gross_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab_f:Vec<f64> = mort_vec.iter()
        .map(|&[_,i]| i)
        .collect();
    let mort_liab:Vec<String> = mort_vec.iter()
        .map(|&[_,i]| format_dollar_amount(i))
        .collect();
    let mut net_port_value:Vec<f64> = portflio_value_f.iter()
        .zip(&mort_liab_f)
        .map(|(&i, &j)| i - j)
        .collect();
    net_port_value = net_port_value.iter().zip(&income_earned_f).map(|(&i, &j)| i + j).collect();
    let net_port_out:Vec<String> = net_port_value.iter()
        .map(|&i|format_dollar_amount(i))
        .collect();
    let months:Vec<String> = mort_vec.iter()
        .map(|&[i,_]| i.to_string())
        .collect();

    let desktop: PathBuf = match dirs::desktop_dir(){
        Some(path)=>path,
        None=>{
            println!("No Directory called Desktop found on system");
            return;
        }
    };
    let file_path:PathBuf = [desktop.to_str().unwrap(), "InBest For Profit Mix Results.csv"].iter().collect();
    let display:Display = file_path.display();
    let file = match File::create(&file_path){
        Err(why) => panic!("Couldn't Create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut writer = Writer::from_writer(file);
    let headers:Vec<String> = vec!["Months".to_string(),"Gross Value".to_string(),"Mortgage Value".to_string(),"Net Value".to_string(), "Capital Expenditures".to_string(),"Income".to_string()];
    let string_data_vec:Vec<Vec<String>> = vec![months, portflio_value,mort_liab,net_port_out,cap_exp_vec,income_earned];
    writer.serialize(headers).unwrap();
    
    for i in 0..string_data_vec[0].len(){
        let row :Vec<&str> = string_data_vec.iter().map(|a| a[i].as_str()).collect();
        writer.serialize(row).unwrap();    

    }
    writer.flush().expect("Error Writing");

}
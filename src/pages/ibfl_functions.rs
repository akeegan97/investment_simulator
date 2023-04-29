use chrono::{NaiveDate, DateTime,Local, Datelike};


pub fn ibfl_precon(years:&mut f64, _interest_rate:&mut f64, mort_rate:&mut f64, price_precon:&mut f64, first_payment:&mut NaiveDate, second_payment:&mut NaiveDate, third_payment:&mut NaiveDate,
    fourth_payment:&mut NaiveDate, fifth_payment:&mut NaiveDate, first_payment_percent:&mut f64, second_payment_percent:&mut f64, third_payment_percent:&mut f64, fourth_payment_percent:&mut f64, fifth_payment_percent:&mut f64,
    service_pkg_price:&mut f64, prop_mgmt: &mut f64,app_rate: &mut f64, expense_withholding:&mut f64, rent:&mut f64, debt_ratio:&mut f64, rent_app:&mut f64, closing_costs:&mut f64,selected_mgmt:&mut bool)->Vec<([f64;2],[f64;2],[f64;2],[f64;2])>{
        let y: f64 = *years * 12.0;//total months
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
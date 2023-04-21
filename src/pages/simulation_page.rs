use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2, SelectableLabel};
use egui_extras::RetainedImage;

use super::product_page::LISTINGS;

pub fn ibfl_sim(ctx:&egui::Context,years:&mut f64,selected_ibfl:&mut Option<LISTINGS>){
    egui::TopBottomPanel::top("Heading").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
        col[1].add(egui::Label::new(RichText::new("FINANZ BUTIK SIMULATION").color(Color32::WHITE).font(FontId::proportional(24.0))));
        });
    });
    egui::SidePanel::left("Paramter Panel").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
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
        ui.label(RichText::new(format!("Price of Unit: {}",format_dollar_amount(picked_listing))).color(Color32::GREEN).font(FontId::proportional(18.0)));
        ui.add(egui::Slider::new(years,0.0..=15.0).text(RichText::new("Years").color(Color32::WHITE)).clamp_to_range(false).fixed_decimals(0));



    });
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
use eframe::egui;
use egui::{Color32, RichText, FontId};
use egui_extras::RetainedImage;

#[derive(PartialEq)]
pub enum Sectors{
    InbestForLife,
    InbestForProfit,
    InbestFund,
}

pub fn show(ctx:&egui::Context, selected_product: &mut Option<Sectors>){
    let margin = egui::vec2(7.0, 7.0);
    egui::TopBottomPanel::top("Sector Selector")
        .frame(egui::Frame::default()
            .inner_margin(margin)
            .fill(Color32::LIGHT_BLUE)
        ).show(ctx,|ui|{
            ui.columns(3, |col|{
                let inbest_for_life = *selected_product == Some(Sectors::InbestForLife);
                let ibfl_logo:RetainedImage = RetainedImage::from_image_bytes("Ibfl", include_bytes!("../../assets/INBFL.webp")).unwrap();
                col[0].add(egui::Image::new(ibfl_logo.texture_id(ctx),ibfl_logo.size_vec2()));
                if col[0].add(egui::SelectableLabel::new(inbest_for_life,RichText::new(IBFL)
                    .color(Color32::WHITE)
                    .font(FontId::proportional(18.0))))
                        .clicked(){
                            *selected_product = Some(Sectors::InbestForLife);
                        }
                let inbest_for_profit = *selected_product == Some(Sectors::InbestForProfit);
                let ibfp_logo:RetainedImage = RetainedImage::from_image_bytes("Ibfl", include_bytes!("../../assets/INBFP.webp")).unwrap();
                col[1].add(egui::Image::new(ibfp_logo.texture_id(ctx),ibfp_logo.size_vec2()));
                if col[1].add(egui::SelectableLabel::new(inbest_for_profit,RichText::new(IBFP)
                    .color(Color32::WHITE)
                    .font(FontId::proportional(18.0))))
                        .clicked(){
                            *selected_product = Some(Sectors::InbestForProfit);
                        }
                let inbest_fund = *selected_product == Some(Sectors::InbestFund);
                let ibfund_logo:RetainedImage = RetainedImage::from_image_bytes("Ibfl", include_bytes!("../../assets/INBFund.webp")).unwrap();
                col[2].add(egui::Image::new(ibfund_logo.texture_id(ctx),ibfund_logo.size_vec2()));
                if col[2].add(egui::SelectableLabel::new(inbest_fund, RichText::new(FUND)
                    .color(Color32::WHITE)
                    .font(FontId::proportional(18.0))))
                        .clicked(){
                            *selected_product = Some(Sectors::InbestFund);
                        }
            });
        });
}

pub const IBFL :&str= "
InBest For Life: 
This is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc";

pub const IBFP: &str= "
InBest For Profit:
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc";

pub const FUND :&str= "
InBest Fund:
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc
this is filler text etc etc etc";
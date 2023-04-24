use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2, SelectableLabel};
use egui_extras::RetainedImage;

#[derive(PartialEq)]
pub enum LISTINGS {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
#[derive(PartialEq)]
pub enum PACKAGES{
    Low,
    Med,
    High,
}
#[derive(PartialEq)]
pub enum IbfpType{
    Precon,
    Mix,
    Mkt,
}
#[derive(PartialEq)]
pub enum Fund{
    Core,
    Plus,
    FixedIncome,
}

pub fn inbest_for_life(ctx:&egui::Context, selected_ibfl: &mut Option<LISTINGS>, selected_package: &mut Option<PACKAGES>,selected_mgmt: &mut bool){
    egui::TopBottomPanel::top("Infomation tab")
        .frame(egui::Frame::default()
            .fill(Color32::LIGHT_BLUE)
            .inner_margin(MARGIN))
                .show(ctx, |ui|{
                    ui.columns(3, |col|{
                        let ibfl_logo:RetainedImage = RetainedImage::from_image_bytes("Ibfl", include_bytes!("../../assets/INBFL.webp")).unwrap();
                        col[1].add(egui::Image::new(ibfl_logo.texture_id(ctx),ibfl_logo.size_vec2()));
                    });
    });
    egui::TopBottomPanel::top("Listings/Offerings")
        .frame(egui::Frame::default()
            .fill(Color32::LIGHT_BLUE)
            .inner_margin(MARGIN))
                .show(ctx,|ui|{
                    ui.columns(5, |col|{
                        col[0].add(egui::Label::new(RichText::new("Listing #1 Image").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[0].add(egui::Label::new(RichText::new("Price of Listing").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[0].add(egui::Label::new(RichText::new("Listing Information").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        let l_1 = *selected_ibfl == Some(LISTINGS::First);
                        let first=  col[0].add(egui::SelectableLabel::new(l_1,RichText::new( "Select").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if first.clicked(){
                            *selected_ibfl = Some(LISTINGS::First);}
                        if first.double_clicked(){
                            *selected_ibfl = None;
                        }
                        col[1].add(egui::Label::new(RichText::new("Listing #2 Image").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[1].add(egui::Label::new(RichText::new("Price of Listing").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[1].add(egui::Label::new(RichText::new("Listing Information").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        let l_2 = *selected_ibfl == Some(LISTINGS::Second);
                        let second=  col[1].add(egui::SelectableLabel::new(l_2,RichText::new( "Select").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if second.clicked(){
                            *selected_ibfl = Some(LISTINGS::Second);}
                        if second.double_clicked(){
                            *selected_ibfl = None;
                        }
                        col[2].add(egui::Label::new(RichText::new("Listing #3 Image").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[2].add(egui::Label::new(RichText::new("Price of Listing").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[2].add(egui::Label::new(RichText::new("Listing Information").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        let l_3 = *selected_ibfl == Some(LISTINGS::Third);
                        let third=  col[2].add(egui::SelectableLabel::new(l_3,RichText::new( "Select").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if third.clicked(){
                            *selected_ibfl = Some(LISTINGS::Third);}
                        if third.double_clicked(){
                            *selected_ibfl = None;
                        }
                        col[3].add(egui::Label::new(RichText::new("Listing #4 Image").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[3].add(egui::Label::new(RichText::new("Price of Listing").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[3].add(egui::Label::new(RichText::new("Listing Information").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        let l_4 = *selected_ibfl == Some(LISTINGS::Fourth);
                        let fourth=  col[3].add(egui::SelectableLabel::new(l_4,RichText::new( "Select").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if fourth.clicked(){
                            *selected_ibfl = Some(LISTINGS::Fourth);}
                        if fourth.double_clicked(){
                            *selected_ibfl = None;
                        }
                        col[4].add(egui::Label::new(RichText::new("Listing #5 Image").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[4].add(egui::Label::new(RichText::new("Price of Listing").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        col[4].add(egui::Label::new(RichText::new("Listing Information").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        let l_5 = *selected_ibfl == Some(LISTINGS::Fifth);
                        let fifth=  col[4].add(egui::SelectableLabel::new(l_5,RichText::new( "Select").color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if fifth.clicked(){
                            *selected_ibfl = Some(LISTINGS::Fifth);}
                        if fifth.double_clicked(){
                            *selected_ibfl = None;
                        }
                    });
    });
    egui::TopBottomPanel::top("Service Packages")
        .frame(egui::Frame::default()
            .inner_margin(MARGIN)
            .fill(Color32::LIGHT_BLUE))
                .show(ctx,|ui|{
                    ui.columns(4, |col|{//selecting service packages 
                        let low = *selected_package == Some(PACKAGES::Low);
                        let pkg_1 = col[0].add(egui::SelectableLabel::new(low, RichText::new(PACKAGE_1).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_1.clicked(){
                            *selected_package = Some(PACKAGES::Low);
                        }
                        if pkg_1.double_clicked(){
                            *selected_package = None;
                        }
                        let med = *selected_package == Some(PACKAGES::Med);
                        let pkg_2 = col[1].add(egui::SelectableLabel::new(med, RichText::new(PACKAGE_2).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_2.clicked(){
                            *selected_package = Some(PACKAGES::Med);
                        }
                        if pkg_2.double_clicked(){
                            *selected_package = None;
                        }
                        let high = *selected_package == Some(PACKAGES::High);
                        let pkg_3 = col[2].add(egui::SelectableLabel::new(high, RichText::new(PACKAGE_3).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_3.clicked(){
                            *selected_package = Some(PACKAGES::High);
                        }
                        if pkg_3.double_clicked(){
                            *selected_package = None;
                        }
                        let p_mgmt = *selected_mgmt;
                        let prop_mgmt = col[3].add(egui::SelectableLabel::new(p_mgmt, RichText::new(PROP_MGMT).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if prop_mgmt.clicked(){
                            *selected_mgmt = true;
                        }
                        if prop_mgmt.double_clicked(){
                            *selected_mgmt = false;
                        }
                    });
    });       

    egui::CentralPanel::default()
        .frame(egui::Frame::default()
            .fill(Color32::LIGHT_BLUE)
            .inner_margin(MARGIN))
                .show(ctx,|_ui|{
    });
}

pub fn inbest_for_profit(ctx:&egui::Context,selected_ibfp:&mut Option<IbfpType>, selected_package: &mut Option<PACKAGES>, selected_mgmt: &mut bool){
    egui::TopBottomPanel::top("Header Image").frame(egui::Frame::default()
        .inner_margin(MARGIN)
        .fill(Color32::LIGHT_BLUE))
        .show(ctx,|ui|{
            ui.columns(3,|col|{
                let ibfp_logo:RetainedImage = RetainedImage::from_image_bytes("Ibpl", include_bytes!("../../assets/INBFP.webp")).unwrap();
                col[1].add(egui::Image::new(ibfp_logo.texture_id(ctx),ibfp_logo.size_vec2()));
            });
    });
    egui::TopBottomPanel::top("Offerings Area").frame(egui::Frame::default()
        .inner_margin(MARGIN)
        .fill(Color32::LIGHT_BLUE))
        .show(ctx,|ui|{
            ui.columns(3, |col|{
                let precon = *selected_ibfp == Some(IbfpType::Precon);
                let pc = col[0].add(SelectableLabel::new(precon, RichText::new(PRECON).color(Color32::WHITE).font(FontId::proportional(18.0))));
                if pc.clicked(){
                    *selected_ibfp = Some(IbfpType::Precon);
                }
                if pc.double_clicked(){
                    *selected_ibfp = None;
                }
                let mkt = *selected_ibfp == Some(IbfpType::Mkt);
                let mk = col[1].add(SelectableLabel::new(mkt,RichText::new(MKT).color(Color32::WHITE).font(FontId::proportional(18.0))));
                if mk.clicked(){
                    *selected_ibfp = Some(IbfpType::Mkt);
                }
                if mk.double_clicked(){
                    *selected_ibfp = None;
                }
                let mix = *selected_ibfp == Some(IbfpType::Mix);
                let mixed = col[2].add(SelectableLabel::new(mix,RichText::new(MIX).color(Color32::WHITE).font(FontId::proportional(18.0))));
                if mixed.clicked(){
                    *selected_ibfp = Some(IbfpType::Mix);
                }
                if mixed.double_clicked(){
                    *selected_ibfp = None;
                }
            });
    });
    egui::TopBottomPanel::top("Service Packages")
        .frame(egui::Frame::default()
            .inner_margin(MARGIN)
            .fill(Color32::LIGHT_BLUE))
                .show(ctx,|ui|{
                    ui.columns(4, |col|{//selecting service packages 
                        let low = *selected_package == Some(PACKAGES::Low);
                        let pkg_1 = col[0].add(egui::SelectableLabel::new(low, RichText::new(PACKAGE_1).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_1.clicked(){
                            *selected_package = Some(PACKAGES::Low);
                        }
                        if pkg_1.double_clicked(){
                            *selected_package = None;
                        }
                        let med = *selected_package == Some(PACKAGES::Med);
                        let pkg_2 = col[1].add(egui::SelectableLabel::new(med, RichText::new(PACKAGE_2).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_2.clicked(){
                            *selected_package = Some(PACKAGES::Med);
                        }
                        if pkg_2.double_clicked(){
                            *selected_package = None;
                        }
                        let high = *selected_package == Some(PACKAGES::High);
                        let pkg_3 = col[2].add(egui::SelectableLabel::new(high, RichText::new(PACKAGE_3).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if pkg_3.clicked(){
                            *selected_package = Some(PACKAGES::High);
                        }
                        if pkg_3.double_clicked(){
                            *selected_package = None;
                        }
                        let p_mgmt = *selected_mgmt;
                        let prop_mgmt = col[3].add(egui::SelectableLabel::new(p_mgmt, RichText::new(PROP_MGMT).color(Color32::WHITE).font(FontId::proportional(18.0))));
                        if prop_mgmt.clicked(){
                            *selected_mgmt = true;
                        }
                        if prop_mgmt.double_clicked(){
                            *selected_mgmt = false;
                        }
                    });
    });       

    egui::CentralPanel::default()
        .frame(egui::Frame::default()
            .fill(Color32::LIGHT_BLUE)
            .inner_margin(MARGIN))
                .show(ctx,|_ui|{
    });

}

pub fn inbest_fund(ctx:&egui::Context, selected_fund: &mut Option<Fund>){
    egui::TopBottomPanel::top("Fund Logo").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx, |ui|{
        ui.columns(3,|col|{
            let ib_fund:RetainedImage = RetainedImage::from_image_bytes("Ib_Fund", include_bytes!("../../assets/INBFund.webp")).unwrap();
            col[1].add(egui::Image::new(ib_fund.texture_id(ctx),ib_fund.size_vec2()));
        });
    });
    egui::TopBottomPanel::top("Offerings of the Fund").frame(egui::Frame::default().fill(Color32::LIGHT_BLUE).inner_margin(MARGIN)).show(ctx,|ui|{
        ui.columns(3,|col|{
            let core = *selected_fund == Some(Fund::Core);
            let corcol = col[0].add(SelectableLabel::new(core,RichText::new(CORE).color(Color32::WHITE).font(FontId::proportional(18.0))));
            if corcol.clicked(){
                *selected_fund = Some(Fund::Core);
            }
            if corcol.double_clicked(){
                *selected_fund = None;
            }
            let plus = *selected_fund == Some(Fund::Plus);
            let pluscol = col[1].add(SelectableLabel::new(plus,RichText::new(PLUS).color(Color32::WHITE).font(FontId::proportional(18.0))));
            if pluscol.clicked(){
                *selected_fund = Some(Fund::Plus);
            }
            if pluscol.double_clicked(){
                *selected_fund = None;
            }
            let fi = *selected_fund == Some(Fund::FixedIncome);
            let ficol = col[2].add(SelectableLabel::new(fi,RichText::new(FIXED).color(Color32::WHITE).font(FontId::proportional(18.0))));
            if ficol.clicked(){
                *selected_fund = Some(Fund::FixedIncome);
            }
            if ficol.double_clicked(){
                *selected_fund = None;
            }
        });
    });
}

const MARGIN:Vec2 = Vec2{x: 7.0,y: 7.0};
const PACKAGE_1:&str = "
Service Package 1                   $5,000USD
Services Below
-
-
-
";
const PACKAGE_2:&str="
Service Package 2                   $7,000USD
Services Below
-
-
-
";
const PACKAGE_3:&str="
Service Package 3                   $10,000USD
Services Below
-
-
-
";
const PROP_MGMT:&str="
Property Management Service         35% of Rent
-
-
-
-
";
const PRECON:&str="
Preconstruction
Units that are newly built 
and payments follow a payment schedule
that breaks the sale price into 
smaller payments paid in advance
";
const MKT:&str="
Open Market
Units that are on the open
market and available for purchase
these units are available to be 
purchased with all equity or via 
financing through a mortgage any 
capital generated is used to purchase 
additional units";
const MIX:&str="
A mixed strategy comprises of 
starting with a preconstruction unit
following a payment schedule and the
capital that is generated from the 
unit is used to purchase open
market units to build a larger portfolio 
";

const CORE:&str="
InBest Fund Core:
Core offerings
-
-
-
-
";
const PLUS:&str="
InBest Fund Plus:
Plus offerings
-
-
-
-
";
const FIXED:&str="
InBest Fund Fixed Income:
FI offerings
-
-
-
-
";
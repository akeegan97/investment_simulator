
use eframe::egui;
use egui::{Margin,Color32, RichText};


//use egui_extras::RetainedImage;
use pages::{main_page::Sectors, product_page::{LISTINGS, PACKAGES,IbfpType, Fund}};
mod pages{
    pub mod main_page;
    pub mod product_page;
    pub mod simulation_page;
}


//fn to set fonts for the text style
fn setup(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/TimesNewRoman.ttf"
        )),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
    
}
fn main()->Result<(),eframe::Error>{
    let options = eframe::NativeOptions{
        initial_window_size:Some(egui::Vec2 { x: 1800.0, y: 1400.0 }),
        ..Default::default()
    };
    eframe::run_native("FB_SIMS", 
    options, 
    Box::new(|cc|Box::new(Sim::new(cc)))
    )
}
#[derive(PartialEq)]
enum Page{
    Main,
    Products,
    Simulate,
}
struct Sim{
    selected_page: Option<Page>,
    product: Option<Sectors>,
    selected_ibfl: Option<LISTINGS>,
    selected_package: Option<PACKAGES>,
    selected_mgmt:bool,
    selected_ibfp:Option<IbfpType>,
    selected_fund:Option<Fund>,
    years:f64,
    mort_rate:f64
    
}
impl Sim{
    fn new(cc:&eframe::CreationContext<'_>)->Self{
        setup(&cc.egui_ctx);
        Self{
            selected_page: Some(Page::Main),
            product: None,
            selected_ibfl:None,
            selected_package:None,
            selected_mgmt:false,
            selected_ibfp:None,
            selected_fund:None,
            years:0.0,
            mort_rate:5.0,
        }
    }
}
impl eframe::App for Sim{
    fn update(&mut self, ctx:&egui::Context, _frame: &mut eframe::Frame){
        let margin = Margin::symmetric(7.0, 7.0);
        egui::TopBottomPanel::top("Header")
            .frame(egui::Frame::default()
                .inner_margin(margin)
                .fill(Color32::LIGHT_BLUE)
            ).show(ctx,|ui|{
                ui.heading("Finanz Butik Simulator");
                ui.horizontal(|ui|{
                    let main_page = self.selected_page == Some(Page::Main);
                    if ui.selectable_label(main_page, egui::RichText::new("Investment Vehicles").color(Color32::WHITE))
                        .clicked(){
                            self.selected_page = Some(Page::Main)
                        }
                    let product_page = self.selected_page == Some(Page::Products);
                    if ui.selectable_label(product_page, RichText::new("Strategies").color(Color32::WHITE))
                        .clicked(){
                            self.selected_page = Some(Page::Products)
                        }
                    let sim_page = self.selected_page == Some(Page::Simulate);
                    if ui.selectable_label(sim_page, RichText::new("Simulation").color(Color32::WHITE))
                        .clicked(){
                            self.selected_page = Some(Page::Simulate)
                        }
                });
            }
        );
        match self.selected_page{
            Some(Page::Main)=> pages::main_page::show(ctx, &mut self.product),
            Some(Page::Products)=> match self.product {
                                                        Some(Sectors::InbestForLife) => pages::product_page::inbest_for_life(ctx, &mut self.selected_ibfl,&mut self.selected_package, &mut self.selected_mgmt),
                                                        Some(Sectors::InbestForProfit)=> pages::product_page::inbest_for_profit(ctx,&mut self.selected_ibfp, &mut self.selected_package,&mut self.selected_mgmt),
                                                        Some(Sectors::InbestFund) => pages::product_page::inbest_fund(ctx, &mut self.selected_fund),
                                                        _ => (),
                                                        },
            Some(Page::Simulate) => match self.product{
                                                        Some(Sectors::InbestForLife) => pages::simulation_page::ibfl_sim(ctx,&mut self.years, &mut self.selected_ibfl, &mut self.selected_package,
                                                                                                                        &mut self.mort_rate),
                                                        Some(Sectors::InbestForProfit) => match self.selected_ibfp{
                                                                                                               Some(IbfpType::Precon) => pages::simulation_page::ibfp_1_precon_sim() ,
                                                                                                               Some(IbfpType::Mkt)=> pages::simulation_page::ibfp_mkt_sim(),
                                                                                                               Some(IbfpType::Mix)=> pages::simulation_page::ibfp_precon_mkt(),
                                                                                                               _ => (),
                                                        }
                                                        Some(Sectors::InbestFund)=> match self.selected_fund{
                                                            Some(Fund::Core) => pages::simulation_page::fund_core(),
                                                            Some(Fund::Plus) => pages::simulation_page::fund_plus(),
                                                            Some(Fund::FixedIncome)=>pages::simulation_page::fund_fi(),
                                                            _=>(),
                                                        },
                                                        _ => (),
                                                        },
            None => println!("MASSIVE ERROR OCCURED"),
        }
    }
}
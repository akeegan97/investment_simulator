use eframe::egui;
use egui::{Margin,Color32, RichText};


use pages::main_page::Sectors;
mod pages{
    pub mod main_page;
    pub mod product_page;
    pub mod simulation_page;
}

//fn to set fonts for the text style
fn setup(ctx: &egui::Context){
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Times_New_Roman".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/TimesNewRoman.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("Times_New_Roman".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Times_New_Roman".to_owned());
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
    product: Option<Sectors>
}
impl Sim{
    fn new(cc:&eframe::CreationContext<'_>)->Self{
        setup(&cc.egui_ctx);
        Self{
            selected_page: Some(Page::Main),
            product: None,
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
            Some(Page::Products)=> pages::product_page::show(),
            Some(Page::Simulate) => pages::simulation_page::show(),
            None => println!("MASSIVE ERROR OCCURED"),
        }
    }
}
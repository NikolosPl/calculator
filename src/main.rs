use eframe::egui;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust GUI Calculator",
        options,
        Box::new(|_cc| {
            let mut app = MyApp::default();
            app.load_history_from_file();
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Default)]
struct MyApp {
    num1: String,
    num2: String,
    result: String,
    selected_operator: String,
    memory: Vec<f64>,
    history: Vec<String>,
    dark_mode: bool,
}

impl MyApp {
    fn calculate_and_store(&mut self) {
        let op = self.selected_operator.trim();
        let parsed1 = self.num1.trim().parse::<f64>();
        let parsed2 = self.num2.trim().parse::<f64>();

        let res = match (parsed1, parsed2) {
            (Ok(n1), Ok(n2)) => calculate(op, n1, n2),
            (Ok(n1), Err(_)) if op == "sqrt" => calculate(op, n1, 0.0),
            _ => Err("Invalid input".to_string()),
        };

        match res {
            Ok(val) => {
                self.result = val.to_string();
                let entry = if op == "sqrt" {
                    format!("{} {} = {}", self.num1, op, self.result)
                } else {
                    format!("{} {} {} = {}", self.num1, op, self.num2, self.result)
                };
                self.history.push(entry);
                self.save_history_to_file();
            }
            Err(e) => self.result = format!("Error: {}", e),
        }
    }

    fn save_history_to_file(&self) {
        if let Some(last) = self.history.last(){
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("history.txt")
            {
                let _ = writeln!(file, "{}", last);
            }
        }
    }

    fn load_history_from_file(&mut self) {
        if let Ok(file) = File::open("history.txt") {
            let reader = BufReader::new(file);
            self.history = reader.lines().filter_map(Result::ok).collect();
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            if self.dark_mode{
                ctx.set_visuals(egui::Visuals::light());
            }
            else{
                ctx.set_visuals(egui::Visuals::dark());
            }
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.checkbox(&mut self.dark_mode, "Light mode").changed(){
                if self.dark_mode{
                    ctx.set_visuals(egui::Visuals::dark());
                }
                else{
                    ctx.set_visuals(egui::Visuals::light());
                }
            }
            ui.heading("🧮 Rust GUI Calculator");

            ui.horizontal(|ui| {
                ui.label("Num 1:");
                ui.text_edit_singleline(&mut self.num1);
            });

            ui.horizontal(|ui| {
                ui.label("Num 2:");
                ui.text_edit_singleline(&mut self.num2);
            });

            ui.label("Operator:");

            ui.horizontal_wrapped(|ui| {
                for op in ["+", "-", "*", "/", "^", "√"] {
                    if ui.button(op).clicked() {
                        self.selected_operator = op.to_string();
                        self.calculate_and_store();
                    }
                }
            });

            ui.label(format!("Result: {}", self.result));

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("M+").clicked() {
                    if let Ok(r) = self.result.parse::<f64>() {
                        self.memory.push(r);
                    }
                }
                if ui.button("MR").clicked() {
                    if let Some(last) = self.memory.last() {
                        self.result = last.to_string();
                    }
                }
                if ui.button("MC").clicked() {
                    self.memory.clear();
                }
            });

            ui.separator();

            ui.collapsing("📜 History", |ui| {
                for line in &self.history {
                    ui.label(line);
                }
            });
        });
    }
}

fn calculate(op: &str, n1: f64, n2: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(n1 + n2),
        "-" => Ok(n1 - n2),
        "*" => Ok(n1 * n2),
        "/" => {
            if n2 == 0.0 {
                Err("Cannot divide by zero".into())
            } else {
                Ok(n1 / n2)
            }
        }
        "^" => Ok(n1.powf(n2)),
        "√" => {
            if n1 < 0.0 {
                Err("Cannot sqrt negative number".into())
            } else {
                Ok(n1.sqrt())
            }
        }
        _ => Err("Unknown operator".into()),
    }
}

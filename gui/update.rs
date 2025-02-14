use std::arch::aarch64::uint32x2_t;


use crate::model::application::OptiRust;

#[derive(Debug, Clone)]
pub enum Message {
    ShowImport,
    HideImport,
    ImportData,
    ImportIndexChanged(String),
    SubmitApiKey,
    HideSubmitApiKey,
    ApiKeyChanged(String),
    SChanged(i32),
}

impl OptiRust {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SChanged(value) => {},
            Message::ShowImport => {
                if self.api_key.is_empty() {
                    self.require_api_key = true;

                }
                else {
                    self.show_import = true
                }
            },
            Message::HideImport => self.show_import = false, 
            Message::ImportData => {
                // TODO: send request, update graphic, etc.
            },
            Message::ImportIndexChanged(value) => self.imported_index = value,
            Message::ApiKeyChanged(value) => self.api_key = value,
            Message::SubmitApiKey => {
                self.require_api_key = false;
                self.show_import = true;
            },
            Message::HideSubmitApiKey => {
                self.require_api_key = false;
                self.api_key = String::new();
            },
        }
        // match message {
        //     Message::SChanged(value) => self.s = value,
        //     Message::KChanged(value) => self.k = value,
        //     Message::RChanged(value) => self.r = value,
        //     Message::SigmaChanged(value) => self.sigma = value,
        //     Message::TChanged(value) => self.t = value,
        //     Message::NumSimulationsChanged(value) => self.num_simulations = value,
        //     Message::NumStepsChanged(value) => self.num_steps = value,
        //     Message::ToggleOptionType => self.is_call = !self.is_call,
        //     Message::RunSimulation => {
        //         // Parse input values
        //         let s = self.s.parse().unwrap_or(0.0);
        //         let k = self.k.parse().unwrap_or(0.0);
        //         let r = self.r.parse().unwrap_or(0.0);
        //         let sigma = self.sigma.parse().unwrap_or(0.0);
        //         let t = self.t.parse().unwrap_or(0.0);
        //         let num_simulations = self.num_simulations.parse().unwrap_or(10000);
        //         let num_steps = self.num_steps.parse().unwrap_or(252);
    
        //         // Run Monte Carlo simulation
        //         self.result = Some(monte_carlo_american_option(
        //             s, k, r, sigma, t, self.is_call, num_simulations, num_steps,
        //         ));
    
        //         // Invalidate the chart to redraw it with new data
        //         self.chart.clear();
        //     }
        // }
        // Command::none()
    }
}
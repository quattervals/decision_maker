mod decision_model;

use decision_model::model as DM;
use slint::{SharedString, VecModel, Weak};
use std::{cell::RefCell, rc::Rc};
slint::include_modules!();

type ModelRef = Rc<RefCell<DM::DecisionModel>>;

struct AppState {
    model: ModelRef,
    window: Weak<MainWindow>,
}

impl AppState {
    fn new(window: &MainWindow) -> Self {
        Self {
            model: Rc::new(RefCell::new(DM::DecisionModel::new())),
            window: window.as_weak(),
        }
    }

    fn window(&self) -> MainWindow {
        self.window.unwrap()
    }

    fn handle_dialog_play(&self) {
        self.model.borrow_mut().reset_score_and_indices();

        if let Some(current_pair) = self.model.borrow_mut().get_current_pair() {
            self.window()
                .set_lhs_param_name(current_pair.0.get_name_and_score().0.into());
            self.window()
                .set_rhs_param_name(current_pair.1.get_name_and_score().0.into());
            View::Compete.set_visible(&self.window());
        }
    }

    fn handle_dialog_return_edit(&self) {
        self.model.borrow_mut().reset_score_and_indices();
        View::Edit.set_visible(&self.window());
    }

    fn handle_dialog_results(&self) {
        let params = self.model.borrow_mut().sorted_by_score();
        let vm = Rc::new(VecModel::<Parameter>::default());

        vm.extend(params.into_iter().map(|i| Parameter {
            name: i.get_name_and_score().0.into(),
            score: i.get_name_and_score().1 as i32,
        }));

        self.window().set_results(vm.into());
        View::Result.set_visible(&self.window());
    }

    fn handle_next_pair(&self, winner: Winner) {
        let winner_side = match winner {
            Winner::Lhs => DM::Side::Lhs,
            Winner::Rhs => DM::Side::Rhs,
            _ => DM::Side::Other,
        };

        self.model
            .borrow_mut()
            .record_score_of_current_pair(winner_side, 1);
        self.model.borrow_mut().move_to_next_pair();

        match self.model.borrow_mut().get_current_pair() {
            Some(p) => {
                self.window()
                    .set_lhs_param_name(p.0.get_name_and_score().0.into());
                self.window()
                    .set_rhs_param_name(p.1.get_name_and_score().0.into());
            }
            None => {
                self.window().set_lhs_param_name("--".into());
                self.window().set_rhs_param_name("--".into());
                self.window().set_results_enabled(true);
            }
        }
    }

    fn handle_show(&self) {
        self.window()
            .set_parameters(self.model.borrow_mut().get_parameters().into());
    }

    fn handle_discard(&self) {
        self.window().set_parameters(SharedString::new());
        self.window().set_play_enabled(false);
        self.model.borrow_mut().reset();
    }

    fn handle_append(&self) {
        let parameters_ui = self.window().get_parameters();
        println!("add params clicked:\n{}", parameters_ui);

        let parameter_list = DM::clean_input(&parameters_ui);
        self.model.borrow_mut().prepare_model(&parameter_list, true);

        if self.model.borrow_mut().is_model_ready_to_play() {
            self.window().set_play_enabled(true);
        }
    }

    fn handle_parameters_edited(&self, new_text: SharedString) {
        self.window().set_parameters(new_text);
    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let app_state = Rc::new(AppState::new(&main_window));

    {
        let app_state = app_state.clone();
        main_window.on_dialog_play(move || app_state.handle_dialog_play());
    }

    {
        let app_state = app_state.clone();
        main_window.on_dialog_return_edit(move || app_state.handle_dialog_return_edit());
    }

    {
        let app_state = app_state.clone();
        main_window.on_dialog_results(move || app_state.handle_dialog_results());
    }

    {
        let app_state = app_state.clone();
        main_window.on_next_pair(move |winner| app_state.handle_next_pair(winner));
    }

    {
        let app_state = app_state.clone();
        main_window.on_show(move || app_state.handle_show());
    }

    {
        let app_state = app_state.clone();
        main_window.on_discard(move || app_state.handle_discard());
    }

    {
        let app_state = app_state.clone();
        main_window.on_append(move || app_state.handle_append());
    }

    {
        let app_state = app_state.clone();
        main_window
            .on_parameters_edited(move |new_text| app_state.handle_parameters_edited(new_text));
    }

    main_window.run().unwrap();
}

enum View {
    Edit,
    Compete,
    Result,
}

impl View {
    fn set_visible(&self, main_window: &MainWindow) {
        main_window.set_edit_visible(matches!(self, View::Edit));
        main_window.set_compete_visible(matches!(self, View::Compete));
        main_window.set_result_visible(matches!(self, View::Result));
    }
}

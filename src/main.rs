mod decision_model;

use decision_model::model as DM;

use std::rc::Rc;

use slint::{SharedString, VecModel};
slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();
    let model = Rc::new(DM::DecisionModel::new());

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_dialog_play(move || {
        mdl.reset_score_and_indices();

        let current_pair = mdl.get_current_pair().unwrap();

        mw.unwrap()
            .set_lhs_param_name(current_pair.0.get_name_and_score().0.into());
        mw.unwrap()
            .set_rhs_param_name(current_pair.1.get_name_and_score().0.into());

        View::Compete.set_visible(&mw.unwrap());
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_dialog_return_edit(move || {
        mdl.reset_score_and_indices();
        View::Edit.set_visible(&mw.unwrap())
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_dialog_results(move || {
        let params = mdl.sorted_by_score();
        let vm = Rc::new(VecModel::<Parameter>::default());

        vm.extend(params.into_iter().map(|i| Parameter {
            name: i.get_name_and_score().0.into(),
            score: i.get_name_and_score().1 as i32,
        }));

        mw.unwrap().set_results(vm.into());

        View::Result.set_visible(&mw.unwrap());
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_next_pair(move |winner| {
        let winner_side: DM::Side = match winner {
            Winner::Lhs => DM::Side::Lhs,
            Winner::Rhs => DM::Side::Rhs,
            _ => DM::Side::Other,
        };

        mdl.record_score_of_current_pair(winner_side, 1);

        mdl.move_to_next_pair();

        match mdl.get_current_pair() {
            Some(p) => {
                mw.unwrap()
                    .set_lhs_param_name(p.0.get_name_and_score().0.into());
                mw.unwrap()
                    .set_rhs_param_name(p.1.get_name_and_score().0.into());
            }
            None => {
                mw.unwrap().set_lhs_param_name("--".into());
                mw.unwrap().set_rhs_param_name("--".into());
                mw.unwrap().set_results_enabled(true);
            }
        }
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_show(move || {
        mw.unwrap().set_parameters(mdl.get_parameters().into());
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_discard(move || {
        mw.unwrap().set_parameters(SharedString::new());
        mw.unwrap().set_play_enabled(false);

        mdl.reset();
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_append(move || {
        let parameters_ui = mw.unwrap().get_parameters();
        println!("add params clicked:\n{}", parameters_ui);

        let parameter_list = DM::clean_input(&parameters_ui);
        mdl.prepare_model(&parameter_list, true);

        if mdl.is_model_ready_to_play() {
            mw.unwrap().set_play_enabled(true);
        }
    });

    let mw = main_window.as_weak();
    main_window.on_parameters_edited(move |new_text| mw.unwrap().set_parameters(new_text));

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

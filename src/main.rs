use std::{cell::RefCell, rc::Rc};

use slint::{SharedString, VecModel};

slint::include_modules!();

enum Side {
    Lhs,
    Rhs,
    Other,
}

#[derive(Default, PartialEq, Debug, Clone)]
struct Parameters {
    name: String,
    score: u32,
}

struct DecisionModel {
    parameters: RefCell<Vec<Parameters>>,
    indices: RefCell<Vec<(usize, usize)>>,
    current_index: RefCell<Option<usize>>,
}
impl DecisionModel {
    fn new() -> Self {
        Self {
            parameters: RefCell::new(Vec::new()),
            indices: RefCell::new(Vec::new()),
            current_index: RefCell::new(None),
        }
    }

    fn reset(&self) {
        let mut params = self.parameters.borrow_mut();
        let mut idx = self.indices.borrow_mut();
        let mut ci = self.current_index.borrow_mut();

        *params = Vec::new();
        *idx = Vec::new();
        *ci = None;
    }

    fn move_to_next_pair(&self) {
        let mut ci = self.current_index.borrow_mut();
        let idx = self.indices.borrow();

        if ci.is_some() {
            match idx.get(ci.unwrap() + 1) {
                Some(_) => *ci = Some(ci.unwrap() + 1),
                None => *ci = None,
            };
        }
    }

    fn get_current_pair(&self) -> Option<(Parameters, Parameters)> {
        let ci = self.current_index.borrow();
        let params = self.parameters.borrow();
        let idx = self.indices.borrow();
        match *ci {
            Some(p) => Some((params[idx[p].0].clone(), params[idx[p].1].clone())),
            None => None,
        }
    }

    fn record_score_of_current_pair(&self, winner: Side, points_to_add: u32) {
        let ci = self.current_index.borrow();
        let mut params = self.parameters.borrow_mut();
        let idx = self.indices.borrow();

        if ci.is_some() {
            match winner {
                Side::Lhs => {
                    params[idx[ci.unwrap()].0].score += points_to_add;
                }
                Side::Rhs => {
                    params[idx[ci.unwrap()].1].score += points_to_add;
                }
                _ => {}
            };
        }
    }

    fn prepare_model(&self, param_list: &Vec<String>, randomize: bool) {
        let mut parameters = self.parameters.borrow_mut();
        for param in param_list {
            parameters.push(Parameters {
                name: param.to_string(),
                score: 0,
            });
        }

        parameters.sort_by(|a, b| a.name.cmp(&b.name));
        parameters.retain(|i| i.name.ne(""));
        parameters.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));

        let mut indices = self.indices.borrow_mut();
        *indices = index_pairs(parameters.len());
        if randomize {
            let mut rng = rand::thread_rng();
            rand::seq::SliceRandom::shuffle(&mut indices[..], &mut rng);
        }

        match parameters.len() > 1 {
            true => {
                let mut ci = self.current_index.borrow_mut();
                *ci = Some(0);
            }
            false => {
                let mut ci = self.current_index.borrow_mut();
                *ci = None;
            }
        }
    }

    fn reset_score_and_indices(&self) {
        let mut parameters: std::cell::RefMut<Vec<Parameters>> = self.parameters.borrow_mut();
        parameters.iter_mut().for_each(|p| p.score = 0);

        if parameters.len() > 1 {
            let mut ci = self.current_index.borrow_mut();
            *ci = Some(0);
        }
    }

    fn is_model_ready_to_play(&self) -> bool {
        (self.parameters.borrow().len() >= 2)
            && (self.parameters.borrow().iter().all(|x| x.score == 0))
    }

    fn get_parameters(&self) -> String {
        self.parameters
            .borrow()
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let model = Rc::new(DecisionModel::new());

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_dialog_play(move || {
        mdl.reset_score_and_indices();

        let current_pair = mdl.get_current_pair().unwrap();

        mw.unwrap().set_lhs_param_name(current_pair.0.name.into());
        mw.unwrap().set_rhs_param_name(current_pair.1.name.into());

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
        let mut params: Vec<Parameters> = mdl.parameters.borrow().clone();
        params.sort_by(|a, b| b.score.cmp(&a.score));

        let vm = Rc::new(VecModel::<Parameter>::default());

        vm.extend(params.into_iter().map(|i| Parameter {
            name: i.name.into(),
            score: i.score as i32,
        }));

        mw.unwrap().set_results(vm.into());

        View::Result.set_visible(&mw.unwrap());
    });

    let mw = main_window.as_weak();
    let mdl = model.clone();
    main_window.on_next_pair(move |winner| {
        let winner_side: Side = match winner {
            Winner::Lhs => Side::Lhs,
            Winner::Rhs => Side::Rhs,
            _ => Side::Other,
        };

        mdl.record_score_of_current_pair(winner_side, 1);

        mdl.move_to_next_pair();

        match mdl.get_current_pair() {
            Some(p) => {
                mw.unwrap().set_lhs_param_name(p.0.name.into());
                mw.unwrap().set_rhs_param_name(p.1.name.into());
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

        let parameter_list = clean_input(&parameters_ui);
        mdl.prepare_model(&parameter_list, true);

        if mdl.is_model_ready_to_play() {
            mw.unwrap().set_play_enabled(true);
        }
    });

    let mw = main_window.as_weak();
    main_window.on_parameters_edited(move |new_text| mw.unwrap().set_parameters(new_text));

    main_window.run().unwrap();
}

fn index_pairs(matrix_size: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::<(usize, usize)>::with_capacity(matrix_size);

    for (i, j) in (0..matrix_size).flat_map(|j| (0..j).map(move |i| (i, j))) {
        indices.push((i, j));
    }

    indices
}

fn clean_input(input: &str) -> Vec<String> {
    let mut v: Vec<String> = input.split('\n').map(str::to_string).collect();
    v = v.into_iter().map(|s| s.trim().to_string()).collect();
    v.retain(|i| i.ne(""));
    v
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

#[cfg(test)]
mod tests {
    use super::*; // import functions from outer scope

    #[test]
    fn model_go_to_next_pair() {
        let input: Vec<String> = vec!["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
        let model = DecisionModel::new();
        model.prepare_model(&input, false);
        model.move_to_next_pair();

        let current_model = model.get_current_pair().unwrap();
        assert_eq!("c", current_model.1.name);
    }

    #[test]
    fn model_increment_score() {
        let input: Vec<String> = vec!["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
        let model = DecisionModel::new();
        model.prepare_model(&input, false);

        model.record_score_of_current_pair(Side::Rhs, 15);

        let current_model = model.get_current_pair().unwrap();
        assert_eq!(15, current_model.1.score);
    }

    #[test]
    fn model_current_pair_good_model() {
        let input: Vec<String> = vec!["a", "b", "c"].iter().map(|&s| s.to_string()).collect();

        let model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!(
            (
                Parameters {
                    name: "a".to_string(),
                    score: 0
                },
                Parameters {
                    name: "b".to_string(),
                    score: 0
                }
            ),
            model.get_current_pair().unwrap()
        );
    }

    #[test]
    fn model_current_pair_bad_model() {
        let input: Vec<String> = vec!["a"].iter().map(|&s| s.to_string()).collect();

        let model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!(None, model.get_current_pair());
    }

    #[test]
    fn model_parameter_list() {
        let input: Vec<String> = vec!["a", "miao", "x"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!("a\nmiao\nx", model.get_parameters());
    }
    #[test]
    fn model_prepare() {
        let input: Vec<String> = vec!["a", "miao", "x"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!(true, model.is_model_ready_to_play());
    }

    #[test]
    fn model_dedupe() {
        let expected: Vec<String> = vec!["a", "miao", "x"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let input = String::from("a \nmiao\n\nx\n");
        let result = clean_input(&input);
        assert_eq!(expected, result);
    }
}

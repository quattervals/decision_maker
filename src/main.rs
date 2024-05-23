use std::{cell::RefCell, rc::Rc};

use rand::{distributions::Standard, seq::index};
use slint::{Model, SharedString, VecModel};

slint::include_modules!();

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

type DecisionModel = Vec<Parameter>;

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();
    let input_model = Rc::new(VecModel::<String>::default());

    let results = Rc::new(VecModel::<Parameter>::default());
    results.as_ref().push(Parameter {
        name: "karl".into(),
        score: 99,
    });
    results.as_ref().push(Parameter {
        name: "susi".into(),
        score: 100,
    });
    main_window.set_results(results.into());

    let parameter = Rc::new(RefCell::new(DecisionModel::new()));
    let combination_indices: Rc<RefCell<Vec<(usize, usize)>>> =
        Rc::new(RefCell::new(Vec::<(usize, usize)>::new()));
    let index_iter = Rc::new(RefCell::new(Vec::<(usize, usize)>::new().into_iter()));
    let current_pair: Rc<RefCell<(usize, usize)>> = Rc::new(RefCell::new((0, 0)));

    let pm = parameter.clone();
    let ii: Rc<RefCell<std::vec::IntoIter<(usize, usize)>>> = index_iter.clone();
    let cp = current_pair.clone();
    let mmw = main_window.as_weak();
    main_window.on_next_pair(move |winner| {
        let current_indices = *cp.borrow_mut();

        match winner {
            Winner::Lhs => {
                pm.borrow_mut()[current_indices.0].score += 1;
                println!("lhs wins");
            }
            Winner::Rhs => {
                pm.borrow_mut()[current_indices.1].score += 1;
                println!("rhs wins");
            }
            _ => {}
        };

        let next_indices = ii.borrow_mut().next();

        let next_indices = match next_indices {
            Some(ix) => ix,
            None => (0, 0),
        };

        if next_indices == (0, 0) {
            pm.borrow_mut().sort_by(|a, b| b.score.cmp(&a.score));

            mmw.unwrap().set_ranking_visible(true);
            mmw.unwrap().set_compete_visible(false);

            let vm = Rc::new(VecModel::<Parameter>::default());
            vm.set_vec(pm.borrow().clone());

            mmw.unwrap().set_results(vm.into());
        }

        println!("indices ({}, {})", next_indices.0, next_indices.1);

        let lhs = pm.borrow()[next_indices.0].clone();
        let rhs = pm.borrow()[next_indices.1].clone();

        mmw.unwrap().set_lhs_param(lhs);
        mmw.unwrap().set_rhs_param(rhs);

        *cp.borrow_mut() = next_indices;
    });

    let mww: slint::Weak<MainWindow> = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_show(move || {
        sort_dedupe_clean_input(im.clone());

        let vec_as_string = im.as_ref().iter().collect::<Vec<String>>().join("\n");
        let mut s = SharedString::new();
        s.push_str(&vec_as_string);
        mww.unwrap().set_parameters(s);
    });

    let mww: slint::Weak<MainWindow> = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_discard(move || {
        im.set_vec(Vec::<String>::new());
        mww.unwrap().set_parameters(SharedString::new());
    });

    let mww = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_append(move || {
        let parameters = mww.unwrap().get_parameters();
        println!("add params clicked:\n{}", parameters);

        im.extend(parameters.as_str().split('\n').map(str::to_string));
    });

    let im = input_model.clone();
    let pm = parameter.clone();
    let ci = combination_indices.clone();
    let ii = index_iter.clone();
    let cp = current_pair.clone();
    let mmw = main_window.as_weak();
    main_window.on_play(move || {
        sort_dedupe_clean_input(im.clone());

        pm.borrow_mut().clear();
        pm.borrow_mut().extend(prepare_model(im.clone()));

        ci.borrow_mut().clear();
        ci.borrow_mut().extend(index_pairs(pm.borrow().len()));

        let model_ok = model_ok(&pm.borrow());

        //update iterator
        *ii.borrow_mut() = ci.borrow().clone().into_iter();

        let current_pair = ii.borrow_mut().next().unwrap();
        *cp.borrow_mut() = current_pair;

        let lhs = pm.borrow()[current_pair.0].clone();
        let rhs = pm.borrow()[current_pair.1].clone();

        mmw.unwrap().set_lhs_param(lhs);
        mmw.unwrap().set_rhs_param(rhs);

        println!("indices ({},{})", current_pair.0, current_pair.1);

        return model_ok;
    });

    let mmw = main_window.as_weak();
    main_window.on_parameters_edited(move |new_text| mmw.unwrap().set_parameters(new_text));

    main_window.run().unwrap();

    let check_output: Vec<String> = input_model.as_ref().iter().collect();
    let mdl = parameter
        .borrow()
        .iter()
        .map(|n: &Parameter| n.name.to_string())
        .collect::<Vec<String>>();
    let scrs = parameter
        .borrow()
        .iter()
        .map(|s: &Parameter| s.score)
        .collect::<Vec<i32>>();

    println!("at the end of the program:\n{:?}", check_output);
    println!("model and rating\n{:?} - {:?}", mdl, scrs);
}

fn index_pairs(matrix_size: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::<(usize, usize)>::with_capacity(matrix_size);

    for (i, j) in (0..matrix_size).flat_map(|j| (0..j).map(move |i| (i, j))) {
        indices.push((i, j));
    }
    indices
    //todo: shuffle indices
}

fn sort_dedupe_clean_input(input_model: Rc<VecModel<String>>) {
    let mut s: Vec<String> = input_model.as_ref().iter().collect();
    s.iter_mut().for_each(|s| *s = s.trim().to_string());
    s.sort();
    s.retain(|i| i.ne(""));
    s.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    input_model.set_vec(s);
}

fn prepare_model(input_model: Rc<VecModel<String>>) -> DecisionModel {
    input_model
        .as_ref()
        .iter()
        .map(|s| Parameter {
            name: s.into(),
            score: 0,
        })
        .collect()
}

fn model_ok(model: &DecisionModel) -> bool {
    model.len() >= 2 && model.iter().all(|x| x.score == 0)
}

#[cfg(test)]
mod tests {
    use super::*; // import functions from outer scope

    #[test]
    fn index_update() {
        let combo_indices_basis: Vec<(usize, usize)> = Vec::new();
        let combination_indices: Rc<RefCell<Vec<(usize, usize)>>> =
            Rc::new(RefCell::new(combo_indices_basis));
        let index_iter: Rc<RefCell<std::vec::IntoIter<(usize, usize)>>> = Rc::new(RefCell::new(
            combination_indices.borrow().clone().into_iter(),
        ));
        let current_pair: Rc<RefCell<(usize, usize)>> = Rc::new(RefCell::new((0, 0)));

        let ci = combination_indices.clone();
        let ii = index_iter.clone();
        let cp = current_pair.clone();

        let play = move || {
            ci.borrow_mut().clear();
            ci.borrow_mut().extend(index_pairs(3));

            //update iterator
            *ii.borrow_mut() = ci.borrow().clone().into_iter();
            let current_pair = ii.borrow_mut().next();
            *cp.borrow_mut() = current_pair.unwrap();
        };

        let ii = index_iter.clone();
        let cp = current_pair.clone();
        let yielding = move || {
            let next_indices = ii.borrow_mut().next();
            let next_indices = match next_indices {
                Some(ix) => ix,
                None => (0, 0),
            };
            *cp.borrow_mut() = next_indices;
        };

        play();
        yielding();
        yielding();
    }

    #[test]
    fn check_model_ok() {
        let bad_model = vec![
            Parameter {
                name: "a".into(),
                score: 0,
            },
            Parameter {
                name: "b".into(),
                score: 2,
            },
        ];

        assert_eq!(false, model_ok(&bad_model));

        let ok_model = vec![
            Parameter {
                name: "a".into(),
                score: 0,
            },
            Parameter {
                name: "b".into(),
                score: 0,
            },
        ];

        assert_eq!(true, model_ok(&ok_model));
    }

    #[test]
    fn fill_input_model() {
        let inner_input_model = vec_of_strings!["b", "a", "x"];
        let input_model: Rc<VecModel<String>> = Rc::new(VecModel::<String>::default());
        input_model.set_vec(inner_input_model);

        let model = prepare_model(input_model.clone());
        assert_eq!(3, model.len());
        assert_eq!(0, model[2].score);
        assert_eq!("x", model[2].name.as_str());
    }

    #[test]
    fn clean_input_model() {
        let inner_input_model = vec_of_strings!["b", "a", "", "a", "  ix"];
        let input_model: Rc<VecModel<String>> = Rc::new(VecModel::<String>::default());
        input_model.set_vec(inner_input_model);

        sort_dedupe_clean_input(input_model.clone());

        let inner_test_model: Vec<String> = input_model.as_ref().iter().collect();
        assert_eq!("a", inner_test_model[0]);
        assert_eq!("ix", inner_test_model[2]);
        assert_eq!(3, inner_test_model.len());
    }
    #[test]
    fn index_generation() {
        assert_eq!(0, index_pairs(0).len());
        assert_eq!(0, index_pairs(1).len());
        assert_eq!(1, index_pairs(2).len());
        assert_eq!((0, 1), index_pairs(2)[0]);

        assert_eq!(3, index_pairs(3).len());
        assert_eq!((0, 1), index_pairs(3)[0]);
        assert_eq!((0, 2), index_pairs(3)[1]);
        assert_eq!((1, 2), index_pairs(3)[2]);

        const HI_INDEX: usize = 8;
        assert_eq!(
            (HI_INDEX - 2, HI_INDEX - 1),
            index_pairs(HI_INDEX)[((HI_INDEX * (HI_INDEX - 1)) / 2) - 1]
        );
    }
}

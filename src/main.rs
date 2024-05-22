use std::rc::Rc;

use rand::distributions::Standard;
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

    let mut parameter = DecisionModel::new();

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
    main_window.on_play(move || {
        sort_dedupe_clean_input(im.clone());

        parameter.clear();
        parameter.extend(prepare_model(im.clone()));

        model_ok(&parameter)
    });

    let mmw = main_window.as_weak();
    main_window.on_parameters_edited(move |new_text| mmw.unwrap().set_parameters(new_text));

    main_window.run().unwrap();

    let check_output: Vec<String> = input_model.as_ref().iter().collect();

    println!("at the end of the program:\n{:?}", check_output);
}

fn index_pairs(matrix_size: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::<(usize, usize)>::with_capacity(matrix_size);

    for (i, j) in (0..matrix_size).flat_map(|j| (0..j).map(move |i| (i, j))) {
        indices.push((i, j));
    }
    indices
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

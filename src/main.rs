use std::rc::Rc;

use rand::distributions::Standard;
use slint::{SortModel, StandardListViewItem, VecModel};

slint::include_modules!();

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

    let mww = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_append(move || {
        let parameters = mww.unwrap().get_parameters();
        println!("add params clicked:\n{}", parameters);

        im.extend(parameters.as_str().split('\n').map(str::to_string));
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

#[cfg(test)]
mod tests {
    use super::*; // import functions from outer scope

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

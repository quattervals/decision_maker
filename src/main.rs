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

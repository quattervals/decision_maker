use std::rc::Rc;

use slint::VecModel;

slint::include_modules!();

fn input_to_words(line: &str) -> Vec<String> {
    line.split('\n').map(str::to_string).collect()
}

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();
    let input_model = Rc::new(VecModel::<String>::default());

    let mww = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_add_parameters(move || {
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

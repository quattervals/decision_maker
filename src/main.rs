use std::{borrow::BorrowMut, rc::Rc, string};

use slint::VecModel;

slint::include_modules!();

fn input_to_words(line: &str) -> Vec<String> {
    line.split('\n').map(str::to_string).collect()
  }

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    let input_model = Rc::new(VecModel::<String>::default());

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });



    let mww = main_window.as_weak();
    let im = input_model.clone();
    main_window.on_friss(move || {


        let thatext = mww.unwrap();
        println!("friss clicked:\n{}", thatext.get_params());

        // let im = input_model.borrow_mut();

        let intermediate = input_to_words(thatext.get_params().as_str());

        im.extend(intermediate);


    });

    let ui_handle = main_window.as_weak();
    main_window.on_text_edited(move |new_text| {
        let ui = ui_handle.unwrap();
        ui.set_params(new_text);

        println!("current text {}", ui.get_params());
    });

    main_window.run().unwrap();

    let bla: Vec<String> = input_model.as_ref().iter().collect();

    println!("at the end of the world:\n{:?}", bla);
}

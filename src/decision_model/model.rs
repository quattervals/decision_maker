pub enum Side {
    Lhs,
    Rhs,
    Other,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Parameters {
    name: String,
    score: u32,
}

impl Parameters {
    pub fn get_name_and_score(&self) -> (&str, u32) {
        (&self.name, self.score)
    }
}

pub struct DecisionModel {
    parameters: Vec<Parameters>,
    indices: Vec<(usize, usize)>,
    current_index: Option<usize>,
}
impl DecisionModel {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            indices: Vec::new(),
            current_index: None,
        }
    }

    pub fn reset(&mut self) {
        // let mut params = self.parameters.borrow_mut();
        // let mut idx = self.indices.borrow_mut();
        // let mut ci = self.current_index.borrow_mut();

        self.parameters = Vec::new();
        self.indices = Vec::new();
        self.current_index = None;
    }

    pub fn move_to_next_pair(&mut self) {
        let ci = self.current_index;

        let idx = &self.indices;

        if ci.is_some() {
            match idx.get(ci.unwrap() + 1) {
                Some(_) => self.current_index = Some(ci.unwrap() + 1),
                None => self.current_index = None,
            };
        }
    }

    pub fn get_current_pair(&mut self) -> Option<(Parameters, Parameters)> {
        self.current_index.map(|p| {
            (
                self.parameters[self.indices[p].0].clone(),
                self.parameters[self.indices[p].1].clone(),
            )
        })
    }

    pub fn record_score_of_current_pair(&mut self, winner: Side, points_to_add: u32) {
        let ci = self.current_index;

        if ci.is_some() {
            match winner {
                Side::Lhs => {
                    self.parameters[self.indices[ci.unwrap()].0].score += points_to_add;
                }
                Side::Rhs => {
                    self.parameters[self.indices[ci.unwrap()].1].score += points_to_add;
                }
                _ => {}
            };
        }
    }

    pub fn prepare_model(&mut self, param_list: &Vec<String>, randomize: bool) {
        for param in param_list {
            self.parameters.push(Parameters {
                name: param.to_string(),
                score: 0,
            });
        }

        self.parameters.sort_by(|a, b| a.name.cmp(&b.name));
        self.parameters.retain(|i| !i.name.is_empty());
        self.parameters
            .dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));

        self.indices = index_pairs(self.parameters.len());
        if randomize {
            let mut rng = rand::thread_rng();
            rand::seq::SliceRandom::shuffle(&mut self.indices[..], &mut rng);
        }

        match self.parameters.len() > 1 {
            true => {
                self.current_index = Some(0);
            }
            false => {
                self.current_index = None;
            }
        }
    }

    pub fn reset_score_and_indices(&mut self) {
        // let mut parameters: std::cell::RefMut<Vec<Parameters>> = self.parameters.borrow_mut();
        self.parameters.iter_mut().for_each(|p| p.score = 0);

        if self.parameters.len() > 1 {
            self.current_index = Some(0);
        }
    }

    pub fn is_model_ready_to_play(&mut self) -> bool {
        (self.parameters.len() >= 2) && (self.parameters.iter().all(|x| x.score == 0))
    }

    pub fn get_parameters(&mut self) -> String {
        self.parameters
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn sorted_by_score(&self) -> Vec<Parameters> {
        let mut params = self.parameters.clone();
        params.sort_by(|a, b| b.score.cmp(&a.score));
        params
    }
}

fn index_pairs(matrix_size: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::<(usize, usize)>::with_capacity(matrix_size);

    for (i, j) in (0..matrix_size).flat_map(|j| (0..j).map(move |i| (i, j))) {
        indices.push((i, j));
    }

    indices
}

pub fn clean_input(input: &str) -> Vec<String> {
    let mut v: Vec<String> = input.split('\n').map(str::to_string).collect();
    v = v.into_iter().map(|s| s.trim().to_string()).collect();
    v.retain(|i| !i.is_empty());
    v
}

#[cfg(test)]
mod tests {
    use super::*; // import functions from outer scope

    #[test]
    fn return_sorted() {
        let input = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);
        model.parameters[0].score = 1;
        model.parameters[1].score = 2;
        model.parameters[2].score = 3;

        let result = model.sorted_by_score();
        assert_eq!(3, result[0].score);
        assert_eq!("c", result[0].name);
    }
    #[test]
    fn model_go_to_next_pair() {
        let input: Vec<String> = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);
        model.move_to_next_pair();

        let current_model = model.get_current_pair().unwrap();
        assert_eq!("c", current_model.1.name);
    }

    #[test]
    fn model_increment_score() {
        let input: Vec<String> = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);

        model.record_score_of_current_pair(Side::Rhs, 15);

        let current_model = model.get_current_pair().unwrap();
        assert_eq!(15, current_model.1.score);
    }

    #[test]
    fn model_current_pair_good_model() {
        let input: Vec<String> = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();

        let mut model = DecisionModel::new();
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
        let input: Vec<String> = ["a"].iter().map(|&s| s.to_string()).collect();

        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!(None, model.get_current_pair());
    }

    #[test]
    fn model_parameter_list() {
        let input: Vec<String> = ["a", "miao", "x"].iter().map(|&s| s.to_string()).collect();

        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert_eq!("a\nmiao\nx", model.get_parameters());
    }
    #[test]
    fn model_prepare() {
        let input: Vec<String> = ["a", "miao", "x"].iter().map(|&s| s.to_string()).collect();

        let mut model = DecisionModel::new();
        model.prepare_model(&input, false);

        assert!(model.is_model_ready_to_play());
    }

    #[test]
    fn model_dedupe() {
        let expected: Vec<String> = ["a", "miao", "x"].iter().map(|&s| s.to_string()).collect();

        let input = String::from("a \nmiao\n\nx\n");
        let result = clean_input(&input);
        assert_eq!(expected, result);
    }
}

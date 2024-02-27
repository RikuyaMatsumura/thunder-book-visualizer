use dioxus::prelude::*;
use std::mem::swap;

const MOVE_YX: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

const HEIGHT: usize = 3;
const WIDTH: usize = 3;
const END_TURN: usize = 4;

pub(crate) fn alternate_maze_state(cx: Scope) -> Element {
    let input_data = use_state(cx, || "554\nA7B\n961".to_string());
    let output_data = use_state(cx, || "RUUD".to_string());
    let mut turn = use_state(cx, || 0);
    let states = use_state(cx, || {
        CalcedState::new("554\nA7B\n961".to_string(), "RUUD".to_string())
    });
    log::info!("input_data: {}", input_data);
    log::info!("output_data: {}", output_data);
    log::info!("turn: {}", turn);
    cx.render(rsx! {
        ul {
            li {
                p {
                    "HEIGHT: {HEIGHT}"
                }
            }
            li {
                p {
                    "WIDTH: {WIDTH}"
                }
            }
            li {
                p {
                    "END_TURN: {END_TURN}"
                }
            }
            li {
                p {
                    class: "block m-2 text-sm font-medium text-gray-900 dark:text-white",
                    "input"
                }
                textarea {
                    rows: 4,
                    class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    value: "{input_data}",
                    oninput: move |evt| {
                        input_data.set(evt.value.clone());
                    }
                }
            }
            li {
                p {
                    class: "block m-2 text-sm font-medium text-gray-900 dark:text-white",
                    "output"
                }
                input {
                    class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    value: "{output_data}",
                    oninput: move |evt| {
                        output_data.set(evt.value.clone());
                    }
                }
            }
            li {
                button {
                    class: "m-2 text-white bg-blue-500 border-0 rounded py-1 px-4 focus:outline-none hover:bg-gray-300",
                    onclick: move |_| {
                        turn.set(0);
                        states.set(CalcedState::new(input_data.to_string(), output_data.to_string()));
                    },
                    "init"
                }
            }
            li {
                class: "max-w-xs mx-auto",
                label {
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "turn: 0 ~ {END_TURN} で指定してください。"
                }
                div {
                    class: "inline-flex",
                    button {
                        class: "bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded-l",
                        onclick: move |_| {
                            if turn > &0 {
                                turn -= 1
                            }
                        },
                        "Prev"
                    }
                    p {
                        class: "bg-gray-50 border-x-0 border-gray-300 px-8 flex items-center justify-center text-gray-900 dark:text-white",
                        "{turn}"
                    }
                    button {
                        class: "bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded-r",
                        onclick: move |_| {
                            if turn < &END_TURN {
                                turn += 1
                            }
                        },
                        "Next"
                    }
                }
            }
            li {
                p {
                    class: "block m-2 font-medium text-gray-900 dark:text-white",
                    "ゲームの状況: {states.get_winning_status(turn.to_string())}"
                }
            }
            li {
                p {
                    class: "block m-2 font-medium text-gray-900 dark:text-white",
                    "A score: {states.get_score(turn.to_string(), 0)}"
                }
            }
            li {
                p {
                    class: "block m-2 font-medium text-gray-900 dark:text-white",
                    "B score: {states.get_score(turn.to_string(), 1)}"
                }
            }
            li {
                class: "grid p-5 grid-cols-3 gap-1",
                for h in 0..HEIGHT {
                    for w in 0..WIDTH {
                        p {
                            class: "size-32 bg-green-100 text-5xl flex items-center justify-center",
                            "{states.get_grid_hw(turn.to_string(), h, w)}"
                        }
                    }
                }
            }
        }
    })
}

enum WinningStatue {
    Win,
    Lose,
    Draw,
    None,
}

#[derive(Clone)]
struct Character {
    y: usize,
    x: usize,
    game_score: u32,
}

impl Character {
    fn new(y: usize, x: usize) -> Self {
        Character {
            y,
            x,
            game_score: 0,
        }
    }
}

#[derive(Clone)]
struct AlternateMazeState {
    points: [[u32; WIDTH]; HEIGHT],
    characters: [Character; 2],
    turn: usize,
}

impl AlternateMazeState {
    fn new(points: [[u32; WIDTH]; HEIGHT], characters: [Character; 2]) -> Self {
        AlternateMazeState {
            points,
            characters,
            turn: 0,
        }
    }

    fn init(mut init_data: String) -> Self {
        let mut points = [[0; WIDTH]; HEIGHT];
        let mut characters = [
            Character::new(HEIGHT / 2, WIDTH / 2 - 1),
            Character::new(HEIGHT / 2, WIDTH / 2 + 1),
        ];
        let mut now_h = 0;
        let mut now_w = 0;
        while init_data.len() > 0 {
            let c = init_data.remove(0);
            if c != 'A' && c != 'B' && c.to_digit(10).is_none() {
                continue;
            }

            if c == 'A' {
                characters[0].y = now_h;
                characters[0].x = now_w;
            } else if c == 'B' {
                characters[1].y = now_h;
                characters[1].x = now_w;
            } else {
                points[now_h][now_w] = c.to_digit(10).unwrap();
            }

            now_w += 1;
            if now_w == WIDTH {
                now_w = 0;
                now_h += 1;
            }
            if now_h == HEIGHT {
                break;
            }
        }
        return AlternateMazeState::new(points, characters);
    }

    fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    fn advance(&mut self, action: usize) -> Self {
        self.characters[0].y = self.characters[0].y.wrapping_add_signed(MOVE_YX[action].0);
        self.characters[0].x = self.characters[0].x.wrapping_add_signed(MOVE_YX[action].1);
        self.characters[0].game_score += self.points[self.characters[0].y][self.characters[0].x];
        self.points[self.characters[0].y][self.characters[0].x] = 0;
        self.turn += 1;
        self.characters.swap(0, 1);
        return self.clone();
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..MOVE_YX.len() {
            let ny = self.characters[0].y.wrapping_add_signed(MOVE_YX[action].0);
            let nx = self.characters[0].x.wrapping_add_signed(MOVE_YX[action].1);
            if ny < HEIGHT && nx < WIDTH {
                actions.push(action);
            }
        }
        return actions;
    }

    fn get_winning_status(&self) -> WinningStatue {
        if self.is_done() {
            if self.characters[0].game_score > self.characters[1].game_score {
                return WinningStatue::Win;
            }
            if self.characters[0].game_score < self.characters[1].game_score {
                return WinningStatue::Lose;
            } else {
                return WinningStatue::Draw;
            }
        }
        return WinningStatue::None;
    }

    fn get_score(&self, player_id: usize) -> u32 {
        let mut actual_player_id = player_id;
        if self.turn % 2 == 1 {
            actual_player_id ^= 1;
        }
        return self.characters[actual_player_id].game_score;
    }

    fn get_grid_hw(&self, h: usize, w: usize) -> String {
        let mut is_written = false;
        let mut result = String::new();
        for i in 0..self.characters.len() {
            let mut actual_player_id = i;
            if self.turn % 2 == 1 {
                actual_player_id ^= 1;
            }
            if self.characters[i].y == h && self.characters[i].x == w {
                if actual_player_id == 0 {
                    result.push_str("A");
                } else {
                    result.push_str("B");
                }
                is_written = true;
            }
        }
        if !is_written {
            if self.points[h][w] > 0 {
                result.push_str(&self.points[h][w].to_string());
            } else {
                result.push_str(".");
            }
        }
        return result;
    }
}

struct CalcedState {
    states: Vec<AlternateMazeState>,
}

impl CalcedState {
    fn new(init_data: String, mut moves: String) -> Self {
        let mut now_state = AlternateMazeState::init(init_data);
        let mut states = vec![now_state.clone(); END_TURN + 1];

        while !now_state.is_done() && moves.len() > 0 {
            let c = moves.remove(0);
            if c != 'U' && c != 'D' && c != 'L' && c != 'R' {
                continue;
            }
            if c == 'U' {
                if now_state.legal_actions().contains(&3) {
                    now_state = now_state.advance(3);
                }
            } else if c == 'D' {
                if now_state.legal_actions().contains(&2) {
                    now_state = now_state.advance(2);
                }
            } else if c == 'L' {
                if now_state.legal_actions().contains(&1) {
                    now_state = now_state.advance(1);
                }
            } else if c == 'R' {
                if now_state.legal_actions().contains(&0) {
                    now_state = now_state.advance(0);
                }
            }
            states[now_state.turn] = now_state.clone();
        }
        CalcedState { states }
    }

    fn get_winning_status(&self, turn: String) -> String {
        let turn = turn.parse::<usize>().unwrap();
        let mut turn_player = "A";
        let mut not_turn_player = "B";
        if turn % 2 == 1 {
            swap(&mut turn_player, &mut not_turn_player);
        }
        return match self.states[turn].get_winning_status() {
            WinningStatue::Win => turn_player.to_string() + "の勝ち",
            WinningStatue::Lose => not_turn_player.to_string() + "の勝ち",
            WinningStatue::Draw => "引き分け".to_string(),
            WinningStatue::None => "残り".to_string() + &(END_TURN - turn).to_string() + "ターン",
        };
    }

    fn get_score(&self, turn: String, player_id: usize) -> u32 {
        return self.states[turn.parse::<usize>().unwrap()].get_score(player_id);
    }

    fn get_grid_hw(&self, turn: String, h: usize, w: usize) -> String {
        return self.states[turn.parse::<usize>().unwrap()].get_grid_hw(h, w);
    }
}

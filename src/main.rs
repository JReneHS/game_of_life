mod cell;
mod grid;
mod structs;
mod types;

use crate::grid::Grid;
use crate::structs::*;
use crate::types::Point;
use clap::{Arg, Command};

use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

const GRID: bool = false;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: String,
}

struct MainState {
    grid: Grid,
    config: Config,
}
impl MainState {
    pub fn new(_ctx: &mut Context, config: Config) -> Self {
        // Initialize the grid based on configuration
        let mut grid = Grid::new(config.grid_width, config.grid_height);
        // Initialize starting configuration
        let mut start_cells_coords: Vec<Point> = vec![];
        match &config.initial_state[..] {
            "glider-gun" => {
                start_cells_coords = GLIDER_GUN.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "toad" => {
                start_cells_coords = TOAD.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "gliderA" => {
                start_cells_coords = GLIDER_A.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "gliderB" => {
                start_cells_coords = GLIDER_B.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "gliderC" => {
                start_cells_coords = GLIDER_C.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "gliderD" => {
                start_cells_coords = GLIDER_D.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "blinker" => {
                start_cells_coords = BLINKER.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "collisionAAI180" => {
                start_cells_coords = GLIDER_COLLISION_AAI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionABI180" => {
                start_cells_coords = GLIDER_COLLISION_ABI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionACI180" => {
                start_cells_coords = GLIDER_COLLISION_ACI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionADI180" => {
                start_cells_coords = GLIDER_COLLISION_ADI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBBI180" => {
                start_cells_coords = GLIDER_COLLISION_BBI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBCI180" => {
                start_cells_coords = GLIDER_COLLISION_BCI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBDI180" => {
                start_cells_coords = GLIDER_COLLISION_BDI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCCI180" => {
                start_cells_coords = GLIDER_COLLISION_CCI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCDI180" => {
                start_cells_coords = GLIDER_COLLISION_CDI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionDDI180" => {
                start_cells_coords = GLIDER_COLLISION_DDI180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionAAP180" => {
                start_cells_coords = GLIDER_COLLISION_AAP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionABP180" => {
                start_cells_coords = GLIDER_COLLISION_ABP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionACP180" => {
                start_cells_coords = GLIDER_COLLISION_ACP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionADP180" => {
                start_cells_coords = GLIDER_COLLISION_ADP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBBP180" => {
                start_cells_coords = GLIDER_COLLISION_BBP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBCP180" => {
                start_cells_coords = GLIDER_COLLISION_BCP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBDP180" => {
                start_cells_coords = GLIDER_COLLISION_BDP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCCP180" => {
                start_cells_coords = GLIDER_COLLISION_CCP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCDP180" => {
                start_cells_coords = GLIDER_COLLISION_CDP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionDDP180" => {
                start_cells_coords = GLIDER_COLLISION_DDP180
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionAAI90" => {
                start_cells_coords = GLIDER_COLLISION_AAI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionABI90" => {
                start_cells_coords = GLIDER_COLLISION_ABI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionACI90" => {
                start_cells_coords = GLIDER_COLLISION_ACI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionADI90" => {
                start_cells_coords = GLIDER_COLLISION_ADI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBBI90" => {
                start_cells_coords = GLIDER_COLLISION_BBI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBCI90" => {
                start_cells_coords = GLIDER_COLLISION_BCI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBDI90" => {
                start_cells_coords = GLIDER_COLLISION_BDI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCCI90" => {
                start_cells_coords = GLIDER_COLLISION_CCI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCDI90" => {
                start_cells_coords = GLIDER_COLLISION_CDI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionDDI90" => {
                start_cells_coords = GLIDER_COLLISION_DDI90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionAAP90" => {
                start_cells_coords = GLIDER_COLLISION_AAP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionABP90" => {
                start_cells_coords = GLIDER_COLLISION_ABP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionACP90" => {
                start_cells_coords = GLIDER_COLLISION_ACP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionADP90" => {
                start_cells_coords = GLIDER_COLLISION_ADP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBBP90" => {
                start_cells_coords = GLIDER_COLLISION_BBP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBCP90" => {
                start_cells_coords = GLIDER_COLLISION_BCP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionBDP90" => {
                start_cells_coords = GLIDER_COLLISION_BDP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCCP90" => {
                start_cells_coords = GLIDER_COLLISION_CCP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionCDP90" => {
                start_cells_coords = GLIDER_COLLISION_CDP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "collisionDDP90" => {
                start_cells_coords = GLIDER_COLLISION_DDP90
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "100collisionAA" => {
                start_cells_coords = GLIDER_100_COLLISION_AA
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "100collisionAC" => {
                start_cells_coords = GLIDER_100_COLLISION_AC
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "100collisionAH1" => {
                start_cells_coords = GLIDER_100_COLLISION_AH1
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            "100collisionAH" => {
                start_cells_coords = GLIDER_100_COLLISION_AH
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<Point>>();
            }
            _ => {
                let mut rng = rand::thread_rng();
                for i in 0..config.grid_width {
                    for j in 0..config.grid_height {
                        if rng.gen::<bool>() {
                            start_cells_coords.push((i, j).into());
                        }
                    }
                }
            }
        }
        // Convert the starting states into a vector of points
        grid.set_state(&start_cells_coords);
        MainState { grid, config }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, self.config.fps) {
            self.grid.update();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        // Mesh builder
        let mut builder = graphics::MeshBuilder::new();
        // Init, otherwise doesn't work for some reason
        builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., 0., 0.),
            graphics::Color::BLACK,
        )?;
        // Draw cells
        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if cell.is_alive() {
                let pos = self.grid.index_to_coords(idx);
                let color = graphics::Color::new(0., 200., 0., 1.); // Green
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                )?;
            }
        }
        // Draw grid
        if GRID {
            for idx in 0..self.grid.cells.len() {
                let color = graphics::Color::new(10., 10., 10., 1.); // ?
                let pos = self.grid.index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::stroke(1.),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                )?;
            }
        }
        let mesh = builder.build(ctx)?;
        // Draw
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // Present on screen
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // CLI
    let matches = Command::new("Game of Life")
        .version("0.1")
        .author("J. Rene H.S.")
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .help("Grid width")
                .value_name("width")
                .takes_value(true)
                .required(false)
                .default_value("400"),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .help("Grid height")
                .value_name("height")
                .takes_value(true)
                .required(false)
                .default_value("400"),
        )
        .arg(
            Arg::new("initial_state")
                .short('s')
                .long("initial-state")
                .help(
                    "Initial state options: blinker, toad, gliderA, gliderB, gliderC, gliderD, glider-gun, random, collisionAA180, collisionAB180, collisionAC180, collisionAD180, collisionBB180, collisionBC180, collisionBD180, collisionCC180, collisionCD180, collisionDD180, collisionAA90, collisionAB90, collisionAC90, collisionAD90, collisionBB90, collisionBC90, collisionBD90, collisionCC90, collisionCD90, collisionDD90, 100collisionAA , 100collisionAC , 100collisionAH, 100collisionAH1",
                )
                .value_name("initial_state")
                .takes_value(true)
                .required(false)
                .default_value("random"),
        )
        .get_matches();

    // Get Configurations
    let grid_width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let grid_height = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let initial_state = matches.value_of("initial_state").unwrap();
    let screen_size = (700.0, 700.0);
    let fps = 30;
    // Set configuration
    let config: Config = Config {
        grid_width,
        grid_height,
        cell_size: screen_size.0 / grid_width as f32,
        screen_size,
        fps,
        initial_state: initial_state.to_string(),
    };

    // Setup ggez stuff
    let cb = ContextBuilder::new("Game of life", "J. Rene H.S.")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Game of life");
    // Setup game state -> game loop
    let state = MainState::new(&mut ctx, config);

    event::run(ctx, event_loop, state);
}

use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    baddie: Baddie,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            baddie: Baddie::new(15, 25),
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.baddie = Baddie::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        //255001,  134001
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(10, "(P) Play Game");
        ctx.print_centered(15, "(H) High Scores");
        ctx.print_centered(20, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::H => self.high_score(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn high_score(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.mode = GameMode::HighScore;
        ctx.print_centered(5, "High Scores");
        ctx.print_centered(7, "TEST.........");
        ctx.print_centered(9, &format!("You reached {} points!", self.score));
        ctx.print_centered(12, "(P) Play Again");
        ctx.print_centered(15, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        if let Some(VirtualKeyCode::W) = ctx.key {
            self.player.move_forward();
        }
        if let Some(VirtualKeyCode::Q) = ctx.key {
            self.player.move_backwards();
        }
        let mut random = RandomNumberGenerator::new();
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        self.baddie.render(ctx, self.player.x, 40);
        if self.player.x > self.baddie.x {
            let mut v = vec![];
            let horde = random.range(1, 5);
            println!("{}", horde);
            for _ in 1..horde {
                v.push(Baddie::new(
                    self.player.x + SCREEN_WIDTH + 20,
                    random.range(10, 45),
                ));
            }
            for bad in v {
                self.baddie = bad;
            }
        }

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Yo you're dead!");
        ctx.print_centered(6, &format!("You reached {} points!", self.score));
        ctx.print_centered(10, "(P) Play Again");
        ctx.print_centered(15, "(H) High Scores");
        ctx.print_centered(20, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::H => self.high_score(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

enum GameMode {
    Menu,
    HighScore,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Baddie {
    x: i32,
    y: i32,
}
impl Baddie {
    fn new(x: i32, y: i32) -> Self {
        Baddie { x, y }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32, player_y: i32) {
        let screen_x = self.x - player_x;
        let screen_y = self.y;

        // Draw the baddie
        ctx.set(screen_x, screen_y, RED, BLACK, to_cp437('#'));
    }
    fn hit_baddie(&mut self) {}
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Draw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        //Draw the bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(3, self.y, RGB::from_u8(255, 101, 80), BLACK, to_cp437('☻'));
        ctx.set(2, self.y, RGB::from_u8(255, 101, 80), BLACK, to_cp437('∟'));
        ctx.set(4, self.y, RGB::from_u8(255, 101, 80), BLACK, to_cp437('┘'));
        ctx.set(
            3,
            self.y + 1,
            RGB::from_u8(255, 101, 80),
            BLACK,
            to_cp437('║'),
        );
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn move_forward(&mut self) {
        self.x += 0;
    }
    fn move_backwards(&mut self) {
        self.x -= 1;
    }
    fn flap(&mut self) {
        self.velocity = -2.0
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::HighScore => self.high_score(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}

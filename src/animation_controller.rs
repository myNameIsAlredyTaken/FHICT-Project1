use bevy::{prelude::*, time::TimerMode::Repeating};


#[derive(Resource)]
pub(super) struct SpriteSheet(pub Handle<TextureAtlasLayout>);


#[derive(Component, Debug, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        println!("Timer Constructed");


        AnimationTimer(Timer::from_seconds(0.1, Repeating))
    }
}


#[derive(Component, Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    Idle,
    Walk,
}


#[derive(Component, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Component, Debug)]
pub struct AnimationController {
    timer: AnimationTimer,
    state: State,
    direction: Direction,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            timer: AnimationTimer::default(),
            state: State::Idle,
            direction: Direction::Down,
        }
    }
}

impl AnimationController {
    pub fn change_dir(&mut self, dir: Direction) {
        if self.direction == dir {
            return
        }
        self.direction = dir;
    }

    pub fn change_state(&mut self, state: State) {
        if self.state == state {
            return
        }
        self.state = state;
    }
}


fn animate_sprites(
    query: Query<(&mut AnimationController, &mut Sprite)>,
    time: Res<Time>
) {
    for (
        mut controller,
        mut sprite
    ) in query {
        let atlas = sprite.texture_atlas.as_mut().unwrap();

        controller.timer.tick(time.delta()); 
        if controller.timer.just_finished() {
            match controller.state {
                State::Idle => {
                    atlas.index = match controller.direction {
                        Direction::Up => 8,
                        Direction::Down => 0,
                        Direction::Left => 24,
                        Direction::Right => 16
                    };
                },
                State::Walk => {
                    match controller.direction {
                        Direction::Up => {
                            if atlas.index > 44 || atlas.index < 39 { 
                                atlas.index = 40;
                            } else {
                                atlas.index += 1;
                            }
                        },
                        Direction::Down => {
                            if atlas.index > 36 || atlas.index < 31 { 
                                atlas.index = 32;
                            } else {
                                atlas.index += 1;
                            }
                        },
                        Direction::Left => {
                            if atlas.index > 60 || atlas.index < 56 { 
                                atlas.index = 56;
                            } else {
                                atlas.index += 1;
                            }
                        },
                        Direction::Right => {
                            if atlas.index > 52 || atlas.index < 48 { 
                                atlas.index = 48;
                            } else {
                                atlas.index += 1;
                            }
                        },
                    }
                }
            }
        }
    }
}


// Defining a resource PlayerSpriteSheet
// so each time it's initialized it loads this configuration
impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (20, 29).into(),
            8,
            8,
            Some(UVec2::new(44, 35)),
            Some(UVec2::new(24, 15)),
        );

        let mut texture_atlases = 
             world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let texture_atlas_handle =
        texture_atlases.add(texture_atlas);

        Self(texture_atlas_handle)
   
    }
}


pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpriteSheet>();
    app.add_systems(Update, animate_sprites);
}

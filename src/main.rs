use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PieceType {
    Red2x2,
    Yellow1x2,
    Yellow2x1,
    Blue1x1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Piece {
    piece_type: PieceType,
    x: u8,
    y: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    fn new() -> Self {
        let mut pieces = Vec::new();
        pieces.push(Piece {
            piece_type: PieceType::Red2x2,
            x: 1,
            y: 0,
        });
        pieces.push(Piece {
            piece_type: PieceType::Yellow1x2,
            x: 0,
            y: 0,
        });
        pieces.push(Piece {
            piece_type: PieceType::Yellow1x2,
            x: 3,
            y: 0,
        });
        pieces.push(Piece {
            piece_type: PieceType::Yellow1x2,
            x: 0,
            y: 2,
        });
        pieces.push(Piece {
            piece_type: PieceType::Yellow1x2,
            x: 3,
            y: 2,
        });
        pieces.push(Piece {
            piece_type: PieceType::Yellow2x1,
            x: 1,
            y: 2,
        });
        pieces.push(Piece {
            piece_type: PieceType::Blue1x1,
            x: 1,
            y: 3,
        });
        pieces.push(Piece {
            piece_type: PieceType::Blue1x1,
            x: 2,
            y: 3,
        });
        pieces.push(Piece {
            piece_type: PieceType::Blue1x1,
            x: 1,
            y: 4,
        });
        pieces.push(Piece {
            piece_type: PieceType::Blue1x1,
            x: 2,
            y: 4,
        });
        Board { pieces }
    }

    fn to_grid(&self) -> [[i8; 4]; 5] {
        let mut grid = [[-1; 4]; 5];
        for (idx, p) in self.pieces.iter().enumerate() {
            let (w, h) = match p.piece_type {
                PieceType::Red2x2 => (2, 2),
                PieceType::Yellow1x2 => (1, 2),
                PieceType::Yellow2x1 => (2, 1),
                PieceType::Blue1x1 => (1, 1),
            };
            for dx in 0..w {
                for dy in 0..h {
                    grid[(p.y + dy) as usize][(p.x + dx) as usize] = idx as i8;
                }
            }
        }
        grid
    }

    fn canonicalize(&mut self) {
        // Find the red piece first to keep it as the first element if possible,
        // but the key is consistent identification of the OTHER pieces.
        // We'll sort by type, then position.
        self.pieces.sort_by(|a, b| {
            if a.piece_type != b.piece_type {
                return a.piece_type.cmp(&b.piece_type);
            }
            (a.y, a.x).cmp(&(b.y, b.x))
        });
    }

    fn is_goal(&self) -> bool {
        for p in &self.pieces {
            if p.piece_type == PieceType::Red2x2 {
                return p.x == 1 && p.y == 3;
            }
        }
        false
    }

    fn get_moves(&self) -> Vec<Board> {
        let grid = self.to_grid();
        let mut moves = Vec::new();
        for (idx, p) in self.pieces.iter().enumerate() {
            let (w, h) = match p.piece_type {
                PieceType::Red2x2 => (2, 2),
                PieceType::Yellow1x2 => (1, 2),
                PieceType::Yellow2x1 => (2, 1),
                PieceType::Blue1x1 => (1, 1),
            };
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let nx = p.x as i8 + dx;
                let ny = p.y as i8 + dy;
                if nx >= 0 && ny >= 0 && nx + w as i8 <= 4 && ny + h as i8 <= 5 {
                    let mut possible = true;
                    for ox in 0..w {
                        for oy in 0..h {
                            let tx = nx + ox as i8;
                            let ty = ny + oy as i8;
                            let occupant = grid[ty as usize][tx as usize];
                            if occupant != -1 && occupant != idx as i8 {
                                possible = false;
                                break;
                            }
                        }
                        if !possible {
                            break;
                        }
                    }
                    if possible {
                        let mut next_board = self.clone();
                        next_board.pieces[idx].x = nx as u8;
                        next_board.pieces[idx].y = ny as u8;
                        next_board.canonicalize();
                        moves.push(next_board);
                    }
                }
            }
        }
        moves
    }
}

#[derive(Resource)]
struct SolutionPath {
    steps: Vec<Board>,
    current_idx: usize,
}

#[derive(Component)]
struct PieceEntity {
    piece_type: PieceType,
    original_idx: usize,
}

#[derive(Component)]
enum NavButton {
    Back,
    Forward,
}

const TILE_SIZE: f32 = 100.0;
const MARGIN: f32 = 5.0;

fn main() {
    let mut start = Board::new();
    start.canonicalize();

    let mut path = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), vec![start.clone()]));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((current, p)) = queue.pop_front() {
        if current.is_goal() {
            path = p;
            break;
        }
        for next in current.get_moves() {
            if !visited.contains(&next) {
                visited.insert(next.clone());
                let mut next_path = p.clone();
                next_path.push(next.clone());
                queue.push_back((next, next_path));
            }
        }
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Block Shifting Puzzle Solver".into(),
                resolution: (600.0, 700.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SolutionPath {
            steps: path,
            current_idx: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, update_pieces))
        .run();
}

fn setup(mut commands: Commands, solution: Res<SolutionPath>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn board background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(4.0 * TILE_SIZE + 10.0, 5.0 * TILE_SIZE + 10.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // Initial pieces
    if let Some(board) = solution.steps.get(0) {
        for (idx, piece) in board.pieces.iter().enumerate() {
            let color = match piece.piece_type {
                PieceType::Red2x2 => Color::srgb(0.8, 0.2, 0.2),
                PieceType::Yellow1x2 | PieceType::Yellow2x1 => Color::srgb(0.8, 0.8, 0.2),
                PieceType::Blue1x1 => Color::srgb(0.2, 0.2, 0.8),
            };
            let (w, h) = match piece.piece_type {
                PieceType::Red2x2 => (2.0, 2.0),
                PieceType::Yellow1x2 => (1.0, 2.0),
                PieceType::Yellow2x1 => (2.0, 1.0),
                PieceType::Blue1x1 => (1.0, 1.0),
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(
                            w * TILE_SIZE - MARGIN,
                            h * TILE_SIZE - MARGIN,
                        )),
                        ..default()
                    },
                    ..default()
                },
                PieceEntity {
                    piece_type: piece.piece_type,
                    original_idx: idx,
                },
            ));
        }
    }

    // UI Buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::End,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    NavButton::Back,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    NavButton::Forward,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Forward",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &NavButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut solution: ResMut<SolutionPath>,
) {
    for (interaction, nav, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                match nav {
                    NavButton::Back => {
                        if solution.current_idx > 0 {
                            solution.current_idx -= 1;
                        }
                    }
                    NavButton::Forward => {
                        if solution.current_idx < solution.steps.len() - 1 {
                            solution.current_idx += 1;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn update_pieces(
    solution: Res<SolutionPath>,
    mut query: Query<(Entity, &PieceEntity, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some(board) = solution.steps.get(solution.current_idx) {
        let mut board_pieces_by_type: std::collections::HashMap<PieceType, Vec<&Piece>> =
            std::collections::HashMap::new();
        for piece in &board.pieces {
            board_pieces_by_type
                .entry(piece.piece_type)
                .or_default()
                .push(piece);
        }
        for pieces in board_pieces_by_type.values_mut() {
            pieces.sort_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
        }

        let mut entity_data: Vec<(Entity, PieceType, usize)> = Vec::new();
        for (entity, pe, _) in query.iter() {
            entity_data.push((entity, pe.piece_type, pe.original_idx));
        }
        entity_data.sort_by_key(|(_, _, idx)| *idx);

        let mut entities_by_type: std::collections::HashMap<PieceType, Vec<Entity>> =
            std::collections::HashMap::new();
        for (entity, ptype, _) in entity_data {
            entities_by_type.entry(ptype).or_default().push(entity);
        }

        for (ptype, entities) in entities_by_type {
            if let Some(pieces) = board_pieces_by_type.get(&ptype) {
                for (i, entity) in entities.into_iter().enumerate() {
                    if let Some(piece) = pieces.get(i) {
                        if let Ok((_, _, mut transform)) = query.get_mut(entity) {
                            let (w, h) = match piece.piece_type {
                                PieceType::Red2x2 => (2.0, 2.0),
                                PieceType::Yellow1x2 => (1.0, 2.0),
                                PieceType::Yellow2x1 => (2.0, 1.0),
                                PieceType::Blue1x1 => (1.0, 1.0),
                            };
                            let target_x = (piece.x as f32 - 2.0 + w / 2.0) * TILE_SIZE;
                            let target_y = (2.5 - piece.y as f32 - h / 2.0) * TILE_SIZE;
                            let target_pos = Vec3::new(target_x, target_y, 0.0);
                            transform.translation = transform
                                .translation
                                .lerp(target_pos, time.delta_seconds() * 10.0);
                        }
                    }
                }
            }
        }
    }
}

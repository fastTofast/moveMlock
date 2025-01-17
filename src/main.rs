// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
// use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

extern crate random_number;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Cell {
    w: f32,
    h: f32,
}

fn add_person(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let h = 35.0;
    let w = 60.0;
    fn get_color(n: u8) -> Color {
        let color_list = [
            Color::rgb(0.6, 0.9, 0.6),
            Color::rgb(0.1, 0.9, 0.8),
            Color::rgb(0.9, 0.9, 0.1),
            Color::rgb(0.7, 0.4, 0.9),
            Color::rgb(0.9, 0.7, 0.7),
        ];
        if 0 <= n && n <= 4 {
            color_list[n as usize]
        } else {
            Color::rgb(0.5, 0.5, 0.3)
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            let x = if i < 5 {
                i as f32 * (w + 5.0)
            } else {
                (i - 10) as f32 * (w + 5.0)
            };
            let y = if j < 5 {
                j as f32 * (h + 5.0)
            } else {
                (j - 10) as f32 * (h + 5.0)
            };
            // let mut color = [0u8; 3];
            // random_number::random_fill_ranged(&mut color, 1..10);
            let r: u8 = random_number::random_ranged(0..5);
            println!("x: {x}, y: {y}, r: {r}");
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: get_color(r),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(w, h, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Cell { w, h });
        }
    }
}
// #[derive(Inspectable, Default)]
struct MouseStatus {
    x: f32,
    y: f32,
    pressed: bool,
    released: bool,
}
struct PositionTimer(Timer);
#[derive(Component)]
struct SelectedCellCom(Entity);
// #[derive(Inspectable, Default)]
struct SelectedCell {
    oldPos: Vec3,
    newPos: Vec3,
}

fn timer_fn(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected_cell: ResMut<SelectedCell>,
    mut mousestatus: ResMut<MouseStatus>,
    mut query: Query<(&Cell, &mut Transform, Entity)>,
) {
    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let win = windows.get_primary().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = win.cursor_position() {
            mousestatus.x = pos.x;
            mousestatus.y = pos.y;
        } else {
            println!("鼠标不在窗口内");
        };
        // Left button was pressed
        println!("just_pressed:");
        mousestatus.pressed = true;
        mousestatus.released = false;
        println!("mousestatus: x: {}  y: {}", mousestatus.x, mousestatus.y);
        for (cell, mut transform, entity) in query.iter_mut() {
            translation = transform.translation;
            if is_in(translation, &mousestatus, &cell, win) {
                println!(
                    "++++++++++++++++++++++++just_pressed+++++++++++++++++++++++++ x: {} y: {}",
                    translation.x, translation.y
                );
                selected_cell.oldPos.x = translation.x;
                selected_cell.oldPos.y = translation.y;
                selected_cell.newPos.x = translation.x;
                selected_cell.newPos.y = translation.y;
                transform.scale = Vec3::new(cell.w - 5.0, cell.h - 5.0, 0.0);
                commands.entity(entity).insert(SelectedCellCom(entity));
                translation.x = mousestatus.x;
                translation.y = mousestatus.y;
                break;
            } else {
                translation = Vec3::new(0.0, 0.0, 0.0);
            }
        }
    };
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
        println!("pressed:");
    };
}
fn is_in(translation: Vec3, mousestatus: &MouseStatus, cell: &Cell, win: &Window) -> bool {
    let w_x = win.requested_width() / 2.0;
    let w_y = win.requested_height() / 2.0;
    let abs_x = translation.x + w_x;
    let abs_y = translation.y + w_y;
    // 精灵图的坐标系原点是窗口居中的，transform的（0，0）为精灵图居中点
    let inx = abs_x - cell.w / 2.0 <= mousestatus.x && mousestatus.x <= abs_x + cell.w / 2.0;
    let iny = abs_y - cell.h / 2.0 <= mousestatus.y && mousestatus.y <= abs_y + cell.h / 2.0;
    // println!(
    //     "translation  x: {} y: {}  ---  mousestatus  x: {} y: {}  ---  abs_x {} abs_y: {}  ---  inx {} iny: {}",
    //     translation.x, translation.y, mousestatus.x, mousestatus.y, abs_x, abs_y, inx, iny
    // );
    inx && iny
}

fn release_cell(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut mousestatus: ResMut<MouseStatus>,
    mut selected_cell: ResMut<SelectedCell>,
    mut query: Query<(&Cell, &mut Transform), Without<SelectedCellCom>>,
) {
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        println!("release_cell just_released");
        let win = windows.get_primary().unwrap();
        if let Some(pos) = win.cursor_position() {
            mousestatus.x = pos.x;
            mousestatus.y = pos.y;
        }
        println!("mousestatus: x: {}  y: {}", mousestatus.x, mousestatus.y);
        for (cell, mut transform) in query.iter_mut() {
            let translation = transform.translation;
            if is_in(translation, &mousestatus, &cell, win) {
                println!(
                    "+++++++++++++++++++release_cell+++++++++++++++++++ x: {} y: {}",
                    translation.x, translation.y
                );
                selected_cell.newPos = Vec3::new(translation.x, translation.y, translation.z);
                transform.translation.x = selected_cell.oldPos.x;
                transform.translation.y = selected_cell.oldPos.y;
                break;
            }
        }
    };
}
fn release_cell_selected(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    selected_cell: Res<SelectedCell>,
    windows: Res<Windows>,
    mut mousestatus: ResMut<MouseStatus>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut query: Query<(&mut Transform, Entity, &Cell), With<SelectedCellCom>>,
) {
    for (mut transform, entity, cell) in query.iter_mut() {
        let win = windows.get_primary().unwrap();
        let win_w_half = win.requested_width() / 2.0;
        let win_h_half = win.requested_height() / 2.0;
        if buttons.just_released(MouseButton::Left) {
            println!("release_cell_selected just_released:");
            if mousestatus.pressed {
                transform.translation.x = selected_cell.newPos.x;
                transform.translation.y = selected_cell.newPos.y;
                transform.scale = Vec3::new(cell.w, cell.h, 0.0);
                commands.entity(entity).remove::<SelectedCellCom>();
            }
            mousestatus.pressed = false;
            mousestatus.released = true;
        }
        for ev in cursor_evr.iter() {
            if mousestatus.pressed {
                transform.translation.x = ev.position.x
                    - win_w_half
                    - (mousestatus.x - win_w_half - selected_cell.oldPos.x);
                transform.translation.y = ev.position.y
                    - win_h_half
                    - (mousestatus.y - win_h_half - selected_cell.oldPos.y);
                // println!(
                //     "ev.position== x: {} y: {}  |||| translation: {} {}",
                //     ev.position.x, ev.position.y, transform.translation.x, transform.translation.y
                // )
            };
        }
    }
}
fn timer_print(mut timer: ResMut<PositionTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("timer_print");
    };
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<SelectedCell>::new())
        // .add_plugin(InspectorPlugin::<MouseStatus>::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PositionTimer(Timer::from_seconds(2.0, true)))
            .insert_resource(SelectedCell {
                oldPos: Vec3::new(0.0, 0.0, 0.0),
                newPos: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert_resource(MouseStatus {
                x: 0.0,
                y: 0.0,
                pressed: false,
                released: false,
            })
            .add_startup_system(add_person)
            .add_system(timer_fn)
            .add_system(release_cell.label("release_cell"))
            .add_system(release_cell_selected.after("release_cell"));
        // .add_system(cursor_pos)
        // .add_system(mouse_button_input);
    }
}

mod egui;

use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<egui::EguiWantsFocus>()
      .add_systems(Startup, setup_camera)
      .add_systems(
        Update,
        egui::check_egui_wants_focus.after(bevy_egui::EguiSet::InitContexts),
      )
      .add_systems(
        Update,
        (pan_camera_on_drag, zoom_camera_on_scroll)
          .after(egui::check_egui_wants_focus)
          .run_if(egui::not_interfering_with_egui),
      );
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

#[derive(Default)]
enum MousePanning {
  #[default]
  None,
  Tethered(Vec2),
}

fn pan_camera_on_drag(
  mut camera_q: Query<(&Camera, &mut Transform, &GlobalTransform)>,
  mouse_button_input: Res<Input<MouseButton>>,
  window_q: Query<&Window, With<PrimaryWindow>>,
  mut last_mouse_position: Local<MousePanning>,
  mut gizmos: Gizmos,
) {
  // fetch the worldspace position of the mouse
  let Some(screenspace_position) = window_q.single().cursor_position() else {
    return;
  };
  let (camera, mut transform, global_transform) = camera_q.single_mut();
  let Some(worldspace_position) =
    camera.viewport_to_world_2d(global_transform, screenspace_position)
  else {
    return;
  };

  match *last_mouse_position {
    // if we're not currently panning, check if we should start
    MousePanning::None => {
      if mouse_button_input.just_pressed(MouseButton::Left) {
        *last_mouse_position = MousePanning::Tethered(worldspace_position);
      }
    }
    MousePanning::Tethered(tethered_point) => {
      // update the camera position
      if mouse_button_input.pressed(MouseButton::Left) {
        let delta = worldspace_position - tethered_point;
        transform.translation -= delta.extend(0.0);
        gizmos.circle_2d(worldspace_position, 10., Color::WHITE);
      // stop panning
      } else {
        *last_mouse_position = MousePanning::None;
      }
    }
  }
}

fn zoom_camera_on_scroll(
  mut camera_q: Query<&mut OrthographicProjection, With<Camera>>,
  mut mouse_wheel_events: EventReader<MouseWheel>,
) {
  let Some(mut projection) = camera_q.iter_mut().next() else {
    return;
  };

  for event in mouse_wheel_events.read() {
    projection.scale *= 1.0 - event.y * 0.005;
  }
}

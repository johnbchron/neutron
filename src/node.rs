use bevy::{
  prelude::*,
  render::view::NoFrustumCulling,
  sprite::{Anchor, MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
  text::Text2dBounds,
};

use crate::{
  curve_material::CurveMaterial, rounded_box::RoundedBoxMaterial,
  style::ThemeColor, NODE_HEIGHT, NODE_LEVEL_SEP, NODE_MARGIN, NODE_PADDING,
  NODE_WIDTH,
};

pub struct Node {
  dep:      Dependency,
  children: Vec<Node>,
}

impl Node {
  fn recursive_width(&self) -> f32 {
    let mut width = 0.0;
    for child in &self.children {
      width += child.recursive_width();
    }
    width += (NODE_MARGIN * 2.0) * (self.children.len() as f32 - 1.0);
    width.max(NODE_WIDTH)
  }

  fn spawn(
    &self,
    commands: &mut ChildBuilder,
    quad_handle: Mesh2dHandle,
    font_styles: &FontStyles,
    rounded_box_materials: &mut Assets<RoundedBoxMaterial>,
    curve_materials: &mut Assets<CurveMaterial>,
  ) {
    commands
      .spawn((
        MaterialMesh2dBundle {
          mesh: quad_handle.clone(),
          material: rounded_box_materials.add(RoundedBoxMaterial::default()),
          transform: Transform::from_xyz(0.0, 0.0, 1.0),
          ..default()
        },
        NoFrustumCulling,
        Name::new("node"),
      ))
      .with_children(|parent| {
        // spawn the text
        let text: Text = self.dep.to_text(font_styles);

        // spawn text
        parent.spawn((
          Text2dBundle {
            text,
            text_anchor: Anchor::TopLeft,
            text_2d_bounds: Text2dBounds {
              size: Vec2::new(
                NODE_WIDTH - NODE_PADDING * 2.0,
                NODE_HEIGHT - NODE_PADDING * 2.0,
              ),
              ..default()
            },
            transform: Transform::from_translation(Vec3::new(
              -(NODE_WIDTH / 2.0) + NODE_PADDING,
              (NODE_HEIGHT / 2.0) - NODE_PADDING,
              0.0,
            )),
            ..default()
          },
          Name::new("text"),
        ));

        // spawn children
        let mut offset = Vec2::new(
          -self.recursive_width() / 2.0,
          -NODE_HEIGHT - (NODE_LEVEL_SEP * 2.0),
        );
        for child in &self.children {
          offset.x += child.recursive_width() / 2.0;
          parent
            .spawn((
              SpatialBundle::from_transform(Transform::from_xyz(
                offset.x, offset.y, 0.0,
              )),
              Name::new("node_container"),
            ))
            .with_children(|parent| {
              child.spawn(
                parent,
                quad_handle.clone(),
                font_styles,
                rounded_box_materials,
                curve_materials,
              );
            });

          // spawn curve
          let point_a = Vec2::new(0.0, -NODE_HEIGHT / 2.0);
          let point_d = Vec2::new(offset.x, offset.y + NODE_HEIGHT / 2.0);
          parent.spawn((
            MaterialMesh2dBundle {
              mesh: quad_handle.clone(),
              transform: Transform::from_xyz(0.0, 0.0, -1.0),
              material: curve_materials.add(CurveMaterial {
                point_a,
                point_b: Vec2::new(
                  point_a.x,
                  point_a.y + (point_d.y - point_a.y) / 2.0,
                ),
                point_c: Vec2::new(
                  point_d.x,
                  point_a.y + (point_d.y - point_a.y) / 2.0,
                ),
                point_d,
                ..default()
              }),
              ..default()
            },
            NoFrustumCulling,
            Name::new("curve"),
          ));

          offset.x += child.recursive_width() / 2.0 + NODE_MARGIN * 2.0;
        }
      });
  }
}

#[derive(Clone)]
pub enum Dependency {
  Task {
    title: String,
    desc:  Option<String>,
  },
  Condition {
    title: String,
    desc:  Option<String>,
  },
  Circumstance {
    thing: String,
    state: String,
  },
}

impl Dependency {
  fn to_text(&self, font_styles: &FontStyles) -> Text {
    match self {
      Dependency::Task { title, desc }
      | Dependency::Condition { title, desc } => {
        let mut sections =
          vec![TextSection::new(title, font_styles.title.clone())];
        if let Some(desc) = desc {
          sections.push(TextSection::new("\n", font_styles.title.clone()));
          sections.push(TextSection::new(desc, font_styles.text.clone()));
        }
        Text::from_sections(sections)
      }
      Dependency::Circumstance { thing, state } => Text::from_sections(vec![
        TextSection::new(thing, font_styles.title.clone()),
        TextSection::new("\n", font_styles.title.clone()),
        TextSection::new(state, font_styles.text.clone()),
      ]),
    }
    .with_alignment(TextAlignment::Left)
  }
}

#[derive(Resource)]
pub struct RootNode(pub Node);

impl Default for RootNode {
  fn default() -> Self {
    Self(Node {
      dep:      Dependency::Task {
        title: "Write Neutron".to_string(),
        desc:  Some("Root node".to_string()),
      },
      children: vec![
        Node {
          dep:      Dependency::Task {
            title: "Implement the DAG".to_string(),
            desc:  Some(
              "Implement the DAG in Rust, using Bevy for the UI and \
               visualization."
                .to_string(),
            ),
          },
          children: vec![
            Node {
              dep:      Dependency::Task {
                title: "Plan the DAG".to_string(),
                desc:  Some(
                  "Figure out how to structure the DAG and what constraints \
                   are practical."
                    .to_string(),
                ),
              },
              children: vec![],
            },
            Node {
              dep:      Dependency::Circumstance {
                thing: "Mental Stability".to_string(),
                state: "Within grasp".to_string(),
              },
              children: vec![],
            },
          ],
        },
        Node {
          dep:      Dependency::Circumstance {
            thing: "Spare Time".to_string(),
            state: ">= 10 hours".to_string(),
          },
          children: vec![],
        },
      ],
    })
  }
}

#[derive(Resource)]
pub struct FontStyles {
  pub title: TextStyle,
  pub text:  TextStyle,
}

impl FromWorld for FontStyles {
  fn from_world(world: &mut World) -> Self {
    Self {
      title: TextStyle {
        font:      world
          .resource::<AssetServer>()
          .load("fonts/FiraMono-Medium.ttf"),
        font_size: 32.0,
        color:     ThemeColor::Text.color(),
      },
      text:  TextStyle {
        font:      world
          .resource::<AssetServer>()
          .load("fonts/FiraMono-Medium.ttf"),
        font_size: 16.0,
        color:     ThemeColor::Text.color(),
      },
    }
  }
}

pub struct NodePlugin;

impl Plugin for NodePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<FontStyles>()
      .init_resource::<RootNode>()
      .add_systems(Startup, setup_root_node);
  }
}

fn setup_root_node(
  mut commands: Commands,
  root_node: Res<RootNode>,
  font_styles: Res<FontStyles>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut rounded_box_materials: ResMut<Assets<RoundedBoxMaterial>>,
  mut curve_materials: ResMut<Assets<CurveMaterial>>,
) {
  commands
    .spawn((SpatialBundle::default(), Name::new("top_level_container")))
    .with_children(|parent| {
      root_node.0.spawn(
        parent,
        meshes.add(Mesh::from(shape::Quad::default())).into(),
        &font_styles,
        &mut rounded_box_materials,
        &mut curve_materials,
      );
    });
}

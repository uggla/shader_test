//! A shader and a material that uses it.
use clap::{Parser, ValueEnum};

use bevy::{
    color,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Shader name
    name: ShaderNameValue,
}

#[derive(Clone, ValueEnum, Debug, PartialEq, Eq)]
enum ShaderNameValue {
    Water,
    Truc,
}

#[derive(Resource)]
struct ShaderName(ShaderNameValue);

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    println!("Value for name: {:?}", cli.name);

    App::new()
        .insert_resource(ShaderName(cli.name))
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    shader_name: Res<ShaderName>,
    // asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    match shader_name.0 {
        ShaderNameValue::Water => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: materials.add(CustomMaterial {
                    // color: LinearRgba::GREEN, // color_texture: Some(asset_server.load("icon.png")),
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::Truc => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: materials.add(TrucMaterial {
                    // color: LinearRgba::GREEN, // color_texture: Some(asset_server.load("icon.png")),
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "water_material.wgsl".into()
    }
}

impl From<TrucMaterial> for CustomMaterial {
    fn from(material: TrucMaterial) -> Self {
        CustomMaterial {
            color: material.color,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TrucMaterial {
    #[uniform(0)]
    color: LinearRgba,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
}

impl Material2d for TrucMaterial {
    fn fragment_shader() -> ShaderRef {
        "truc_material.wgsl".into()
    }
}

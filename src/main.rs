//! A shader and a material that uses it.
use clap::{Parser, ValueEnum};

use bevy::{
    color,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    name: ShaderNameValue,
}

#[derive(Clone, ValueEnum, Debug, PartialEq, Eq)]
enum ShaderNameValue {
    Water,
    Goldcube,
    Circle,
    HypnoticCircle,
    Crystal,
    Stars,
    Smoke,
    SmokeRust,
}

#[derive(Resource)]
struct ShaderName(ShaderNameValue);

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    dbg!("Value for name: {:?}", &cli.name);

    let mut app = App::new();

    app.insert_resource(ShaderName(cli.name))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup);

    app.add_plugins(Material2dPlugin::<WaterMaterial>::default());
    app.add_plugins(Material2dPlugin::<GoldcubeMaterial>::default());
    app.add_plugins(Material2dPlugin::<CircleMaterial>::default());
    app.add_plugins(Material2dPlugin::<HypnoticCircleMaterial>::default());
    app.add_plugins(Material2dPlugin::<CrystalMaterial>::default());
    app.add_plugins(Material2dPlugin::<StarsMaterial>::default());
    app.add_plugins(Material2dPlugin::<SmokeMaterial>::default());
    app.add_plugins(Material2dPlugin::<SmokeRustMaterial>::default());

    app.run();
}

// Setup a simple 2d scene
#[allow(clippy::too_many_arguments)]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water: ResMut<Assets<WaterMaterial>>,
    mut gold_cube: ResMut<Assets<GoldcubeMaterial>>,
    mut circle: ResMut<Assets<CircleMaterial>>,
    mut hypnotic_circle: ResMut<Assets<HypnoticCircleMaterial>>,
    mut crystal: ResMut<Assets<CrystalMaterial>>,
    mut stars: ResMut<Assets<StarsMaterial>>,
    mut smoke: ResMut<Assets<SmokeMaterial>>,
    mut smoke_rust: ResMut<Assets<SmokeRustMaterial>>,
    shader_name: Res<ShaderName>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    match shader_name.0 {
        ShaderNameValue::Stars => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                // transform: Transform::default().with_scale(Vec3::splat(720.)),
                transform: Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
                material: stars.add(StarsMaterial {
                    // color_texture: Some(asset_server.load("icon.png")),
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::Smoke => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                // transform: Transform::default().with_scale(Vec3::splat(720.)),
                transform: Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
                material: smoke.add(SmokeMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                    // color_texture: Some(asset_server.load("icon.png")),
                }),
                ..default()
            });
        }
        ShaderNameValue::SmokeRust => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                // transform: Transform::default().with_scale(Vec3::splat(720.)),
                transform: Transform::default().with_scale(Vec3::new(720.0, 720.0, 1.0)),
                material: smoke_rust.add(SmokeRustMaterial {
                    color_texture: Some(asset_server.load("rust_logo.png")),
                    color: LinearRgba {
                        red: 1.0,
                        green: 0.44,
                        blue: 0.04,
                        alpha: 1.0,
                    },
                }),
                ..default()
            });
        }
        ShaderNameValue::Crystal => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                // transform: Transform::default().with_scale(Vec3::splat(720.)),
                transform: Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
                material: crystal.add(CrystalMaterial {
                    // color_texture: Some(asset_server.load("icon.png")),
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::Water => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: water.add(WaterMaterial {
                    // color_texture: Some(asset_server.load("icon.png")),
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::Goldcube => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: gold_cube.add(GoldcubeMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::Circle => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: circle.add(CircleMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
        ShaderNameValue::HypnoticCircle => {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(720.)),
                material: hypnotic_circle.add(HypnoticCircleMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                }),
                ..default()
            });
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(0)]
    color: LinearRgba,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "water_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GoldcubeMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for GoldcubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "gold_cube_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CircleMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "circle_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct HypnoticCircleMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for HypnoticCircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "hypnotic_circle_material.wgsl".into()
    }
}

impl Material2d for CrystalMaterial {
    fn fragment_shader() -> ShaderRef {
        "crystal_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CrystalMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for StarsMaterial {
    fn fragment_shader() -> ShaderRef {
        "stars_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StarsMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for SmokeMaterial {
    fn fragment_shader() -> ShaderRef {
        "smoke_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SmokeMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for SmokeRustMaterial {
    fn fragment_shader() -> ShaderRef {
        "smoke_rust_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SmokeRustMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

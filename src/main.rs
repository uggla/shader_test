//! A shader and a material that uses it.
use clap::{Parser, ValueEnum};

use bevy::{
    color,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
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
    Snow,
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
    app.add_plugins(Material2dPlugin::<SnowMaterial>::default());

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
    mut snow: ResMut<Assets<SnowMaterial>>,
    shader_name: Res<ShaderName>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d);

    // quad
    match shader_name.0 {
        ShaderNameValue::Stars => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(stars.add(StarsMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }
        ShaderNameValue::Smoke => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(smoke.add(SmokeMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::SmokeRust => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(smoke_rust.add(SmokeRustMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                    color_texture: Some(asset_server.load("rust_logo.png")),
                })),
                Transform::default().with_scale(Vec3::new(720.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::Crystal => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(crystal.add(CrystalMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::Water => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(water.add(WaterMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::Goldcube => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(gold_cube.add(GoldcubeMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::Circle => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(circle.add(CircleMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::HypnoticCircle => {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(hypnotic_circle.add(HypnoticCircleMaterial {
                    color: LinearRgba::from(color::palettes::css::GOLD),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
        }

        ShaderNameValue::Snow => {
            // Background image
            commands.spawn((
                Sprite::from_image(asset_server.load("photo.png")),
                Transform::default().with_translation(Vec3::new(0.0, 0.0, -1.0)),
            ));
            
            // Snow overlay
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(snow.add(SnowMaterial {
                    color: LinearRgba::from(color::palettes::css::WHITE),
                })),
                Transform::default().with_scale(Vec3::new(1280.0, 720.0, 1.0)),
            ));
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

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SnowMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for SnowMaterial {
    fn fragment_shader() -> ShaderRef {
        "snow_material.wgsl".into()
    }
    
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

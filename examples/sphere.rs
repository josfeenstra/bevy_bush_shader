use bevy::color::palettes::css;
use hedron::prelude::*;
use ribit::prelude::*;
use ribit::re_export::inspect::*;
use shaders::prelude::*;
use strum::Display;

#[derive(Component)]
struct Local;

pub struct BushPlugin;

impl BoundPlugin for BushPlugin {
    fn build<T: States + Copy + Default>(app: &mut App, state: T) {
        app.register_type::<Shaper>();
        build_demo_scene_plugin(
            app,
            state,
            "Bush Shader".into(),
            "Bush Shader".into(),
            Settings::default(),
            on_changed,
        );
        app.add_bound_component::<Local>(state).add_bound_systems(
            state,
            on_enter,
            || {},
            on_exit,
            AppSet::Update,
        );
    }
}

#[derive(Reflect, InspectorOptions, Clone, Resource)]
#[reflect(InspectorOptions)]
struct Settings {
    #[inspector(min = 0.0, max = 100.0, speed = 1.0)]
    pub displace: f32,
    #[inspector(min = 1.00, max = 20.0, speed = 0.1)]
    pub size: f32,
    #[inspector()]
    pub u_steps: usize,
    #[inspector()]
    pub v_steps: usize,

    #[inspector()]
    pub dark: Color,
    #[inspector()]
    pub mid: Color,
    #[inspector()]
    pub light: Color,

    #[inspector(min = 0.0, max = 50.0, speed = 0.1)]
    pub offset_size: f32,
    #[inspector(min = -10.0, max = 10.0, speed = 0.1)]
    pub rotation_factor: f32,

    #[inspector()]
    pub texture: Texture,
}

#[derive(Reflect, InspectorOptions, Clone, Display)]
#[reflect(InspectorOptions)]
pub enum Texture {
    Blaadje1,
    Blaadje2,
    Blaadje3,
    Blaadje4,
    Blaadje5,
    Blaadje6,
    Blaadje7,
    Blaadje8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            displace: 1.0,
            size: 4.0,
            u_steps: 10,
            v_steps: 10,
            offset_size: 5.0,
            rotation_factor: 1.0,
            texture: Texture::Blaadje1,
            dark: Color::srgba_u8(9, 60, 33, 255),
            mid: Color::srgba_u8(71, 115, 50, 255),
            light: Color::srgba_u8(133, 170, 67, 255),
        }
    }
}

fn on_enter() {
    //
}

fn on_exit(mut vis: Visualizer) {
    vis.delete_all_tagged();
}

fn on_changed(mut vis: Visualizer, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    println!("RIPPLES on_changed called!");
    let Some((trimesh, ids)) = Sphere::new(Vec3::splat(settings.displace), settings.size)
        .into_uv_mesh(settings.u_steps, settings.v_steps)
        .and_then(|m| to_linear_with_quad_uvs(m))
    else {
        println!("invalid mesh");
        return;
    };

    let mat = BushMaterial {
        light: settings.light.to_linear(),
        mid: settings.mid.to_linear(),
        dark: settings.dark.to_linear(),
        rotation_factor: settings.rotation_factor,
        offset_size: settings.offset_size,
        texture: asset_server.load(format!(
            "textures/{}.png",
            settings.texture.to_string().to_lowercase()
        )),
    };

    vis.set(
        "ripples",
        VisMatMesh::new_unique(trimesh, mat).with_attr(QUAD_INDEX, ids),
    );
}

pub fn to_linear_with_quad_uvs(self) -> Option<(Self, Vec<u32>)> {
    let linear = self.to_linear();

    let mut uvs = Vec::new();
    let count = linear.verts.len();

    let mut ids = Vec::new();

    // if this is a quad mesh with 2 triangles back to back, the count should be a multiple of 6
    if count % 6 != 0 {
        return None;
    }

    for i in 0..(count / 6) {
        uvs.extend_from_slice(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
        ]);
        ids.extend_from_slice(&[i as u32; 6]);
    }

    return Some((linear.with_uvs(uvs), ids));
}

use bevy::{
    core::{Pod, Zeroable},
    prelude::*,
    reflect::{FromReflect, Reflect},
    render::render_resource::ShaderRef,
};
use wgpu::vertex_attr_array;

use crate::{
    prelude::*,
    render::{Flags, ShapeComponent, ShapeData, QUAD_BEZIER_HANDLE},
};

/// Component containing the data for drawing a line.
#[derive(Component, Reflect)]
pub struct QuadBezier {
    pub color: Color,
    pub thickness: f32,
    pub thickness_type: ThicknessType,
    pub alignment: Alignment,
    pub cap: Cap,

    /// Position to draw the start of the line in world space relative to it's transform.
    pub start: Vec3,
    /// Position to draw the end of the line in world space relative to it's transform.
    pub end: Vec3,
    /// Control point of the quadratic Bezier.
    pub control: Vec3,
}

impl QuadBezier {
    pub fn new(config: &ShapeConfig, start: Vec3, end: Vec3, control: Vec3) -> Self {
        Self {
            color: config.color,
            thickness: config.thickness,
            thickness_type: config.thickness_type,
            alignment: config.alignment,
            cap: config.cap,

            start,
            end,
            control,
        }
    }
}

impl Default for QuadBezier {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            thickness: 1.0,
            thickness_type: default(),
            alignment: default(),
            cap: default(),

            start: default(),
            end: default(),
            control: default(),
        }
    }
}

impl ShapeComponent for QuadBezier {
    type Data = QuadBezierData;

    fn into_data(&self, tf: &GlobalTransform) -> QuadBezierData {
        let mut flags = Flags(0);
        flags.set_thickness_type(self.thickness_type);
        flags.set_alignment(self.alignment);
        flags.set_cap(self.cap);

        QuadBezierData {
            transform: tf.compute_matrix().to_cols_array_2d(),

            color: self.color.as_rgba_f32(),
            thickness: self.thickness,
            flags: flags.0,

            start: self.start,
            end: self.end,
            control: self.control,
        }
    }
}

/// Raw data sent to the line shader to draw a line
#[derive(Clone, Copy, Reflect, FromReflect, Pod, Zeroable, Default, Debug)]
#[repr(C)]
pub struct QuadBezierData {
    transform: [[f32; 4]; 4],

    color: [f32; 4],
    thickness: f32,
    flags: u32,

    start: Vec3,
    end: Vec3,
    control: Vec3,
}

impl QuadBezierData {
    pub fn new(config: &ShapeConfig, start: Vec3, control: Vec3, end: Vec3) -> Self {
        let mut flags = Flags(0);
        flags.set_thickness_type(config.thickness_type);
        flags.set_alignment(config.alignment);
        flags.set_cap(config.cap);

        QuadBezierData {
            transform: config.transform.compute_matrix().to_cols_array_2d(),

            color: config.color.as_rgba_f32(),
            thickness: config.thickness,
            flags: flags.0,

            start,
            control,
            end,
        }
    }
}

impl ShapeData for QuadBezierData {
    type Component = QuadBezier;

    fn vertex_layout() -> Vec<wgpu::VertexAttribute> {
        vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4,
            2 => Float32x4,
            3 => Float32x4,

            4 => Float32x4,
            5 => Float32,
            6 => Uint32,
            7 => Float32x3,
            8 => Float32x3,
        ]
        .to_vec()
    }

    fn shader() -> ShaderRef {
        QUAD_BEZIER_HANDLE.typed::<Shader>().into()
    }

    fn transform(&self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.transform)
    }
}

/// Extension trait for [`ShapePainter`] to enable it to draw lines.
pub trait QuadBezierPainter {
    fn quad_bezier(&mut self, start: Vec3, control: Vec3, end: Vec3) -> &mut Self;
}

impl<'w, 's> QuadBezierPainter for ShapePainter<'w, 's> {
    fn quad_bezier(&mut self, start: Vec3, control: Vec3, end: Vec3) -> &mut Self {
        self.send(QuadBezierData::new(self.config(), start, control, end))
    }
}

/// Extension trait for [`ShapeBundle`] to enable creation of line bundles.
pub trait QuadBezierBundle {
    fn quad_bezier(config: &ShapeConfig, start: Vec3, control: Vec3, end: Vec3) -> Self;
}

impl QuadBezierBundle for ShapeBundle<QuadBezier> {
    fn quad_bezier(config: &ShapeConfig, start: Vec3, control: Vec3, end: Vec3) -> Self {
        Self::new(config, QuadBezier::new(config, start, control, end))
    }
}

/// Extension trait for [`ShapeSpawner`] to enable spawning of line entities.
pub trait QuadBezierSpawner<'w, 's>: ShapeSpawner<'w, 's> {
    fn quad_bezier(
        &mut self,
        start: Vec3,
        control: Vec3,
        end: Vec3,
    ) -> ShapeEntityCommands<'w, 's, '_>;
}

impl<'w, 's, T: ShapeSpawner<'w, 's>> QuadBezierSpawner<'w, 's> for T {
    fn quad_bezier(
        &mut self,
        start: Vec3,
        control: Vec3,
        end: Vec3,
    ) -> ShapeEntityCommands<'w, 's, '_> {
        self.spawn_shape(ShapeBundle::quad_bezier(self.config(), start, control, end))
    }
}

extern crate spirv_cross;
use spirv_cross::{msl, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn msl_compiler_options_has_default() {
    let compiler_options = msl::CompilerOptions::default();
    assert_eq!(compiler_options.vertex.invert_y, false);
    assert_eq!(compiler_options.vertex.transform_clip_space, false);
    assert!(compiler_options.resource_binding_overrides.is_empty());
    assert!(compiler_options.vertex_attribute_overrides.is_empty());
}

#[test]
fn ast_compiles_to_msl() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("shaders/simple.spv")));
    let mut ast = spirv::Ast::<msl::Target>::parse(&module).unwrap();

    let mut compiler_options = msl::CompilerOptions::default();

    compiler_options.resource_binding_overrides.insert(
        msl::ResourceBindingLocation {
            stage: spirv::ExecutionModel::Vertex,
            desc_set: 0,
            binding: 0,
        },
        msl::ResourceBinding {
            resource_id: 5,
            force_used: false,
        },
    );

    ast.set_compiler_options(&compiler_options).unwrap();
    assert_eq!(
        ast.compile().unwrap(),
        "\
#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct uniform_buffer_object
{
    float4x4 u_model_view_projection;
};

struct main0_in
{
    float3 a_normal [[attribute(1)]];
    float4 a_position [[attribute(0)]];
};

struct main0_out
{
    float3 v_normal [[user(locn0)]];
    float4 gl_Position [[position]];
};

vertex main0_out main0(main0_in in [[stage_in]], constant uniform_buffer_object& _22 [[buffer(5)]])
{
    main0_out out = {};
    out.v_normal = in.a_normal;
    out.gl_Position = _22.u_model_view_projection * in.a_position;
    return out;
}

"
    );
    assert_eq!(ast.get_cleansed_entry_point_name("main").unwrap(), "main0");
}

//! Raw compiler bindings for SPIRV-Cross.
use crate::{bindings as br, spirv::ImageType};
use crate::ptr_util::{read_from_ptr, read_into_vec_from_ptr, read_string_from_ptr};
use crate::spirv::{self, Decoration, Type};
use crate::ErrorCode;
use std::collections::HashSet;
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem::MaybeUninit, ptr};

impl spirv::ExecutionModel {
    fn from_raw(raw: br::spv::ExecutionModel) -> Result<Self, ErrorCode> {
        use crate::bindings::root::spv::ExecutionModel as Em;
        use crate::spirv::ExecutionModel::*;
        match raw {
            Em::ExecutionModelVertex => Ok(Vertex),
            Em::ExecutionModelTessellationControl => Ok(TessellationControl),
            Em::ExecutionModelTessellationEvaluation => Ok(TessellationEvaluation),
            Em::ExecutionModelGeometry => Ok(Geometry),
            Em::ExecutionModelFragment => Ok(Fragment),
            Em::ExecutionModelGLCompute => Ok(GlCompute),
            Em::ExecutionModelKernel => Ok(Kernel),
            _ => Err(ErrorCode::Unhandled),
        }
    }

    pub(crate) fn as_raw(self) -> br::spv::ExecutionModel {
        use crate::bindings::root::spv::ExecutionModel as Em;
        use crate::spirv::ExecutionModel::*;
        match self {
            Vertex => Em::ExecutionModelVertex,
            TessellationControl => Em::ExecutionModelTessellationControl,
            TessellationEvaluation => Em::ExecutionModelTessellationEvaluation,
            Geometry => Em::ExecutionModelGeometry,
            Fragment => Em::ExecutionModelFragment,
            GlCompute => Em::ExecutionModelGLCompute,
            Kernel => Em::ExecutionModelKernel,
        }
    }
}

impl spirv::Decoration {
    fn as_raw(self) -> br::spv::Decoration {
        use crate::bindings::root::spv::Decoration as D;
        match self {
            Decoration::RelaxedPrecision => D::DecorationRelaxedPrecision,
            Decoration::SpecId => D::DecorationSpecId,
            Decoration::Block => D::DecorationBlock,
            Decoration::BufferBlock => D::DecorationBufferBlock,
            Decoration::RowMajor => D::DecorationRowMajor,
            Decoration::ColMajor => D::DecorationColMajor,
            Decoration::ArrayStride => D::DecorationArrayStride,
            Decoration::MatrixStride => D::DecorationMatrixStride,
            Decoration::GlslShared => D::DecorationGLSLShared,
            Decoration::GlslPacked => D::DecorationGLSLPacked,
            Decoration::CPacked => D::DecorationCPacked,
            Decoration::BuiltIn => D::DecorationBuiltIn,
            Decoration::NoPerspective => D::DecorationNoPerspective,
            Decoration::Flat => D::DecorationFlat,
            Decoration::Patch => D::DecorationPatch,
            Decoration::Centroid => D::DecorationCentroid,
            Decoration::Sample => D::DecorationSample,
            Decoration::Invariant => D::DecorationInvariant,
            Decoration::Restrict => D::DecorationRestrict,
            Decoration::Aliased => D::DecorationAliased,
            Decoration::Volatile => D::DecorationVolatile,
            Decoration::Constant => D::DecorationConstant,
            Decoration::Coherent => D::DecorationCoherent,
            Decoration::NonWritable => D::DecorationNonWritable,
            Decoration::NonReadable => D::DecorationNonReadable,
            Decoration::Uniform => D::DecorationUniform,
            Decoration::SaturatedConversion => D::DecorationSaturatedConversion,
            Decoration::Stream => D::DecorationStream,
            Decoration::Location => D::DecorationLocation,
            Decoration::Component => D::DecorationComponent,
            Decoration::Index => D::DecorationIndex,
            Decoration::Binding => D::DecorationBinding,
            Decoration::DescriptorSet => D::DecorationDescriptorSet,
            Decoration::Offset => D::DecorationOffset,
            Decoration::XfbBuffer => D::DecorationXfbBuffer,
            Decoration::XfbStride => D::DecorationXfbStride,
            Decoration::FuncParamAttr => D::DecorationFuncParamAttr,
            Decoration::FpRoundingMode => D::DecorationFPRoundingMode,
            Decoration::FpFastMathMode => D::DecorationFPFastMathMode,
            Decoration::LinkageAttributes => D::DecorationLinkageAttributes,
            Decoration::NoContraction => D::DecorationNoContraction,
            Decoration::InputAttachmentIndex => D::DecorationInputAttachmentIndex,
            Decoration::Alignment => D::DecorationAlignment,
            Decoration::OverrideCoverageNv => D::DecorationOverrideCoverageNV,
            Decoration::PassthroughNv => D::DecorationPassthroughNV,
            Decoration::ViewportRelativeNv => D::DecorationViewportRelativeNV,
            Decoration::SecondaryViewportRelativeNv => D::DecorationSecondaryViewportRelativeNV,
        }
    }
}

impl spirv::Dim {
    fn from_raw(raw: br::spv::Dim) -> Result<Self, ErrorCode> {
        use crate::bindings::root::spv::Dim as D;
        use crate::spirv::Dim::*;
        match raw {
            D::Dim1D => Ok(Dim1D),
            D::Dim2D => Ok(Dim2D),
            D::Dim3D => Ok(Dim3D),
            D::DimCube => Ok(DimCube),
            D::DimRect => Ok(DimRect),
            D::DimBuffer => Ok(DimBuffer),
            D::DimSubpassData => Ok(DimSubpassData),
            _ => Err(ErrorCode::Unhandled),
        }
    }
}

impl spirv::ImageFormat {
    fn from_raw(raw: br::spv::ImageFormat) -> Result<Self, ErrorCode> {
        use crate::bindings::root::spv::ImageFormat as IF;
        use crate::spirv::ImageFormat::*;
        match raw {
            IF::ImageFormatUnknown => Ok(Unknown),
            IF::ImageFormatRgba32f => Ok(Rgba32f),
            IF::ImageFormatRgba16f => Ok(Rgba16f),
            IF::ImageFormatR32f => Ok(R32f),
            IF::ImageFormatRgba8 => Ok(Rgba8),
            IF::ImageFormatRgba8Snorm => Ok(Rgba8Snorm),
            IF::ImageFormatRg32f => Ok(Rg32f),
            IF::ImageFormatRg16f => Ok(Rg16f),
            IF::ImageFormatR11fG11fB10f => Ok(R11fG11fB10f),
            IF::ImageFormatR16f => Ok(R16f),
            IF::ImageFormatRgba16 => Ok(Rgba16),
            IF::ImageFormatRgb10A2 => Ok(Rgb10A2),
            IF::ImageFormatRg16 => Ok(Rg16),
            IF::ImageFormatRg8 => Ok(Rg8),
            IF::ImageFormatR16 => Ok(R16),
            IF::ImageFormatR8 => Ok(R8),
            IF::ImageFormatRgba16Snorm => Ok(Rgba16Snorm),
            IF::ImageFormatRg16Snorm => Ok(Rg16Snorm),
            IF::ImageFormatRg8Snorm => Ok(Rg8Snorm),
            IF::ImageFormatR16Snorm => Ok(R16Snorm),
            IF::ImageFormatR8Snorm => Ok(R8Snorm),
            IF::ImageFormatRgba32i => Ok(Rgba32i),
            IF::ImageFormatRgba16i => Ok(Rgba16i),
            IF::ImageFormatRgba8i => Ok(Rgba8i),
            IF::ImageFormatR32i => Ok(R32i),
            IF::ImageFormatRg32i => Ok(Rg32i),
            IF::ImageFormatRg16i => Ok(Rg16i),
            IF::ImageFormatRg8i => Ok(Rg8i),
            IF::ImageFormatR16i => Ok(R16i),
            IF::ImageFormatR8i => Ok(R8i),
            IF::ImageFormatRgba32ui => Ok(Rgba32ui),
            IF::ImageFormatRgba16ui => Ok(Rgba16ui),
            IF::ImageFormatRgba8ui => Ok(Rgba8ui),
            IF::ImageFormatR32ui => Ok(R32ui),
            IF::ImageFormatRgb10a2ui => Ok(Rgb10a2ui),
            IF::ImageFormatRg32ui => Ok(Rg32ui),
            IF::ImageFormatRg16ui => Ok(Rg16ui),
            IF::ImageFormatRg8ui => Ok(Rg8ui),
            IF::ImageFormatR16ui => Ok(R16ui),
            IF::ImageFormatR8ui => Ok(R8ui),
            IF::ImageFormatR64ui => Ok(R64ui),
            IF::ImageFormatR64i => Ok(R64i),
            _ => Err(ErrorCode::Unhandled),
        }
    }
}

impl spirv::ImageType {
    pub(crate) fn from_raw(ty: br::spirv_cross::SPIRType_ImageType) -> Result<ImageType, ErrorCode> {
        Ok(ImageType {
            type_id: ty.type_,
            dim: spirv::Dim::from_raw(ty.dim)?,
            depth: ty.depth,
            arrayed: ty.arrayed,
            ms: ty.ms,
            sampled: ty.sampled,
            format: spirv::ImageFormat::from_raw(ty.format)?,
        })
    }
}

impl spirv::Type {
    pub(crate) fn from_raw(
        ty: br::spirv_cross::SPIRType_BaseType,
        vecsize: u32,
        columns: u32,
        member_types: Vec<u32>,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
        image: br::spirv_cross::SPIRType_ImageType,
    ) -> Result<Self, ErrorCode> {
        use crate::bindings::root::spirv_cross::SPIRType_BaseType as B;
        use crate::spirv::Type::*;
        Ok(match ty {
            B::Unknown => Unknown,
            B::Void => Void,
            B::Boolean => Boolean {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Char => Char { array, array_size_literal, },
            B::Int => Int {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::UInt => UInt {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Int64 => Int64 { vecsize, array, array_size_literal, },
            B::UInt64 => UInt64 { vecsize, array, array_size_literal, },
            B::AtomicCounter => AtomicCounter { array, array_size_literal, },
            B::Half => Half {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Float => Float {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Double => Double {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Struct => Struct {
                member_types,
                array,
                array_size_literal,
            },
            B::Image => Image { array, array_size_literal, image: ImageType::from_raw(image)? },
            B::SampledImage => SampledImage { array, array_size_literal, image: ImageType::from_raw(image)? },
            B::Sampler => Sampler { array, array_size_literal },
            B::SByte => SByte { vecsize, array, array_size_literal },
            B::UByte => UByte { vecsize, array, array_size_literal },
            B::Short => Short { vecsize, array, array_size_literal },
            B::UShort => UShort { vecsize, array, array_size_literal },
            B::ControlPointArray => ControlPointArray,
            B::AccelerationStructure => AccelerationStructure,
            B::RayQuery => RayQuery,
            B::Interpolant => Interpolant,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Compiler<TTargetData> {
    pub(crate) sc_compiler: *mut br::ScInternalCompilerBase,
    pub(crate) target_data: TTargetData,
    pub(crate) has_been_compiled: bool,
}

impl<TTargetData> Compiler<TTargetData> {
    #[cfg(any(feature = "glsl", feature = "hlsl"))]
    pub fn compile(&mut self) -> Result<String, ErrorCode> {
        unsafe {
            let mut shader_ptr = ptr::null();
            check!(br::sc_internal_compiler_compile(
                self.sc_compiler,
                &mut shader_ptr,
            ));
            let shader = read_string_from_ptr(shader_ptr)?;
            check!(br::sc_internal_free_pointer(shader_ptr as *mut c_void));
            Ok(shader)
        }
    }

    pub fn get_decoration(&self, id: u32, decoration: spirv::Decoration) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_decoration(
                self.sc_compiler,
                &mut result,
                id,
                decoration.as_raw(),
            ));
        }
        Ok(result)
    }

    pub fn get_name(&mut self, id: u32) -> Result<String, ErrorCode> {
        unsafe {
            let mut name_ptr = ptr::null();
            check!(br::sc_internal_compiler_get_name(
                self.sc_compiler,
                id,
                &mut name_ptr,
            ));
            let name = read_string_from_ptr(name_ptr)?;
            check!(br::sc_internal_free_pointer(name_ptr as *mut c_void));
            Ok(name)
        }
    }

    pub fn set_name(&mut self, id: u32, name: &str) -> Result<(), ErrorCode> {
        let name = CString::new(name);
        unsafe {
            match name {
                Ok(name) => {
                    check!(br::sc_internal_compiler_set_name(
                        self.sc_compiler,
                        id,
                        name.as_ptr(),
                    ));
                }
                _ => return Err(ErrorCode::Unhandled),
            }
        }
        Ok(())
    }

    pub fn set_member_name(&mut self, id: u32, index: u32, name: &str) -> Result<(), ErrorCode> {
        let name = CString::new(name);
        unsafe {
            match name {
                Ok(name) => {
                    check!(br::sc_internal_compiler_set_member_name(
                        self.sc_compiler,
                        id,
                        index,
                        name.as_ptr(),
                    ));
                }
                _ => return Err(ErrorCode::Unhandled),
            }
        }
        Ok(())
    }

    pub fn unset_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_unset_decoration(
                self.sc_compiler,
                id,
                decoration.as_raw(),
            ));
        }

        Ok(())
    }

    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_set_decoration(
                self.sc_compiler,
                id,
                decoration.as_raw(),
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_entry_points(&self) -> Result<Vec<spirv::EntryPoint>, ErrorCode> {
        let mut entry_points_raw = ptr::null_mut();
        let mut entry_points_raw_length = 0 as usize;

        unsafe {
            check!(br::sc_internal_compiler_get_entry_points(
                self.sc_compiler,
                &mut entry_points_raw,
                &mut entry_points_raw_length,
            ));

            let entry_points = (0..entry_points_raw_length)
                .map(|offset| {
                    let entry_point_raw_ptr = entry_points_raw.add(offset);
                    let entry_point_raw = read_from_ptr::<br::ScEntryPoint>(entry_point_raw_ptr);
                    let name = read_string_from_ptr(entry_point_raw.name)?;
                    let entry_point = spirv::EntryPoint {
                        name,
                        execution_model: spirv::ExecutionModel::from_raw(
                            entry_point_raw.execution_model,
                        )?,
                        work_group_size: spirv::WorkGroupSize {
                            x: entry_point_raw.work_group_size_x,
                            y: entry_point_raw.work_group_size_y,
                            z: entry_point_raw.work_group_size_z,
                        },
                    };

                    check!(br::sc_internal_free_pointer(
                        entry_point_raw.name as *mut c_void,
                    ));

                    Ok(entry_point)
                })
                .collect::<Result<Vec<_>, _>>();

            check!(br::sc_internal_free_pointer(
                entry_points_raw as *mut c_void,
            ));

            Ok(entry_points?)
        }
    }

    pub fn get_active_buffer_ranges(&self, id: u32) -> Result<Vec<spirv::BufferRange>, ErrorCode> {
        let mut active_buffer_ranges_raw = ptr::null_mut();
        let mut active_buffer_ranges_raw_length = 0 as usize;

        unsafe {
            check!(br::sc_internal_compiler_get_active_buffer_ranges(
                self.sc_compiler,
                id,
                &mut active_buffer_ranges_raw,
                &mut active_buffer_ranges_raw_length,
            ));

            let active_buffer_ranges = (0..active_buffer_ranges_raw_length)
                .map(|offset| {
                    let active_buffer_range_raw_ptr = active_buffer_ranges_raw.add(offset);
                    let active_buffer_range_raw =
                        read_from_ptr::<br::ScBufferRange>(active_buffer_range_raw_ptr);
                    spirv::BufferRange {
                        index: active_buffer_range_raw.index,
                        offset: active_buffer_range_raw.offset,
                        range: active_buffer_range_raw.range,
                    }
                })
                .collect::<Vec<_>>();

            check!(br::sc_internal_free_pointer(
                active_buffer_ranges_raw as *mut c_void
            ));

            Ok(active_buffer_ranges)
        }
    }

    pub fn get_cleansed_entry_point_name(
        &self,
        entry_point_name: &str,
        execution_model: spirv::ExecutionModel,
    ) -> Result<String, ErrorCode> {
        let mut cleansed_ptr = ptr::null();
        let entry_point = CString::new(entry_point_name);
        match entry_point {
            Ok(ep) => unsafe {
                check!(br::sc_internal_compiler_get_cleansed_entry_point_name(
                    self.sc_compiler,
                    ep.as_ptr(),
                    execution_model.as_raw(),
                    &mut cleansed_ptr
                ));
                let cleansed = read_string_from_ptr(cleansed_ptr)?;
                check!(br::sc_internal_free_pointer(cleansed_ptr as *mut c_void));
                Ok(cleansed)
            },
            _ => Err(ErrorCode::Unhandled),
        }
    }

    pub fn get_specialization_constants(
        &self,
    ) -> Result<Vec<spirv::SpecializationConstant>, ErrorCode> {
        let mut constants_raw = ptr::null_mut();
        let mut constants_raw_length = 0 as usize;

        unsafe {
            check!(br::sc_internal_compiler_get_specialization_constants(
                self.sc_compiler,
                &mut constants_raw,
                &mut constants_raw_length,
            ));

            let constants = (0..constants_raw_length)
                .map(|offset| {
                    let constant_raw_ptr = constants_raw.add(offset);
                    let constant_raw =
                        read_from_ptr::<br::ScSpecializationConstant>(constant_raw_ptr);

                    let constant = spirv::SpecializationConstant {
                        id: constant_raw.id,
                        constant_id: constant_raw.constant_id,
                    };

                    Ok(constant)
                })
                .collect::<Result<Vec<_>, _>>();

            check!(br::sc_internal_free_pointer(constants_raw as *mut c_void));

            Ok(constants?)
        }
    }

    pub fn set_scalar_constant(&self, id: u32, value: u64) -> Result<(), ErrorCode> {
        let high_bits = (value >> 32) as u32;
        let low_bits = value as u32;
        unsafe {
            check!(br::sc_internal_compiler_set_scalar_constant(
                self.sc_compiler,
                id,
                high_bits,
                low_bits,
            ));
        }

        Ok(())
    }

    pub fn get_type(&self, id: u32) -> Result<spirv::Type, ErrorCode> {
        unsafe {
            let mut type_ptr = std::mem::zeroed();

            check!(br::sc_internal_compiler_get_type(
                self.sc_compiler,
                id,
                &mut type_ptr,
            ));

            let raw = read_from_ptr::<br::ScType>(type_ptr);
            let member_types = read_into_vec_from_ptr(raw.member_types, raw.member_types_size);
            let array = read_into_vec_from_ptr(raw.array, raw.array_size);
            let array_size_literal = read_into_vec_from_ptr(raw.array_size_literal, raw.array_size);
            let image = raw.image;
            let result = Type::from_raw(raw.type_, raw.vecsize, raw.columns, member_types, array, array_size_literal, image)?;

            if raw.member_types_size > 0 {
                check!(br::sc_internal_free_pointer(
                    raw.member_types as *mut c_void
                ));
            }
            if raw.array_size > 0 {
                check!(br::sc_internal_free_pointer(raw.array as *mut c_void));
            }
            check!(br::sc_internal_free_pointer(type_ptr as *mut c_void));

            Ok(result)
        }
    }

    pub fn get_member_name(&self, id: u32, index: u32) -> Result<String, ErrorCode> {
        unsafe {
            let mut name_ptr = ptr::null();
            check!(br::sc_internal_compiler_get_member_name(
                self.sc_compiler,
                id,
                index,
                &mut name_ptr,
            ));
            let name = read_string_from_ptr(name_ptr)?;
            check!(br::sc_internal_free_pointer(name_ptr as *mut c_void));
            Ok(name)
        }
    }

    pub fn get_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
    ) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration.as_raw(),
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn set_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_set_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration.as_raw(),
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_declared_struct_size(&self, id: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_declared_struct_size(
                self.sc_compiler,
                id,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_declared_struct_member_size(&self, id: u32, index: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_declared_struct_member_size(
                self.sc_compiler,
                id,
                index,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_shader_resources(&self) -> Result<spirv::ShaderResources, ErrorCode> {
        unsafe {
            let mut shader_resources_raw = MaybeUninit::uninit();
            check!(br::sc_internal_compiler_get_shader_resources(
                self.sc_compiler,
                shader_resources_raw.as_mut_ptr(),
            ));
            let shader_resources_raw = shader_resources_raw.assume_init();

            let fill_resources = |array_raw: &br::ScResourceArray| {
                let resources = (0..array_raw.num as usize)
                    .map(|i| {
                        let resource_raw = read_from_ptr::<br::ScResource>(array_raw.data.add(i));
                        let name = read_string_from_ptr(resource_raw.name)?;
                        check!(br::sc_internal_free_pointer(
                            resource_raw.name as *mut c_void,
                        ));

                        Ok(spirv::Resource {
                            id: resource_raw.id,
                            type_id: resource_raw.type_id,
                            base_type_id: resource_raw.base_type_id,
                            name,
                        })
                    })
                    .collect::<Result<Vec<_>, ErrorCode>>();

                check!(br::sc_internal_free_pointer(array_raw.data as *mut c_void));

                resources
            };

            let uniform_buffers = fill_resources(&shader_resources_raw.uniform_buffers)?;
            let storage_buffers = fill_resources(&shader_resources_raw.storage_buffers)?;
            let stage_inputs = fill_resources(&shader_resources_raw.stage_inputs)?;
            let stage_outputs = fill_resources(&shader_resources_raw.stage_outputs)?;
            let subpass_inputs = fill_resources(&shader_resources_raw.subpass_inputs)?;
            let storage_images = fill_resources(&shader_resources_raw.storage_images)?;
            let sampled_images = fill_resources(&shader_resources_raw.sampled_images)?;
            let atomic_counters = fill_resources(&shader_resources_raw.atomic_counters)?;
            let acceleration_structures = fill_resources(&shader_resources_raw.acceleration_structures)?;
            let gl_plain_uniforms = fill_resources(&shader_resources_raw.gl_plain_uniforms)?;
            let push_constant_buffers =
                fill_resources(&shader_resources_raw.push_constant_buffers)?;
            let shader_record_buffers = fill_resources(&shader_resources_raw.shader_record_buffers)?;
            let separate_images = fill_resources(&shader_resources_raw.separate_images)?;
            let separate_samplers = fill_resources(&shader_resources_raw.separate_samplers)?;

            Ok(spirv::ShaderResources {
                uniform_buffers,
                storage_buffers,
                stage_inputs,
                stage_outputs,
                subpass_inputs,
                storage_images,
                sampled_images,
                atomic_counters,
                push_constant_buffers,
                separate_images,
                separate_samplers,
                acceleration_structures,
                gl_plain_uniforms,
                shader_record_buffers,
            })
        }
    }

    pub fn get_active_interface_variables(&self) -> Result<HashSet<u32>, ErrorCode> {
        unsafe {
            let mut ids: *mut u32 = ptr::null_mut();
            let mut size: usize = 0;
            check!(br::sc_internal_compiler_get_active_interface_variables(
                self.sc_compiler,
                &mut ids,
                &mut size
            ));
            let result: HashSet<u32> = std::slice::from_raw_parts(ids, size)
                .iter()
                .cloned()
                .collect();
            check!(br::sc_internal_free_pointer(ids as *mut c_void));
            Ok(result)
        }
    }

    pub fn rename_interface_variable(
        &self,
        resources: &[spirv::Resource],
        location: u32,
        new_name: &str,
    ) -> Result<(), ErrorCode> {
        unsafe {
            let mut resources_names = Vec::new();
            for resource in resources.iter() {
                match CString::new(&*resource.name) {
                    Ok(rn) => resources_names.push(rn),
                    Err(_) => return Err(ErrorCode::Unhandled),
                }
            }

            let new_name = CString::new(new_name).map_err(|_| ErrorCode::Unhandled)?;
            let new_name_ptr = new_name.as_ptr();
            let resources = resources
                .iter()
                .enumerate()
                .map(|(i, r)| br::ScResource {
                    id: r.id,
                    type_id: r.type_id,
                    base_type_id: r.base_type_id,
                    name: resources_names[i].as_ptr() as _,
                })
                .collect::<Vec<_>>();
            let resources_ptr = resources.as_ptr();

            check!(br::sc_internal_compiler_rename_interface_variable(
                self.sc_compiler,
                resources_ptr,
                resources_names.len(),
                location,
                new_name_ptr,
            ));

            Ok(())
        }
    }

    pub fn get_work_group_size_specialization_constants(
        &self,
    ) -> Result<spirv::WorkGroupSizeSpecializationConstants, ErrorCode> {
        let mut constants_raw = ptr::null_mut();

        unsafe {
            check!(
                br::sc_internal_compiler_get_work_group_size_specialization_constants(
                    self.sc_compiler,
                    &mut constants_raw,
                )
            );

            let x = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(0));
            let y = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(1));
            let z = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(2));

            let constants = spirv::WorkGroupSizeSpecializationConstants {
                x: spirv::SpecializationConstant {
                    id: x.id,
                    constant_id: x.constant_id,
                },
                y: spirv::SpecializationConstant {
                    id: y.id,
                    constant_id: y.constant_id,
                },
                z: spirv::SpecializationConstant {
                    id: z.id,
                    constant_id: z.constant_id,
                },
            };

            check!(br::sc_internal_free_pointer(constants_raw as *mut c_void));

            Ok(constants)
        }
    }
}

impl<TTargetData> Drop for Compiler<TTargetData> {
    fn drop(&mut self) {
        unsafe {
            br::sc_internal_compiler_delete(self.sc_compiler);
        }
    }
}

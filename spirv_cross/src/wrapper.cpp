#pragma GCC diagnostic ignored "-Wunknown-pragmas"
#pragma warning(disable : 4996 4101)

#include "vendor/SPIRV-Cross/spirv_cross_util.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"
#include "vendor/SPIRV-Cross/spirv_msl.hpp"
#include "vendor/SPIRV-Cross/spirv_glsl.hpp"
#include "wrapper.hpp"

static const char *latest_exception_message;

#ifdef SPIRV_CROSS_WRAPPER_NO_EXCEPTIONS
#define INTERNAL_RESULT(block_to_attempt)                 \
    do                                                    \
    {                                                     \
        {                                                 \
            block_to_attempt                              \
        }                                                 \
        return ScInternalResult::Success;                 \
    } while (0);
#else
#define INTERNAL_RESULT(block_to_attempt)                 \
    do                                                    \
    {                                                     \
        try                                               \
        {                                                 \
            {                                             \
                block_to_attempt                          \
            }                                             \
            return ScInternalResult::Success;             \
        }                                                 \
        catch (const spirv_cross::CompilerError &ex)      \
        {                                                 \
            latest_exception_message = strdup(ex.what()); \
            return ScInternalResult::CompilationError;    \
        }                                                 \
        catch (const std::exception &ex)                  \
        {                                                 \
            return ScInternalResult::Unhandled;           \
        }                                                 \
        catch (...)                                       \
        {                                                 \
            return ScInternalResult::Unhandled;           \
        }                                                 \
        return ScInternalResult::Unhandled;               \
    } while (0);
#endif

extern "C"
{
    ScInternalResult sc_internal_get_latest_exception_message(const char **message)
    {
        INTERNAL_RESULT(*message = latest_exception_message;)
    }

#ifdef SPIRV_CROSS_WRAPPER_HLSL
    ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, const size_t size)
    {
        INTERNAL_RESULT(*compiler = new spirv_cross::CompilerHLSL(ir, size);)
    }

    ScInternalResult sc_internal_compiler_hlsl_set_options(const ScInternalCompilerHlsl *compiler, const ScHlslCompilerOptions *options)
    {
        INTERNAL_RESULT(
            do {
                auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
                auto glsl_options = compiler_glsl->get_common_options();
                glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
                glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
                glsl_options.force_zero_initialized_variables = options->force_zero_initialized_variables;
                compiler_glsl->set_common_options(glsl_options);

                auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
                auto hlsl_options = compiler_hlsl->get_hlsl_options();
                hlsl_options.shader_model = options->shader_model;
                hlsl_options.point_size_compat = options->point_size_compat;
                hlsl_options.point_coord_compat = options->point_coord_compat;
                hlsl_options.force_storage_buffer_as_uav = options->force_storage_buffer_as_uav;
                hlsl_options.nonwritable_uav_texture_as_srv = options->nonwritable_uav_texture_as_srv;
                hlsl_options.flatten_matrix_vertex_input_semantics = options->flatten_matrix_vertex_input_semantics;

                compiler_hlsl->set_hlsl_options(hlsl_options);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_hlsl_set_root_constant_layout(const ScInternalCompilerHlsl *compiler, const ScHlslRootConstant *constants, size_t count)
    {
        INTERNAL_RESULT(
            do {
                std::vector<spirv_cross::RootConstants> root_constants;
                for (size_t i = 0; i < count; i++)
                {
                    root_constants.push_back(
                        spirv_cross::RootConstants{
                            constants[i].start,
                            constants[i].end,
                            constants[i].binding,
                            constants[i].space});
                }

                auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
                compiler_hlsl->set_root_constant_layouts(root_constants);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_hlsl_add_resource_binding(const ScInternalCompilerHlsl *compiler, ScHlslResourceBinding resource_binding)
    {
        INTERNAL_RESULT(
            do {
                spirv_cross::HLSLResourceBinding rb;
                rb.stage = resource_binding.stage;
                rb.desc_set = resource_binding.desc_set;
                rb.binding = resource_binding.binding;
                rb.cbv.register_space = resource_binding.cbv.register_space;
                rb.cbv.register_binding = resource_binding.cbv.register_binding;
                rb.uav.register_space = resource_binding.uav.register_space;
                rb.uav.register_binding = resource_binding.uav.register_binding;
                rb.srv.register_space = resource_binding.srv.register_space;
                rb.srv.register_binding = resource_binding.srv.register_binding;
                rb.sampler.register_space = resource_binding.sampler.register_space;
                rb.sampler.register_binding = resource_binding.sampler.register_binding;

                auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
                compiler_hlsl->add_hlsl_resource_binding(rb);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_hlsl_add_vertex_attribute_remap(const ScInternalCompilerHlsl *compiler, ScHlslVertexAttributeRemap remap)
    {
        INTERNAL_RESULT(
            do {
                spirv_cross::HLSLVertexAttributeRemap r;
                r.location = remap.location;
                r.semantic = std::string(remap.semantic);

                auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
                compiler_hlsl->add_vertex_attribute_remap(r);
            } while (0);)
    }
#endif

#ifdef SPIRV_CROSS_WRAPPER_MSL
    ScInternalResult sc_internal_compiler_msl_new(ScInternalCompilerMsl **compiler, const uint32_t *ir, const size_t size)
    {
        INTERNAL_RESULT(*compiler = new spirv_cross::CompilerMSL(ir, size);)
    }

    ScInternalResult sc_internal_compiler_msl_compile(const ScInternalCompilerBase *compiler, const char **shader,
                                                      const spirv_cross::MSLShaderInterfaceVariable  *p_vat_overrides, const size_t vat_override_count,
                                                      const spirv_cross::MSLResourceBinding *p_res_overrides, const size_t res_override_count,
                                                      const ScMslConstSamplerMapping *p_const_samplers, const size_t const_sampler_count)
    {
        INTERNAL_RESULT(
            do {
                auto compiler_msl = ((spirv_cross::CompilerMSL *)compiler);

                for (size_t i = 0; i < vat_override_count; i++)
                {
                    compiler_msl->add_msl_shader_input(p_vat_overrides[i]);
                }

                for (size_t i = 0; i < res_override_count; i++)
                {
                    compiler_msl->add_msl_resource_binding(p_res_overrides[i]);
                }

                for (size_t i = 0; i < const_sampler_count; i++)
                {
                    const auto& mapping = p_const_samplers[i];
                    compiler_msl->remap_constexpr_sampler_by_binding(mapping.desc_set, mapping.binding, mapping.sampler);
                }

                *shader = strdup(compiler_msl->compile().c_str());
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_msl_set_options(const ScInternalCompilerMsl *compiler, const ScMslCompilerOptions *options)
    {
        INTERNAL_RESULT(
            do {
                auto compiler_msl = (spirv_cross::CompilerMSL *)compiler;

                auto glsl_options = compiler_msl->get_common_options();
                glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
                glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
                glsl_options.force_zero_initialized_variables = options->force_zero_initialized_variables;
                compiler_msl->set_common_options(glsl_options);

                auto msl_options = compiler_msl->get_msl_options();
                msl_options.platform = static_cast<spirv_cross::CompilerMSL::Options::Platform>(options->platform);
                msl_options.msl_version = options->version;
                msl_options.swizzle_buffer_index = options->swizzle_buffer_index;
                msl_options.indirect_params_buffer_index = options->indirect_params_buffer_index;
                msl_options.shader_output_buffer_index = options->shader_output_buffer_index;
                msl_options.shader_patch_output_buffer_index = options->shader_patch_output_buffer_index;
                msl_options.shader_tess_factor_buffer_index = options->shader_tess_factor_buffer_index;
                msl_options.buffer_size_buffer_index = options->buffer_size_buffer_index;
                msl_options.enable_point_size_builtin = options->enable_point_size_builtin;
                msl_options.disable_rasterization = options->disable_rasterization;
                msl_options.capture_output_to_buffer = options->capture_output_to_buffer;
                msl_options.swizzle_texture_samples = options->swizzle_texture_samples;
                msl_options.tess_domain_origin_lower_left = options->tess_domain_origin_lower_left;
                msl_options.argument_buffers = options->argument_buffers;
                msl_options.pad_fragment_output_components = options->pad_fragment_output_components;
                msl_options.force_native_arrays = options->force_native_arrays;
                msl_options.force_active_argument_buffer_resources = options->force_active_argument_buffer_resources;
                msl_options.pad_argument_buffer_resources = options->pad_argument_buffer_resources;
                compiler_msl->set_msl_options(msl_options);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_msl_get_is_rasterization_disabled(const ScInternalCompilerMsl *compiler, bool *is_rasterization_disabled)
    {
        INTERNAL_RESULT(*is_rasterization_disabled = ((spirv_cross::CompilerMSL *)compiler)->get_is_rasterization_disabled();)
    }
#endif

#ifdef SPIRV_CROSS_WRAPPER_GLSL
    ScInternalResult sc_internal_compiler_glsl_new(ScInternalCompilerGlsl **compiler, const uint32_t *ir, const size_t size)
    {
        INTERNAL_RESULT(*compiler = new spirv_cross::CompilerGLSL(ir, size);)
    }

    ScInternalResult sc_internal_compiler_glsl_set_options(const ScInternalCompilerGlsl *compiler, const ScGlslCompilerOptions *options)
    {
        INTERNAL_RESULT(
            do {
                auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
                auto glsl_options = compiler_glsl->get_common_options();
                glsl_options.version = options->version;
                glsl_options.es = options->es;
                glsl_options.force_temporary = options->force_temporary;
                glsl_options.vulkan_semantics = options->vulkan_semantics;
                glsl_options.separate_shader_objects = options->separate_shader_objects;
                glsl_options.flatten_multidimensional_arrays = options->flatten_multidimensional_arrays;
                glsl_options.enable_420pack_extension = options->enable_420_pack_extension;
                glsl_options.emit_push_constant_as_uniform_buffer = options->emit_push_constant_as_uniform_buffer;
                glsl_options.emit_uniform_buffer_as_plain_uniforms = options->emit_uniform_buffer_as_plain_uniforms;
                glsl_options.emit_line_directives = options->emit_line_directives;
                glsl_options.enable_storage_image_qualifier_deduction = options->enable_storage_image_qualifier_deduction;
                glsl_options.force_zero_initialized_variables = options->force_zero_initialized_variables;
                glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
                glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
                glsl_options.vertex.support_nonzero_base_instance = options->vertex_support_nonzero_base_instance;
                glsl_options.fragment.default_float_precision = static_cast<spirv_cross::CompilerGLSL::Options::Precision>(options->fragment_default_float_precision);
                glsl_options.fragment.default_int_precision = static_cast<spirv_cross::CompilerGLSL::Options::Precision>(options->fragment_default_int_precision);
                compiler_glsl->set_common_options(glsl_options);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_glsl_build_combined_image_samplers(const ScInternalCompilerBase *compiler)
    {
        INTERNAL_RESULT(
            do {
                ((spirv_cross::CompilerGLSL *)compiler)->build_combined_image_samplers();
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_glsl_get_combined_image_samplers(const ScInternalCompilerBase *compiler, const ScCombinedImageSampler **samplers, size_t *size)
    {
        INTERNAL_RESULT(
            do {
                const spirv_cross::SmallVector<spirv_cross::CombinedImageSampler>& ret = ((spirv_cross::CompilerGLSL *)compiler)->get_combined_image_samplers();
                *samplers = (const ScCombinedImageSampler *)ret.data();
                *size = ret.size();
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_glsl_add_header_line(const ScInternalCompilerBase *compiler, const char *str)
    {
        INTERNAL_RESULT(
            do {
                ((spirv_cross::CompilerGLSL *)compiler)->add_header_line(std::string(str));
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_glsl_flatten_buffer_block(const ScInternalCompilerBase *compiler, const uint32_t id)
    {
        INTERNAL_RESULT(
            do {
                ((spirv_cross::CompilerGLSL *)compiler)->flatten_buffer_block(id);
            } while (0);)
    }
#endif

    ScInternalResult sc_internal_compiler_get_decoration(const ScInternalCompilerBase *compiler, uint32_t *result, const uint32_t id, const spv::Decoration decoration)
    {
        INTERNAL_RESULT(*result = ((spirv_cross::Compiler *)compiler)->get_decoration(id, decoration);)
    }

    ScInternalResult sc_internal_compiler_unset_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const spv::Decoration decoration)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->unset_decoration(id, decoration);)
    }

    ScInternalResult sc_internal_compiler_set_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const spv::Decoration decoration, const uint32_t argument)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_decoration(id, decoration, argument);)
    }

    ScInternalResult sc_internal_compiler_get_name(const ScInternalCompilerBase *compiler, const uint32_t id, const char **name)
    {
        INTERNAL_RESULT(
            do {
                auto const _name = ((spirv_cross::Compiler *)compiler)->get_name(id);
                *name = strdup(_name.c_str());
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_set_name(const ScInternalCompilerBase *compiler, const uint32_t id, const char *name)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_name(id, std::string(name));)
    }

    ScInternalResult sc_internal_compiler_set_member_name(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const char *name)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_member_name(id, index, std::string(name));)
    }

    ScInternalResult sc_internal_compiler_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size)
    {
        INTERNAL_RESULT(
            do {
                auto const &comp = *((spirv_cross::Compiler *)compiler);
                auto const &sc_entry_point_names_and_stages = comp.get_entry_points_and_stages();
                auto const sc_size = sc_entry_point_names_and_stages.size();

                *entry_points = (ScEntryPoint *)malloc(sc_size * sizeof(ScEntryPoint));
                *size = sc_size;
                for (uint32_t i = 0; i < sc_size; i++)
                {
                    auto const &sc_entry_point = sc_entry_point_names_and_stages[i];
                    auto const &sc_spir_entry_point = comp.get_entry_point(sc_entry_point.name, sc_entry_point.execution_model);
                    auto &entry_point = (*entry_points)[i];
                    entry_point.name = strdup(sc_entry_point.name.c_str());
                    entry_point.execution_model = sc_spir_entry_point.model;
                    entry_point.work_group_size_x = sc_spir_entry_point.workgroup_size.x;
                    entry_point.work_group_size_y = sc_spir_entry_point.workgroup_size.y;
                    entry_point.work_group_size_z = sc_spir_entry_point.workgroup_size.z;
                }
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_active_buffer_ranges(const ScInternalCompilerBase *compiler, uint32_t id, ScBufferRange **active_buffer_ranges, size_t *size)
    {
        INTERNAL_RESULT(
            do {
                auto const &comp = *((spirv_cross::Compiler *)compiler);
                auto const &sc_active_buffer_ranges = comp.get_active_buffer_ranges(id);
                auto const sc_size = sc_active_buffer_ranges.size();

                *active_buffer_ranges = (ScBufferRange *)malloc(sc_size * sizeof(ScBufferRange));
                *size = sc_size;
                for (uint32_t i = 0; i < sc_size; i++)
                {
                    auto const &sc_active_buffer_range = sc_active_buffer_ranges[i];
                    auto &active_buffer_range = (*active_buffer_ranges)[i];
                    active_buffer_range.index = sc_active_buffer_range.index;
                    active_buffer_range.offset = sc_active_buffer_range.offset;
                    active_buffer_range.range = sc_active_buffer_range.range;
                }
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_cleansed_entry_point_name(const ScInternalCompilerBase *compiler, const char *original_entry_point_name, const spv::ExecutionModel execution_model, const char **compiled_entry_point_name)
    {
        INTERNAL_RESULT(
            do {
                *compiled_entry_point_name = strdup(
                    (*((spirv_cross::Compiler *)compiler))
                        .get_cleansed_entry_point_name(std::string(original_entry_point_name), execution_model)
                        .c_str());
            } while (0);)
    }

    void fill_resource_array(ScResourceArray *resources, const spirv_cross::SmallVector<spirv_cross::Resource> &sc_resources)
    {
        auto const sc_size = sc_resources.size();

        if (sc_size == 0)
        {
            resources->num = 0;
            resources->data = 0x0;
            return;
        }

        resources->num = sc_size;
        resources->data = (ScResource *)malloc(sc_size * sizeof(ScResource));
        for (uint32_t i = 0; i < sc_size; i++)
        {
            auto const &resource = sc_resources[i];
            resources->data[i].id = resource.id;
            resources->data[i].type_id = resource.type_id;
            resources->data[i].base_type_id = resource.base_type_id;
            resources->data[i].name = strdup(resource.name.c_str());
        }
    }

    ScInternalResult sc_internal_compiler_get_shader_resources(const ScInternalCompilerBase *compiler, ScShaderResources *shader_resources)
    {
        INTERNAL_RESULT(
            do {
                auto const sc_resources = ((const spirv_cross::Compiler *)compiler)->get_shader_resources();

                fill_resource_array(&shader_resources->uniform_buffers, sc_resources.uniform_buffers);
                fill_resource_array(&shader_resources->storage_buffers, sc_resources.storage_buffers);
                fill_resource_array(&shader_resources->stage_inputs, sc_resources.stage_inputs);
                fill_resource_array(&shader_resources->stage_outputs, sc_resources.stage_outputs);
                fill_resource_array(&shader_resources->subpass_inputs, sc_resources.subpass_inputs);
                fill_resource_array(&shader_resources->storage_images, sc_resources.storage_images);
                fill_resource_array(&shader_resources->sampled_images, sc_resources.sampled_images);
                fill_resource_array(&shader_resources->atomic_counters, sc_resources.atomic_counters);
                fill_resource_array(&shader_resources->acceleration_structures, sc_resources.acceleration_structures);
                fill_resource_array(&shader_resources->gl_plain_uniforms, sc_resources.gl_plain_uniforms);
                fill_resource_array(&shader_resources->push_constant_buffers, sc_resources.push_constant_buffers);
                fill_resource_array(&shader_resources->separate_images, sc_resources.separate_images);
                fill_resource_array(&shader_resources->separate_samplers, sc_resources.separate_samplers);
                fill_resource_array(&shader_resources->shader_record_buffers, sc_resources.shader_record_buffers);
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_specialization_constants(const ScInternalCompilerBase *compiler, ScSpecializationConstant **constants, size_t *size)
    {
        INTERNAL_RESULT(
            do {
                auto const sc_constants = ((const spirv_cross::Compiler *)compiler)->get_specialization_constants();
                auto const sc_size = sc_constants.size();

                auto p_constants = (ScSpecializationConstant *)malloc(sc_size * sizeof(ScSpecializationConstant));
                *constants = p_constants;
                *size = sc_size;
                for (uint32_t i = 0; i < sc_size; i++)
                {
                    auto const &sc_constant = sc_constants[i];
                    p_constants[i].id = sc_constant.id;
                    p_constants[i].constant_id = sc_constant.constant_id;
                }
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_set_scalar_constant(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t constant_high_bits, const uint32_t constant_low_bits)
    {
        INTERNAL_RESULT(
            do {
                auto &sc_constant = ((spirv_cross::Compiler *)compiler)->get_constant(id);
                sc_constant.m.c[0].r[0].u64 = (((uint64_t)constant_high_bits) << 32) | constant_low_bits;
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_type(const ScInternalCompilerBase *compiler, const uint32_t id, const ScType **spirv_type)
    {
        INTERNAL_RESULT(
            do {
                auto const &type = ((spirv_cross::Compiler *)compiler)->get_type(id);
                auto const member_types_size = type.member_types.size();
                auto const array_size = type.array.size();

                auto ty = (ScType *)malloc(sizeof(ScType));
                ty->type = type.basetype;
                ty->width = type.width;
                ty->vecsize = type.vecsize;
                ty->columns = type.columns;
                ty->member_types_size = member_types_size;
                ty->array_size = array_size;
                ty->storage = type.storage;

                if (member_types_size > 0)
                {
                    auto const &member_types = (uint32_t *)malloc(member_types_size * sizeof(uint32_t));

                    for (size_t i = 0; i < member_types_size; i++)
                    {
                        member_types[i] = type.member_types[i];
                    }

                    ty->member_types = member_types;
                }

                if (array_size > 0)
                {
                    auto const &array = (uint32_t *)malloc(array_size * sizeof(uint32_t));
                    auto const &array_size_literal = (bool *)malloc(array_size * sizeof(bool));

                    for (size_t i = 0; i < array_size; i++)
                    {
                        array[i] = type.array[i];
                        array_size_literal[i] = type.array_size_literal[i];
                    }

                    ty->array = array;
                    ty->array_size_literal = array_size_literal;
                }

                if (type.basetype == spirv_cross::SPIRType::Image || type.basetype == spirv_cross::SPIRType::SampledImage)
                {
                    ty->image = type.image;
                }

                *spirv_type = ty;
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_member_name(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const char **name)
    {
        INTERNAL_RESULT(
            do {
                auto const member_name = ((spirv_cross::Compiler *)compiler)->get_member_name(id, index);
                *name = strdup(member_name.c_str());
            } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_member_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const spv::Decoration decoration, uint32_t *result)
    {
        INTERNAL_RESULT(*result = ((spirv_cross::Compiler *)compiler)->get_member_decoration(id, index, decoration);)
    }

    ScInternalResult sc_internal_compiler_set_member_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const spv::Decoration decoration, const uint32_t argument)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_member_decoration(id, index, decoration, argument);)
    }

    ScInternalResult sc_internal_compiler_get_declared_struct_size(const ScInternalCompilerBase *compiler, const uint32_t id, uint32_t *result)
    {
        INTERNAL_RESULT(do {
            auto const &comp = ((spirv_cross::Compiler *)compiler);
            *result = comp->get_declared_struct_size(comp->get_type(id));
        } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_declared_struct_member_size(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, uint32_t *result)
    {
        INTERNAL_RESULT(do {
            auto const &comp = ((spirv_cross::Compiler *)compiler);
            *result = comp->get_declared_struct_member_size(comp->get_type(id), index);
        } while (0);)
    }

    ScInternalResult sc_internal_compiler_rename_interface_variable(const ScInternalCompilerBase *compiler, const ScResource *resources, const size_t resources_size, uint32_t location, const char *name)
    {
        INTERNAL_RESULT(do {
            spirv_cross::SmallVector<spirv_cross::Resource> sc_resources;
            for (size_t i = 0; i < resources_size; i++)
            {
                auto const &resource = resources[i];
                spirv_cross::Resource sc_resource;
                std::string sc_name(resource.name);
                sc_resource.id = resource.id;
                sc_resource.type_id = resource.type_id;
                sc_resource.base_type_id = resource.base_type_id;
                sc_resource.name = sc_name;
                sc_resources.push_back(sc_resource);
            }

            auto &comp = *(spirv_cross::Compiler *)compiler;
            std::string new_name(name);
            spirv_cross_util::rename_interface_variable(comp, sc_resources, location, new_name);
        } while (0);)
    }

    ScInternalResult sc_internal_compiler_get_work_group_size_specialization_constants(const ScInternalCompilerBase *compiler, ScSpecializationConstant **constants)
    {
        INTERNAL_RESULT(do {
            spirv_cross::SpecializationConstant wg_x;
            spirv_cross::SpecializationConstant wg_y;
            spirv_cross::SpecializationConstant wg_z;
            ((const spirv_cross::Compiler *)compiler)->get_work_group_size_specialization_constants(wg_x, wg_y, wg_z);

            auto p_constants = (ScSpecializationConstant *)malloc(3 * sizeof(ScSpecializationConstant));
            p_constants[0].id = wg_x.id;
            p_constants[0].constant_id = wg_x.constant_id;
            p_constants[1].id = wg_y.id;
            p_constants[1].constant_id = wg_y.constant_id;
            p_constants[2].id = wg_z.id;
            p_constants[2].constant_id = wg_z.constant_id;

            *constants = p_constants;
        } while (0);)
    }


    ScInternalResult sc_internal_compiler_get_active_interface_variables(const ScInternalCompilerBase *compiler, uint32_t **ids, size_t* size) {
        INTERNAL_RESULT(do {
            auto const sc_active_variables = ((const spirv_cross::Compiler *)compiler)->get_active_interface_variables();
            auto const sc_size = sc_active_variables.size();

            *ids = (uint32_t *)malloc(sc_active_variables.size() * sizeof(uint32_t));
            *size = sc_size;
            size_t index = 0;
            for (const auto& elem : sc_active_variables){
                (*ids)[index++] = (uint32_t) elem;
            }
        } while(0);)
    }

    ScInternalResult sc_internal_compiler_set_entry_point(const ScInternalCompilerBase *compiler, const char *name, const spv::ExecutionModel execution_model)
    {
        INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_entry_point(name, execution_model);)
    }

    ScInternalResult sc_internal_compiler_compile(const ScInternalCompilerBase *compiler, const char **shader)
    {
        INTERNAL_RESULT(*shader = strdup(((spirv_cross::Compiler *)compiler)->compile().c_str());)
    }

    ScInternalResult sc_internal_compiler_delete(ScInternalCompilerBase *compiler)
    {
        INTERNAL_RESULT(delete (spirv_cross::Compiler *)compiler;)
    }

    ScInternalResult sc_internal_free_pointer(void *pointer)
    {
        INTERNAL_RESULT(free(pointer);)
    }
}

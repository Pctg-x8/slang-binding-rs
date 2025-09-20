use core::{
    cell::UnsafeCell,
    ffi::{CStr, c_float, c_int, c_uint},
    mem::MaybeUninit,
    ptr::NonNull,
};

use crate::{ffi, *};

pub use crate::ffi::{
    SlangReflectionGenericArg as GenericArg, SlangReflectionGenericArgType as GenericArgType,
};

#[repr(transparent)]
pub struct Attribute(UnsafeCell<ffi::SlangReflectionUserAttribute>);
impl Attribute {
    pub const unsafe fn from_mut_ptr<'a>(
        ptr: *mut ffi::SlangReflectionUserAttribute,
    ) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline(always)]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionUserAttribute_GetName(self.0.get())) }
    }

    #[inline(always)]
    pub fn argument_count(&self) -> u32 {
        unsafe { ffi::spReflectionUserAttribute_GetArgumentCount(self.0.get()) }
    }

    #[inline(always)]
    pub fn argument_type(&self, index: u32) -> &mut Type {
        unsafe {
            core::mem::transmute(UnsafeCell::from_mut(
                &mut *ffi::spReflectionUserAttribute_GetArgumentType(self.0.get(), index),
            ))
        }
    }

    pub fn argument_value_int(&self, index: u32) -> crate::Result<c_int> {
        let mut o = MaybeUninit::uninit();
        crate::rw(unsafe {
            ffi::spReflectionUserAttribute_GetArgumentValueInt(self.0.get(), index, o.as_mut_ptr())
        })?;

        Ok(unsafe { o.assume_init() })
    }

    pub fn argument_value_float(&self, index: u32) -> crate::Result<c_float> {
        let mut o = MaybeUninit::uninit();
        crate::rw(unsafe {
            ffi::spReflectionUserAttribute_GetArgumentValueFloat(
                self.0.get(),
                index,
                o.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { o.assume_init() })
    }

    pub fn argument_value_string(&self, index: u32) -> Option<&CStr> {
        let p = unsafe {
            ffi::spReflectionUserAttribute_GetArgumentValueString(
                self.0.get(),
                index,
                core::ptr::null_mut(),
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(p) })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TypeKind {
    None = ffi::SLANG_TYPE_KIND_NONE,
    Struct = ffi::SLANG_TYPE_KIND_STRUCT,
    Array = ffi::SLANG_TYPE_KIND_ARRAY,
    Matrix = ffi::SLANG_TYPE_KIND_MATRIX,
    Vector = ffi::SLANG_TYPE_KIND_VECTOR,
    Scalar = ffi::SLANG_TYPE_KIND_SCALAR,
    ConstantBuffer = ffi::SLANG_TYPE_KIND_CONSTANT_BUFFER,
    Resource = ffi::SLANG_TYPE_KIND_RESOURCE,
    SamplerState = ffi::SLANG_TYPE_KIND_SAMPLER_STATE,
    TextureBuffer = ffi::SLANG_TYPE_KIND_TEXTURE_BUFFER,
    ShaderStorageBuffer = ffi::SLANG_TYPE_KIND_SHADER_STORAGE_BUFFER,
    ParameterBlock = ffi::SLANG_TYPE_KIND_PARAMETER_BLOCK,
    GenericTypeParameter = ffi::SLANG_TYPE_KIND_GENERIC_TYPE_PARAMETER,
    Interface = ffi::SLANG_TYPE_KIND_INTERFACE,
    OutputStream = ffi::SLANG_TYPE_KIND_OUTPUT_STREAM,
    Specialized = ffi::SLANG_TYPE_KIND_SPECIALIZED,
    Feedback = ffi::SLANG_TYPE_KIND_FEEDBACK,
    Pointer = ffi::SLANG_TYPE_KIND_POINTER,
    DynamicResource = ffi::SLANG_TYPE_KIND_DYNAMIC_RESOURCE,
    MeshOutput = ffi::SLANG_TYPE_KIND_MESH_OUTPUT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ScalarType {
    None = ffi::SLANG_SCALAR_TYPE_NONE,
    Void = ffi::SLANG_SCALAR_TYPE_VOID,
    Bool = ffi::SLANG_SCALAR_TYPE_BOOL,
    Int32 = ffi::SLANG_SCALAR_TYPE_INT32,
    UInt32 = ffi::SLANG_SCALAR_TYPE_UINT32,
    Int64 = ffi::SLANG_SCALAR_TYPE_INT64,
    UInt64 = ffi::SLANG_SCALAR_TYPE_UINT64,
    Float16 = ffi::SLANG_SCALAR_TYPE_FLOAT16,
    Float32 = ffi::SLANG_SCALAR_TYPE_FLOAT32,
    Float64 = ffi::SLANG_SCALAR_TYPE_FLOAT64,
    Int8 = ffi::SLANG_SCALAR_TYPE_INT8,
    UInt8 = ffi::SLANG_SCALAR_TYPE_UINT8,
    Int16 = ffi::SLANG_SCALAR_TYPE_INT16,
    UInt16 = ffi::SLANG_SCALAR_TYPE_UINT16,
}

#[repr(transparent)]
pub struct Type(UnsafeCell<SlangReflectionType>);
impl Type {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionType) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline(always)]
    pub fn kind(&self) -> TypeKind {
        unsafe { core::mem::transmute(ffi::spReflectionType_GetKind(self.0.get())) }
    }

    #[inline(always)]
    pub fn field_count(&self) -> c_uint {
        unsafe { ffi::spReflectionType_GetFieldCount(self.0.get()) }
    }

    pub fn field(&self, index: c_uint) -> Option<&mut Variable> {
        let p = unsafe { ffi::spReflectionType_GetFieldByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }

    #[inline(always)]
    pub fn element_count(&self, reflection: Option<&Shader>) -> usize {
        unsafe {
            ffi::spReflectionType_GetSpecializedElementCount(
                self.0.get(),
                reflection.map_or_else(core::ptr::null_mut, |x| x.0.get()),
            )
        }
    }

    pub fn element_type(&self) -> Option<&mut Type> {
        let p = unsafe { ffi::spReflectionType_GetElementType(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }

    #[inline(always)]
    pub fn row_count(&self) -> c_uint {
        unsafe { ffi::spReflectionType_GetRowCount(self.0.get()) }
    }

    #[inline(always)]
    pub fn column_count(&self) -> c_uint {
        unsafe { ffi::spReflectionType_GetColumnCount(self.0.get()) }
    }

    #[inline(always)]
    pub fn scalar_type(&self) -> ScalarType {
        unsafe { core::mem::transmute(ffi::spReflectionType_GetScalarType(self.0.get())) }
    }

    pub fn resource_result_type(&self) -> Option<&mut Type> {
        let p = unsafe { ffi::spReflectionType_GetResourceResultType(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }

    #[inline(always)]
    pub fn resource_shape(&self) -> ffi::SlangResourceShape {
        unsafe { ffi::spReflectionType_GetResourceShape(self.0.get()) }
    }

    #[inline(always)]
    pub fn resource_access(&self) -> ffi::SlangResourceAccess {
        unsafe { ffi::spReflectionType_GetResourceAccess(self.0.get()) }
    }

    #[inline(always)]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionType_GetName(self.0.get())) }
    }

    pub fn full_name(&self) -> crate::Result<crate::IBlobPtr> {
        let mut o = MaybeUninit::uninit();
        crate::rw(unsafe { ffi::spReflectionType_GetFullName(self.0.get(), o.as_mut_ptr()) })?;

        Ok(IBlobPtr(unsafe { NonNull::new_unchecked(o.assume_init()) }))
    }

    #[inline(always)]
    pub fn user_attribute_count(&self) -> c_uint {
        unsafe { ffi::spReflectionType_GetUserAttributeCount(self.0.get()) }
    }

    pub fn user_attribute(&self, index: c_uint) -> Option<&mut Attribute> {
        let p = unsafe { ffi::spReflectionType_GetUserAttribute(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }

    pub fn find_user_attribute_by_name(&self, name: &CStr) -> Option<&mut Attribute> {
        let p =
            unsafe { ffi::spReflectionType_FindUserAttributeByName(self.0.get(), name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }

    #[inline(always)]
    pub fn apply_specializations(&self, generic: &Generic) -> &mut Type {
        unsafe {
            core::mem::transmute(UnsafeCell::from_mut(
                &mut *ffi::spReflectionType_applySpecializations(self.0.get(), generic.0.get()),
            ))
        }
    }

    pub fn generic_container(&self) -> Option<&mut Generic> {
        let p = unsafe { ffi::spReflectionType_GetGenericContainer(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *p)) })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ParameterCategory {
    None = ffi::SLANG_PARAMETER_CATEGORY_NONE,
    Mixed = ffi::SLANG_PARAMETER_CATEGORY_MIXED,
    ConstantBuffer = ffi::SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER,
    ShaderResource = ffi::SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE,
    UnorderedAccess = ffi::SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS,
    VaryingInput = ffi::SLANG_PARAMETER_CATEGORY_VARYING_INPUT,
    VaryingOutput = ffi::SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT,
    SamplerState = ffi::SLANG_PARAMETER_CATEGORY_SAMPLER_STATE,
    Uniform = ffi::SLANG_PARAMETER_CATEGORY_UNIFORM,
    DescriptorTableSlot = ffi::SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT,
    SpecializationConstant = ffi::SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT,
    PushConstantBuffer = ffi::SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER,
    RegisterSpace = ffi::SLANG_PARAMETER_CATEGORY_REGISTER_SPACE,
    GenericResource = ffi::SLANG_PARAMETER_CATEGORY_GENERIC,
    RayPayload = ffi::SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD,
    HitAttributes = ffi::SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES,
    CallablePayload = ffi::SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD,
    ShaderRecord = ffi::SLANG_PARAMETER_CATEGORY_SHADER_RECORD,
    ExistentialTypeParam = ffi::SLANG_PARAMETER_CATEGORY_EXISTENTIAL_TYPE_PARAM,
    ExistentialObjectParam = ffi::SLANG_PARAMETER_CATEGORY_EXISTENTIAL_OBJECT_PARAM,
    SubElementRegisterSpace = ffi::SLANG_PARAMETER_CATEGORY_SUB_ELEMENT_REGISTER_SPACE,
    InputAttachmentIndex = ffi::SLANG_PARAMETER_CATEGORY_SUBPASS,
    // Alias: MetalBuffer = ConstantBuffer
    // Alias: MetalTexture = ShaderResource
    MetalArgumentBufferElement = ffi::SLANG_PARAMETER_CATEGORY_METAL_ARGUMENT_BUFFER_ELEMENT,
    MetalAttribute = ffi::SLANG_PARAMETER_CATEGORY_METAL_ATTRIBUTE,
    MetalPayload = ffi::SLANG_PARAMETER_CATEGORY_METAL_PAYLOAD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BindingType {
    Unknown = ffi::SLANG_BINDING_TYPE_UNKNOWN,
    Sampler = ffi::SLANG_BINDING_TYPE_SAMPLER,
    Texture = ffi::SLANG_BINDING_TYPE_TEXTURE,
    ConstantBuffer = ffi::SLANG_BINDING_TYPE_CONSTANT_BUFFER,
    ParameterBlock = ffi::SLANG_BINDING_TYPE_PARAMETER_BLOCK,
    TypedBuffer = ffi::SLANG_BINDING_TYPE_TYPED_BUFFER,
    RawBuffer = ffi::SLANG_BINDING_TYPE_RAW_BUFFER,
    CombinedTextureSampler = ffi::SLANG_BINDING_TYPE_COMBINED_TEXTURE_SAMPLER,
    InputRenderTarget = ffi::SLANG_BINDING_TYPE_INPUT_RENDER_TARGET,
    InlineUniformData = ffi::SLANG_BINDING_TYPE_INLINE_UNIFORM_DATA,
    RayTracingAccelerationStructure = ffi::SLANG_BINDING_TYPE_RAY_TRACING_ACCELERATION_STRUCTURE,
    VaryingInput = ffi::SLANG_BINDING_TYPE_VARYING_INPUT,
    VaryingOutput = ffi::SLANG_BINDING_TYPE_VARYING_OUTPUT,
    ExistentialValue = ffi::SLANG_BINDING_TYPE_EXISTENTIAL_VALUE,
    PushConstant = ffi::SLANG_BINDING_TYPE_PUSH_CONSTANT,
    MutableFlag = ffi::SLANG_BINDING_TYPE_MUTABLE_FLAG,
    MutableTexture = ffi::SLANG_BINDING_TYPE_MUTABLE_TETURE,
    MutableTypedBuffer = ffi::SLANG_BINDING_TYPE_MUTABLE_TYPED_BUFFER,
    MutableRawBuffer = ffi::SLANG_BINDING_TYPE_MUTABLE_RAW_BUFFER,
    BaseMask = ffi::SLANG_BINDING_TYPE_BASE_MASK,
    ExtMask = ffi::SLANG_BINDING_TYPE_EXT_MASK,
}

#[repr(transparent)]
pub struct TypeLayout(UnsafeCell<ffi::SlangReflectionTypeLayout>);
impl TypeLayout {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionTypeLayout) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn r#type(&self) -> &mut Type {
        unsafe { Type::from_mut_ptr(ffi::spReflectionTypeLayout_GetType(self.0.get())) }
    }

    #[inline]
    pub fn kind(&self) -> TypeKind {
        unsafe { core::mem::transmute(ffi::spReflectionTypeLayout_getKind(self.0.get())) }
    }

    #[inline]
    pub fn size(&self, category: ParameterCategory) -> usize {
        unsafe { ffi::spReflectionTypeLayout_GetSize(self.0.get(), category as _) }
    }

    #[inline]
    pub fn stride(&self, category: ParameterCategory) -> usize {
        unsafe { ffi::spReflectionTypeLayout_GetStride(self.0.get(), category as _) }
    }

    #[inline]
    pub fn alignment(&self, category: ParameterCategory) -> i32 {
        unsafe { ffi::spReflectionTypeLayout_getAlignment(self.0.get(), category as _) }
    }

    #[inline]
    pub fn field_count(&self) -> c_uint {
        unsafe { ffi::spReflectionTypeLayout_GetFieldCount(self.0.get()) }
    }

    pub fn field(&self, index: c_uint) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_GetFieldByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn find_field_index_by_name(&self, name_begin: &CStr, name_end: Option<&CStr>) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_findFieldIndexByName(
                self.0.get(),
                name_begin.as_ptr(),
                name_end.map_or_else(core::ptr::null, CStr::as_ptr),
            )
        }
    }

    pub fn explicit_counter(&self) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_GetExplicitCounter(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline(always)]
    pub fn element_count(&self, reflection: Option<&Shader>) -> usize {
        self.r#type().element_count(reflection)
    }

    #[inline]
    pub fn element_stride(&self, category: ParameterCategory) -> usize {
        unsafe { ffi::spReflectionTypeLayout_GetElementStride(self.0.get(), category as _) }
    }

    pub fn element_type_layout(&self) -> Option<&mut TypeLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_GetElementTypeLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeLayout::from_mut_ptr(p) })
        }
    }

    pub fn element_var_layout(&self) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_GetElementVarLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    pub fn container_var_layout(&self) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_getContainerVarLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn parameter_category(&self) -> ParameterCategory {
        unsafe {
            core::mem::transmute(ffi::spReflectionTypeLayout_GetParameterCategory(
                self.0.get(),
            ))
        }
    }

    #[inline]
    pub fn category_count(&self) -> c_uint {
        unsafe { ffi::spReflectionTypeLayout_GetCategoryCount(self.0.get()) }
    }

    #[inline]
    pub fn category(&self, index: c_uint) -> ParameterCategory {
        unsafe {
            core::mem::transmute(ffi::spReflectionTypeLayout_GetCategoryByIndex(
                self.0.get(),
                index,
            ))
        }
    }

    #[inline(always)]
    pub fn row_count(&self) -> c_uint {
        self.r#type().row_count()
    }

    #[inline(always)]
    pub fn column_count(&self) -> c_uint {
        self.r#type().column_count()
    }

    #[inline(always)]
    pub fn scalar_type(&self) -> ScalarType {
        self.r#type().scalar_type()
    }

    #[inline(always)]
    pub fn resource_result_type(&self) -> Option<&mut Type> {
        self.r#type().resource_result_type()
    }

    #[inline(always)]
    pub fn resource_shape(&self) -> crate::ResourceShape {
        self.r#type().resource_shape()
    }

    #[inline(always)]
    pub fn resource_access(&self) -> crate::ResourceAccess {
        self.r#type().resource_access()
    }

    #[inline(always)]
    pub fn name(&self) -> &CStr {
        self.r#type().name()
    }

    #[inline]
    pub fn matrix_layout_mode(&self) -> crate::MatrixLayoutMode {
        unsafe { ffi::spReflectionTypeLayout_GetMatrixLayoutMode(self.0.get()) }
    }

    #[inline]
    pub fn generic_param_index(&self) -> c_int {
        unsafe { ffi::spReflectionTypeLayout_getGenericParamIndex(self.0.get()) }
    }

    pub fn pending_data_type_layout(&self) -> Option<&mut TypeLayout> {
        let p = unsafe { ffi::spReflectionTypeLayout_getPendingDataTypeLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeLayout::from_mut_ptr(p) })
        }
    }

    pub fn specialized_type_pending_data_var_layout(&self) -> Option<&mut VariableLayout> {
        let p = unsafe {
            ffi::spReflectionTypeLayout_getSpecializedTypePendingDataVarLayout(self.0.get())
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn binding_range_count(&self) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getBindingRangeCount(self.0.get()) }
    }

    #[inline]
    pub fn binding_range_type(&self, index: SlangInt) -> BindingType {
        unsafe {
            core::mem::transmute(ffi::spReflectionTypeLayout_getBindingRangeType(
                self.0.get(),
                index,
            ))
        }
    }

    #[inline]
    pub fn is_binding_range_specializable(&self, index: SlangInt) -> bool {
        unsafe { ffi::spReflectionTypeLayout_isBindingRangeSpecializable(self.0.get(), index) != 0 }
    }

    #[inline]
    pub fn binding_range_binding_count(&self, index: SlangInt) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getBindingRangeBindingCount(self.0.get(), index) }
    }

    #[inline]
    pub fn field_binding_range_offset(&self, field_index: SlangInt) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getFieldBindingRangeOffset(self.0.get(), field_index) }
    }

    #[inline]
    pub fn explicit_counter_binding_range_offset(&self) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getExplicitCounterBindingRangeOffset(self.0.get()) }
    }

    pub fn binding_range_leaf_type_layout(&self, index: SlangInt) -> Option<&mut TypeLayout> {
        let p = unsafe {
            ffi::spReflectionTypeLayout_getBindingRangeLeafTypeLayout(self.0.get(), index)
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeLayout::from_mut_ptr(p) })
        }
    }

    pub fn binding_range_leaf_variable(&self, index: SlangInt) -> Option<&mut Variable> {
        let p =
            unsafe { ffi::spReflectionTypeLayout_getBindingRangeLeafVariable(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn binding_range_image_format(&self, index: SlangInt) -> crate::ImageFormat {
        unsafe { ffi::spReflectionTypeLayout_getBindingRAngeImageFormat(self.0.get(), index) }
    }

    #[inline]
    pub fn binding_range_descriptor_set_index(&self, index: SlangInt) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getBindingRAngeDescriptorSetIndex(self.0.get(), index)
        }
    }

    #[inline]
    pub fn binding_range_first_descriptor_range_index(&self, index: SlangInt) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getBindingRangeFirstDescriptorRangeIndex(
                self.0.get(),
                index,
            )
        }
    }

    #[inline]
    pub fn binding_range_descriptor_range_count(&self, index: SlangInt) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getBindingRangeDescriptorRangeCount(self.0.get(), index)
        }
    }

    #[inline]
    pub fn descriptor_set_count(&self) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getDescriptorSetCount(self.0.get()) }
    }

    #[inline]
    pub fn descriptor_set_space_offset(&self, set_index: SlangInt) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getDescriptorSetSpaceOffset(self.0.get(), set_index) }
    }

    #[inline]
    pub fn descriptor_set_descriptor_range_count(&self, set_index: SlangInt) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getDescriptorSetDescriptorRangeCount(
                self.0.get(),
                set_index,
            )
        }
    }

    #[inline]
    pub fn descriptor_set_descriptor_range_index_offset(
        &self,
        set_index: SlangInt,
        range_index: SlangInt,
    ) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getDescriptorSetDescriptorRangeIndexOffset(
                self.0.get(),
                set_index,
                range_index,
            )
        }
    }

    #[inline]
    pub fn descriptor_set_descriptor_range_descriptor_count(
        &self,
        set_index: SlangInt,
        range_index: SlangInt,
    ) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getDescriptorSetDescriptorRangeDescriptorCount(
                self.0.get(),
                set_index,
                range_index,
            )
        }
    }

    #[inline]
    pub fn descriptor_set_descriptor_range_type(
        &self,
        set_index: SlangInt,
        range_index: SlangInt,
    ) -> BindingType {
        unsafe {
            core::mem::transmute(
                ffi::spReflectionTypeLayout_getDescriptorSetDescriptorRangeType(
                    self.0.get(),
                    set_index,
                    range_index,
                ),
            )
        }
    }

    #[inline]
    pub fn descriptor_set_descriptor_range_category(
        &self,
        set_index: SlangInt,
        range_index: SlangInt,
    ) -> ParameterCategory {
        unsafe {
            core::mem::transmute(
                ffi::spReflectionTypeLayout_getDescriptorSetDescriptorRangeCategory(
                    self.0.get(),
                    set_index,
                    range_index,
                ),
            )
        }
    }

    #[inline]
    pub fn sub_object_range_count(&self) -> SlangInt {
        unsafe { ffi::spReflectionTypeLayout_getSubObjectRangeCount(self.0.get()) }
    }

    #[inline]
    pub fn sub_object_range_binding_range_index(
        &self,
        sub_object_range_index: SlangInt,
    ) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getSubObjectRangeBindingRangeIndex(
                self.0.get(),
                sub_object_range_index,
            )
        }
    }

    #[inline]
    pub fn sub_object_range_space_offset(&self, sub_object_range_index: SlangInt) -> SlangInt {
        unsafe {
            ffi::spReflectionTypeLayout_getSubObjectRangeSpaceOffset(
                self.0.get(),
                sub_object_range_index,
            )
        }
    }

    pub fn sub_object_range_offset(
        &self,
        sub_object_range_index: SlangInt,
    ) -> Option<&mut VariableLayout> {
        let p = unsafe {
            ffi::spReflectionTypeLayout_getSubObjectRangeOffset(
                self.0.get(),
                sub_object_range_index,
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }
}

#[repr(transparent)]
pub struct Variable(UnsafeCell<ffi::SlangReflectionVariable>);
impl Variable {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionVariable) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionVariable_GetName(self.0.get())) }
    }

    #[inline]
    pub fn r#type(&self) -> &mut Type {
        unsafe { Type::from_mut_ptr(ffi::spReflectionVariable_GetType(self.0.get())) }
    }

    pub fn find_modifier(&self, id: crate::ModifierID) -> Option<&mut Modifier> {
        let p = unsafe { ffi::spReflectionVariable_FindModifier(self.0.get(), id) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Modifier::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn user_attribute_count(&self) -> c_uint {
        unsafe { ffi::spReflectionVariable_GetUserAttributeCount(self.0.get()) }
    }

    pub fn user_attribute(&self, index: c_uint) -> Option<&mut Attribute> {
        let p = unsafe { ffi::spReflectionVariable_GetUserAttribute(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Attribute::from_mut_ptr(p) })
        }
    }

    pub fn find_user_attribute_by_name(
        &self,
        global_session: &impl IGlobalSession,
        name: &CStr,
    ) -> Option<&mut Attribute> {
        let p = unsafe {
            ffi::spReflectionVariable_FindUserAttributeByName(
                self.0.get(),
                global_session.thisptr(),
                name.as_ptr(),
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Attribute::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn has_default_value(&self) -> bool {
        unsafe { ffi::spReflectionVariable_HasDefaultValue(self.0.get()) }
    }

    pub fn default_value_int(&self) -> crate::Result<i64> {
        let mut o = MaybeUninit::uninit();
        crate::rw(unsafe {
            ffi::spReflectionVariable_GetDefaultValueInt(self.0.get(), o.as_mut_ptr())
        })?;

        Ok(unsafe { o.assume_init() })
    }

    pub fn generic_container(&self) -> Option<&mut Generic> {
        let p = unsafe { ffi::spReflectionVariable_GetGenericContainer(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Generic::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn apply_specializations(&self, generic: &Generic) -> &mut Variable {
        unsafe {
            Variable::from_mut_ptr(ffi::spReflectionVariable_applySpecializations(
                self.0.get(),
                generic.0.get(),
            ))
        }
    }
}

#[repr(transparent)]
pub struct VariableLayout(UnsafeCell<ffi::SlangReflectionVariableLayout>);
impl VariableLayout {
    pub const unsafe fn from_mut_ptr<'a>(
        ptr: *mut ffi::SlangReflectionVariableLayout,
    ) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn variable(&self) -> &mut Variable {
        unsafe { Variable::from_mut_ptr(ffi::spReflectionVariableLayout_GetVariable(self.0.get())) }
    }

    #[inline(always)]
    pub fn name(&self) -> &CStr {
        self.variable().name()
    }

    #[inline(always)]
    pub fn find_modifier(&self, id: crate::ModifierID) -> Option<&mut Modifier> {
        self.variable().find_modifier(id)
    }

    #[inline]
    pub fn type_layout(&self) -> &mut TypeLayout {
        unsafe {
            TypeLayout::from_mut_ptr(ffi::spReflectionVariableLayout_GetTypeLayout(self.0.get()))
        }
    }

    #[inline(always)]
    pub fn category(&self) -> ParameterCategory {
        self.type_layout().parameter_category()
    }

    #[inline(always)]
    pub fn category_count(&self) -> c_uint {
        self.type_layout().category_count()
    }

    #[inline(always)]
    pub fn category_by_index(&self, index: c_uint) -> ParameterCategory {
        self.type_layout().category(index)
    }

    #[inline]
    pub fn offset(&self, category: ParameterCategory) -> usize {
        unsafe { ffi::spReflectionVariableLayout_GetOffset(self.0.get(), category as _) }
    }

    #[inline(always)]
    pub fn r#type(&self) -> &mut Type {
        self.variable().r#type()
    }

    #[inline]
    pub fn binding_index(&self) -> c_uint {
        unsafe { ffi::spReflectionParameter_GetBindingIndex(self.0.get()) }
    }

    #[inline]
    pub fn binding_space(&self) -> c_uint {
        unsafe { ffi::spReflectionParameter_GetBindingSpace(self.0.get()) }
    }

    #[inline]
    pub fn space(&self, category: ParameterCategory) -> usize {
        unsafe { ffi::spReflectionVariableLayout_GetSpace(self.0.get(), category as _) }
    }

    #[inline]
    pub fn image_format(&self) -> crate::ImageFormat {
        unsafe { ffi::spReflectionVariableLayout_GetImageFormat(self.0.get()) }
    }

    pub fn semantic_name(&self) -> Option<&CStr> {
        let p = unsafe { ffi::spReflectionVariableLayout_GetSemanticName(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(p) })
        }
    }

    #[inline]
    pub fn semantic_index(&self) -> usize {
        unsafe { ffi::spReflectionVariableLayout_GetSemanticIndex(self.0.get()) }
    }

    #[inline]
    pub fn stage(&self) -> crate::Stage {
        unsafe { ffi::spReflectionVariableLayout_getStage(self.0.get()) }
    }

    pub fn pending_data_layout(&self) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionVariableLayout_getPendingDataLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }
}

#[repr(transparent)]
pub struct Function(UnsafeCell<ffi::SlangReflectionFunction>);
impl Function {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionFunction) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionFunction_GetName(self.0.get())) }
    }

    #[inline]
    pub fn result_type(&self) -> &mut Type {
        unsafe { Type::from_mut_ptr(ffi::spReflectionFunction_GetResultType(self.0.get())) }
    }

    #[inline]
    pub fn parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflectionFunction_GetParameterCount(self.0.get()) }
    }

    pub fn parameter(&self, index: c_uint) -> Option<&mut Variable> {
        let p = unsafe { ffi::spReflectionFunction_GetParameter(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn user_attribute_count(&self) -> c_uint {
        unsafe { ffi::spReflectionFunction_GetUserAttributeCount(self.0.get()) }
    }

    pub fn user_attribute(&self, index: c_uint) -> Option<&mut Attribute> {
        let p = unsafe { ffi::spReflectionFunction_GetUserAttribute(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Attribute::from_mut_ptr(p) })
        }
    }

    pub fn find_user_attribute_by_name(
        &self,
        global_session: &impl IGlobalSession,
        name: &CStr,
    ) -> Option<&mut Attribute> {
        let p = unsafe {
            ffi::spReflectionFunction_FindUserAttributeByName(
                self.0.get(),
                global_session.thisptr(),
                name.as_ptr(),
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Attribute::from_mut_ptr(p) })
        }
    }

    pub fn find_modifier(&self, id: crate::ModifierID) -> Option<&mut Modifier> {
        let p = unsafe { ffi::spReflectionFunction_FindModifier(self.0.get(), id) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Modifier::from_mut_ptr(p) })
        }
    }

    pub fn generic_container(&self) -> Option<&mut Generic> {
        let p = unsafe { ffi::spReflectionFunction_GetGenericContainer(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Generic::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn apply_specializations(&self, generic: &Generic) -> &mut Function {
        unsafe {
            Function::from_mut_ptr(ffi::spReflectionFunction_applySpecializations(
                self.0.get(),
                generic.0.get(),
            ))
        }
    }

    #[inline]
    pub fn specialize_with_arg_types(&self, arg_types: &[&Type]) -> &mut Function {
        unsafe {
            Function::from_mut_ptr(ffi::spReflectionFunction_specializeWithArgTypes(
                self.0.get(),
                arg_types.len() as _,
                arg_types.as_ptr() as _,
            ))
        }
    }

    #[inline]
    pub fn is_overloaded(&self) -> bool {
        unsafe { ffi::spReflectionFunction_isOverloaded(self.0.get()) }
    }

    #[inline]
    pub fn overload_count(&self) -> c_uint {
        unsafe { ffi::spReflectionFunction_getOverloadCount(self.0.get()) }
    }

    pub fn overload(&self, index: c_uint) -> Option<&mut Function> {
        let p = unsafe { ffi::spReflectionFunction_getOverload(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Function::from_mut_ptr(p) })
        }
    }
}

#[repr(transparent)]
pub struct Generic(UnsafeCell<ffi::SlangReflectionGeneric>);
impl Generic {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionGeneric) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn as_decl(&self) -> &mut Decl {
        unsafe { Decl::from_mut_ptr(ffi::spReflectionGeneric_asDecl(self.0.get())) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionGeneric_GetName(self.0.get())) }
    }

    #[inline]
    pub fn type_parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflectionGeneric_GetTypeParameterCount(self.0.get()) }
    }

    pub fn type_parameter(&self, index: c_uint) -> Option<&mut Variable> {
        let p = unsafe { ffi::spReflectionGeneric_GetTypeParameter(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn value_parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflectionGeneric_GetValueParameterCount(self.0.get()) }
    }

    pub fn value_parameter(&self, index: c_uint) -> Option<&mut Variable> {
        let p = unsafe { ffi::spReflectionGeneric_GetValueParameter(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn type_parameter_constraint_count(&self, type_param: &Variable) -> c_uint {
        unsafe {
            ffi::spReflectionGeneric_GetTypeParameterConstraintCount(
                self.0.get(),
                type_param.0.get(),
            )
        }
    }

    pub fn type_parameter_constraint_type(
        &self,
        type_param: &Variable,
        index: c_uint,
    ) -> Option<&mut Type> {
        let p = unsafe {
            ffi::spReflectionGeneric_GetTypeParameterConstraintType(
                self.0.get(),
                type_param.0.get(),
                index,
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }

    pub fn inner_decl(&self) -> Option<&mut Decl> {
        let p = unsafe { ffi::spReflectionGeneric_GetInnerDecl(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Decl::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn inner_kind(&self) -> crate::DeclKind {
        unsafe { ffi::spReflectionGeneric_GetInnerKind(self.0.get()) }
    }

    pub fn outer_generic_container(&self) -> Option<&mut Generic> {
        let p = unsafe { ffi::spReflectionGeneric_GetOuterGenericContainer(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Generic::from_mut_ptr(p) })
        }
    }

    pub fn concrete_type(&self, type_param: &Variable) -> Option<&mut Type> {
        let p =
            unsafe { ffi::spReflectionGeneric_GetConcreteType(self.0.get(), type_param.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn concrete_int_val(&self, value_param: &Variable) -> i64 {
        unsafe { ffi::spReflectionGeneric_GetConcreteIntVal(self.0.get(), value_param.0.get()) }
    }

    #[inline]
    pub fn apply_specializations(&self, generic: &Generic) -> &mut Generic {
        unsafe {
            Generic::from_mut_ptr(ffi::spReflectionGeneric_applySpecializations(
                self.0.get(),
                generic.0.get(),
            ))
        }
    }
}

#[repr(transparent)]
pub struct EntryPoint(UnsafeCell<ffi::SlangReflectionEntryPoint>);
impl EntryPoint {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionEntryPoint) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionEntryPoint_getName(self.0.get())) }
    }

    pub fn name_override(&self) -> Option<&CStr> {
        let p = unsafe { ffi::spReflectionEntryPoint_getNameOverride(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(p) })
        }
    }

    #[inline]
    pub fn parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflectionEntryPoint_getParameterCount(self.0.get()) }
    }

    #[inline]
    pub fn function(&self) -> &mut Function {
        unsafe { Function::from_mut_ptr(ffi::spReflectionEntryPoint_getFunction(self.0.get())) }
    }

    pub fn parameter(&self, index: c_uint) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflectionEntryPoint_getParameterByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn stage(&self) -> crate::Stage {
        unsafe { ffi::spReflectionEntryPoint_getStage(self.0.get()) }
    }

    #[inline]
    pub fn compute_thread_group_size(&self, out_size_along_axis: &mut [MaybeUninit<SlangUInt>]) {
        unsafe {
            ffi::spReflectionEntryPoint_getComputeThreadGroupSize(
                self.0.get(),
                out_size_along_axis.len() as _,
                out_size_along_axis.as_mut_ptr() as _,
            )
        }
    }

    #[inline]
    pub fn compute_wave_size(&self) -> SlangUInt {
        let mut o = MaybeUninit::uninit();
        unsafe {
            ffi::spReflectionEntryPoint_getComputeWaveSize(self.0.get(), o.as_mut_ptr());
        }

        unsafe { o.assume_init() }
    }

    #[inline]
    pub fn uses_any_sample_rate_input(&self) -> bool {
        unsafe { ffi::spReflectionEntryPoint_usesAnySampleRateInput(self.0.get()) != 0 }
    }

    #[inline]
    pub fn var_layout(&self) -> &mut VariableLayout {
        unsafe {
            VariableLayout::from_mut_ptr(ffi::spReflectionEntryPoint_getVarLayout(self.0.get()))
        }
    }

    #[inline(always)]
    pub fn type_layout(&self) -> &mut TypeLayout {
        self.var_layout().type_layout()
    }

    #[inline]
    pub fn result_var_layout(&self) -> &mut VariableLayout {
        unsafe {
            VariableLayout::from_mut_ptr(ffi::spReflectionEntryPoint_getResultVarLayout(
                self.0.get(),
            ))
        }
    }

    #[inline]
    pub fn has_default_constant_buffer(&self) -> bool {
        unsafe { ffi::spReflectionEntryPoint_hasDefaultConstantBuffer(self.0.get()) != 0 }
    }
}

#[repr(transparent)]
pub struct TypeParameter(UnsafeCell<ffi::SlangReflectionTypeParameter>);
impl TypeParameter {
    pub const unsafe fn from_mut_ptr<'a>(
        ptr: *mut ffi::SlangReflectionTypeParameter,
    ) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionTypeParameter_GetName(self.0.get())) }
    }

    #[inline]
    pub fn index(&self) -> c_uint {
        unsafe { ffi::spReflectionTypeParameter_GetIndex(self.0.get()) }
    }

    #[inline]
    pub fn constraint_count(&self) -> c_uint {
        unsafe { ffi::spReflectionTypeParameter_GetConstraintCount(self.0.get()) }
    }

    pub fn constraint(&self, index: c_int) -> Option<&mut Type> {
        let p = unsafe { ffi::spReflectionTypeParameter_GetConstraintByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }
}

#[repr(transparent)]
pub struct Shader(UnsafeCell<ffi::SlangReflection>);
impl Shader {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflection) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflection_GetParameterCount(self.0.get()) }
    }

    #[inline]
    pub fn type_parameter_count(&self) -> c_uint {
        unsafe { ffi::spReflection_GetTypeParameterCount(self.0.get()) }
    }

    #[inline]
    pub fn session(&self) -> crate::ISessionPtr {
        unsafe {
            crate::ISessionPtr(NonNull::new_unchecked(ffi::spReflection_GetSession(
                self.0.get(),
            )))
        }
    }

    pub fn type_parameter(&self, index: c_uint) -> Option<&mut TypeParameter> {
        let p = unsafe { ffi::spReflection_GetTypeParameterByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeParameter::from_mut_ptr(p) })
        }
    }

    pub fn find_type_parameter(&self, name: &CStr) -> Option<&mut TypeParameter> {
        let p = unsafe { ffi::spReflection_FindTypeParameter(self.0.get(), name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeParameter::from_mut_ptr(p) })
        }
    }

    pub fn parameter(&self, index: c_uint) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflection_GetParameterByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn entry_point_count(&self) -> SlangUInt {
        unsafe { ffi::spReflection_getEntryPointCount(self.0.get()) }
    }

    pub fn entry_point(&self, index: SlangUInt) -> Option<&mut EntryPoint> {
        let p = unsafe { ffi::spReflection_getEntryPointByIndex(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { EntryPoint::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn global_constant_buffer_binding(&self) -> SlangUInt {
        unsafe { ffi::spReflection_getGlobalConstantBufferBinding(self.0.get()) }
    }

    #[inline]
    pub fn global_constant_buffer_size(&self) -> usize {
        unsafe { ffi::spReflection_getGlobalConstantBufferSize(self.0.get()) }
    }

    pub fn find_type_by_name(&self, name: &CStr) -> Option<&mut Type> {
        let p = unsafe { ffi::spReflection_FindTypeByName(self.0.get(), name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }

    pub fn find_function_by_name(&self, name: &CStr) -> Option<&mut Function> {
        let p = unsafe { ffi::spReflection_FindFunctionByName(self.0.get(), name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Function::from_mut_ptr(p) })
        }
    }

    pub fn find_function_by_name_in_type(
        &self,
        r#type: &Type,
        name: &CStr,
    ) -> Option<&mut Function> {
        let p = unsafe {
            ffi::spReflection_FindFunctionByNameInType(self.0.get(), r#type.0.get(), name.as_ptr())
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Function::from_mut_ptr(p) })
        }
    }

    pub fn find_var_by_name_in_type(&self, r#type: &Type, name: &CStr) -> Option<&mut Variable> {
        let p = unsafe {
            ffi::spReflection_FindVarByNameInType(self.0.get(), r#type.0.get(), name.as_ptr())
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    pub fn type_layout(&self, r#type: &Type, rules: LayoutRules) -> Option<&mut TypeLayout> {
        let p = unsafe { ffi::spReflection_GetTypeLayout(self.0.get(), r#type.0.get(), rules) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeLayout::from_mut_ptr(p) })
        }
    }

    pub fn find_entry_point_by_name(&self, name: &CStr) -> Option<&mut EntryPoint> {
        let p = unsafe { ffi::spReflection_findEntryPointByName(self.0.get(), name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { EntryPoint::from_mut_ptr(p) })
        }
    }

    pub fn specialize_type(
        &self,
        r#type: &Type,
        specialization_args: &[&Type],
        out_diagnostics: Option<&mut MaybeUninit<Option<crate::IBlobPtr>>>,
    ) -> Option<&mut Type> {
        let p = unsafe {
            ffi::spReflection_specializeType(
                self.0.get(),
                r#type.0.get(),
                specialization_args.len() as _,
                specialization_args.as_ptr() as _,
                out_diagnostics.map_or_else(core::ptr::null_mut, MaybeUninit::as_mut_ptr) as _,
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }

    pub fn specialize_generic(
        &self,
        generic: &Generic,
        specialization_arg_types: &[GenericArgType],
        specialization_arg_vals: &[GenericArg],
        out_diagnostics: Option<&mut MaybeUninit<Option<crate::IBlobPtr>>>,
    ) -> Option<&mut Generic> {
        assert_eq!(
            specialization_arg_types.len(),
            specialization_arg_vals.len()
        );

        let p = unsafe {
            ffi::spReflection_specializeGeneric(
                self.0.get(),
                generic.0.get(),
                specialization_arg_types.len() as _,
                specialization_arg_types.as_ptr(),
                specialization_arg_vals.as_ptr(),
                out_diagnostics.map_or_else(core::ptr::null_mut, MaybeUninit::as_mut_ptr) as _,
            )
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Generic::from_mut_ptr(p) })
        }
    }

    #[inline]
    pub fn is_sub_type(&self, sub_type: &Type, super_type: &Type) -> bool {
        unsafe { ffi::spReflection_isSubType(self.0.get(), sub_type.0.get(), super_type.0.get()) }
    }

    #[inline]
    pub fn hashed_string_count(&self) -> SlangUInt {
        unsafe { ffi::spReflection_getHashedStringCount(self.0.get()) }
    }

    pub fn hashed_string(
        &self,
        index: SlangUInt,
        out_count: &mut MaybeUninit<usize>,
    ) -> Option<&CStr> {
        let p = unsafe {
            ffi::spReflection_getHashedString(self.0.get(), index, out_count.as_mut_ptr())
        };
        if p.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(p) })
        }
    }

    pub fn global_params_type_layout(&self) -> Option<&mut TypeLayout> {
        let p = unsafe { ffi::spReflection_getGlobalParamsTypeLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { TypeLayout::from_mut_ptr(p) })
        }
    }

    pub fn global_params_var_layout(&self) -> Option<&mut VariableLayout> {
        let p = unsafe { ffi::spReflection_getGlobalParamsVarLayout(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { VariableLayout::from_mut_ptr(p) })
        }
    }

    pub fn to_json(&self) -> crate::Result<crate::IBlobPtr> {
        let mut o = MaybeUninit::uninit();
        crate::rw(unsafe {
            ffi::spReflection_ToJson(self.0.get(), core::ptr::null_mut(), o.as_mut_ptr())
        })?;

        Ok(crate::IBlobPtr(unsafe {
            NonNull::new_unchecked(o.assume_init())
        }))
    }
}

#[repr(transparent)]
pub struct Decl(UnsafeCell<ffi::SlangReflectionDecl>);
impl Decl {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionDecl) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::spReflectionDecl_getName(self.0.get())) }
    }

    #[inline]
    pub fn kind(&self) -> crate::DeclKind {
        unsafe { ffi::spReflectionDecl_getKind(self.0.get()) }
    }

    #[inline]
    pub fn children_count(&self) -> c_uint {
        unsafe { ffi::spReflectionDecl_getChildrenCount(self.0.get()) }
    }

    pub fn child(&self, index: c_uint) -> Option<&mut Decl> {
        let p = unsafe { ffi::spReflectionDecl_getChild(self.0.get(), index) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Decl::from_mut_ptr(p) })
        }
    }

    pub fn r#type(&self) -> Option<&mut Type> {
        let p = unsafe { ffi::spReflection_getTypeFromDecl(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Type::from_mut_ptr(p) })
        }
    }

    pub fn as_variable(&self) -> Option<&mut Variable> {
        let p = unsafe { ffi::spReflectionDecl_castToVariable(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Variable::from_mut_ptr(p) })
        }
    }

    pub fn as_function(&self) -> Option<&mut Function> {
        let p = unsafe { ffi::spReflectionDecl_castToFunction(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Function::from_mut_ptr(p) })
        }
    }

    pub fn as_generic(&self) -> Option<&mut Generic> {
        let p = unsafe { ffi::spReflectionDecl_castToGeneric(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Generic::from_mut_ptr(p) })
        }
    }

    pub fn parent(&self) -> Option<&mut Decl> {
        let p = unsafe { ffi::spReflectionDecl_getParent(self.0.get()) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { Decl::from_mut_ptr(p) })
        }
    }
}

#[repr(transparent)]
pub struct Modifier(UnsafeCell<ffi::SlangReflectionModifier>);
impl Modifier {
    pub const unsafe fn from_mut_ptr<'a>(ptr: *mut ffi::SlangReflectionModifier) -> &'a mut Self {
        unsafe { core::mem::transmute(UnsafeCell::from_mut(&mut *ptr)) }
    }
}

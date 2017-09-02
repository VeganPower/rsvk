#![allow(bad_style)]
#![allow(dead_code)]

extern crate winit;
extern crate libloading;

mod vulkan
{
use std::os::raw::c_void;
use std::os::raw::c_char;
use std;

type VkFlags = u32;
type VkInstanceCreateFlags = VkFlags;
type bool32 = u32;
type VkDeviceSize = u64;

pub fn vk_make_version(major : u32, minor : u32, patch : u32) -> u32
{
   (major << 22) | (minor << 12) | patch
}

#[repr(i32)]
pub enum VkResult {
    VkSuccess = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_EVENT_SET = 3,
    VK_EVENT_RESET = 4,
    VK_INCOMPLETE = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    VK_ERROR_FRAGMENTED_POOL = -12,
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
    VK_SUBOPTIMAL_KHR = 1000001003,
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
    VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
    VK_ERROR_INVALID_SHADER_NV = -1000012000,
    VK_ERROR_OUT_OF_POOL_MEMORY_KHR = -1000069000,
    VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR = -1000072003,
}

/*#[repr(C)]
pub struct VkInstance
{
   ptr: *mut u8// = std::ptr::null_mut()
}

impl VkInstance
{
   pub fn new() -> VkInstance
   {
      VkInstance { ptr : std::ptr::null_mut() }
   }
   pub fn print(&self)
   {
      println!("Vk instance {:X}", self.ptr as u64);
   }
}*/
pub type VkInstance = * mut u8;

#[repr(C)]
pub struct VkPhysicalDevice
{
   ptr: *mut u8// = std::ptr::null_mut()
}

impl VkPhysicalDevice
{
   pub fn new() -> VkPhysicalDevice
   {
      VkPhysicalDevice { ptr : std::ptr::null_mut() }
   }
   pub fn print(self)
   {
      println!("Vk physical device {:X}", self.ptr as u64);
   }
}
/*
impl Default for VkInstance {
   fn default() -> VkInstance {
      VkInstance {
         ptr : 
      }
   }
}
*/

#[repr(u32)]
pub enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
    VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,
    VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,
    VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
    VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,
    VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR = 1000003000,
    VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,
    VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,
    VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,
    VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,
    VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,
    VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,
    VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,
    VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000,
    VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001,
    VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002,
    VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD = 1000041000,
    VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX = 1000053000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX = 1000053001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX = 1000053002,
    VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000,
    VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001,
    VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000,
    VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001,
    VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR = 1000059000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR = 1000059001,
    VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR = 1000059002,
    VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR = 1000059004,
    VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR = 1000059005,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR = 1000059006,
    VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059007,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR = 1000059008,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX = 1000060000,
    VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX = 1000060001,
    VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX = 1000060002,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX = 1000060003,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX = 1000060004,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX = 1000060005,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX = 1000060006,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX = 1000060007,
    VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX = 1000060008,
    VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX = 1000060009,
    VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX = 1000060010,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX = 1000060011,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX = 1000060012,
    VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT = 1000061000,
    VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN = 1000062000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX = 1000070000,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX = 1000070001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR = 1000071000,
    VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR = 1000071001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR = 1000071002,
    VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR = 1000071003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR = 1000071004,
    VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR = 1000072000,
    VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR = 1000072001,
    VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR = 1000072002,
    VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073000,
    VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073001,
    VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR = 1000073002,
    VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR = 1000073003,
    VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR = 1000074000,
    VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR = 1000074001,
    VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR = 1000074002,
    VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR = 1000075000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR = 1000076000,
    VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR = 1000076001,
    VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR = 1000077000,
    VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078000,
    VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078001,
    VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR = 1000078002,
    VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR = 1000078003,
    VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR = 1000079000,
    VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR = 1000079001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR = 1000080000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR = 1000083000,
    VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR = 1000084000,
    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR = 1000085000,
    VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX = 1000086000,
    VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX = 1000086001,
    VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX = 1000086002,
    VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX = 1000086003,
    VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX = 1000086004,
    VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX = 1000086005,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV = 1000087000,
    VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT = 1000090000,
    VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT = 1000091000,
    VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT = 1000091001,
    VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT = 1000091002,
    VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT = 1000091003,
    VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE = 1000092000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX = 1000097000,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV = 1000098000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT = 1000099000,
    VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT = 1000099001,
    VK_STRUCTURE_TYPE_HDR_METADATA_EXT = 1000105000,
    VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR = 1000111000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR = 1000112000,
    VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR = 1000112001,
    VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR = 1000113000,
    VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114000,
    VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114001,
    VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR = 1000114002,
    VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR = 1000115000,
    VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR = 1000115001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR = 1000119000,
    VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR = 1000119001,
    VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR = 1000119002,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR = 1000120000,
    VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK = 1000122000,
    VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK = 1000123000,
    VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR = 1000127000,
    VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR = 1000127001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT = 1000130000,
    VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT = 1000130001,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146000,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146001,
    VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146002,
    VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR = 1000146003,
    VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR = 1000146004,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT = 1000148000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT = 1000148001,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT = 1000148002,
    VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV = 1000149000,
    VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV = 1000152000,
}


#[repr(i32)]
pub enum VkSystemAllocationScope {
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4
}

#[repr(i32)]
pub enum VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0
}

pub type PFN_vkAllocationFunction = unsafe extern "system" fn(*mut c_void,
                                                            usize,
                                                            usize,
                                                            VkSystemAllocationScope)
                                                            -> *mut c_void;

pub type PFN_vkReallocationFunction = unsafe extern "system" fn(*mut c_void,
                                                               *mut c_void,
                                                               usize,
                                                               usize,
                                                               VkSystemAllocationScope)
                                                               -> *mut c_void;

pub type PFN_vkFreeFunction = unsafe extern "system" fn(*mut c_void, *mut c_void);

pub type PFN_vkInternalAllocationNotification =
   unsafe extern "system" fn(*mut c_void,
                              usize,
                              VkInternalAllocationType,
                              VkSystemAllocationScope);

pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(*mut c_void,
                                                                  usize,
                                                                  VkInternalAllocationType,
                                                                  VkSystemAllocationScope);
#[repr(C)]
pub struct VkAllocationCallbacks {
    pUserData : * mut c_void,
    pfnAllocation : PFN_vkAllocationFunction,
    pfnReallocation : PFN_vkReallocationFunction,
    pfnFree : PFN_vkFreeFunction,
    pfnInternalAllocation : PFN_vkInternalAllocationNotification,
    pfnInternalFree : PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub sType : VkStructureType,
    pub pNext : *const c_void,
    pub pApplicationName : *const c_char,
    pub applicationVersion : u32,
    pub pEngineName : *const c_char,
    pub engineVersion : u32,
    pub apiVersion : u32,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType                     : VkStructureType,
    pub pNext                     : *const c_void,
    pub flags                     : VkInstanceCreateFlags,
    pub pApplicationInfo          : *const VkApplicationInfo,
    pub enabledLayerCount         : u32,
    pub ppEnabledLayerNames       : *const *const c_char,
    pub enabledExtensionCount     : u32,
    pub ppEnabledExtensionNames   : *const *const c_char,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures
{
    robustBufferAccess                     : i32,
    fullDrawIndexUint32                    : i32,
    imageCubeArray                         : i32,
    independentBlend                       : i32,
    geometryShader                         : i32,
    tessellationShader                     : i32,
    sampleRateShading                      : i32,
    dualSrcBlend                           : i32,
    logicOp                                : i32,
    multiDrawIndirect                      : i32,
    drawIndirectFirstInstance              : i32,
    depthClamp                             : i32,
    depthBiasClamp                         : i32,
    fillModeNonSolid                       : i32,
    depthBounds                            : i32,
    wideLines                              : i32,
    largePoints                            : i32,
    alphaToOne                             : i32,
    multiViewport                          : i32,
    samplerAnisotropy                      : i32,
    textureCompressionETC2                 : i32,
    textureCompressionASTC_LDR             : i32,
    textureCompressionBC                   : i32,
    occlusionQueryPrecise                  : i32,
    pipelineStatisticsQuery                : i32,
    vertexPipelineStoresAndAtomics         : i32,
    fragmentStoresAndAtomics               : i32,
    shaderTessellationAndGeometryPointSize : i32,
    shaderImageGatherExtended              : i32,
    shaderStorageImageExtendedFormats      : i32,
    shaderStorageImageMultisample          : i32,
    shaderStorageImageReadWithoutFormat    : i32,
    shaderStorageImageWriteWithoutFormat   : i32,
    shaderUniformBufferArrayDynamicIndexing: i32,
    shaderSampledImageArrayDynamicIndexing : i32,
    shaderStorageBufferArrayDynamicIndexing: i32,
    shaderStorageImageArrayDynamicIndexing : i32,
    shaderClipDistance                     : i32,
    shaderCullDistance                     : i32,
    shaderFloat64                          : i32,
    shaderInt64                            : i32,
    shaderInt16                            : i32,
    shaderResourceResidency                : i32,
    shaderResourceMinLod                   : i32,
    sparseBinding                          : i32,
    sparseResidencyBuffer                  : i32,
    sparseResidencyImage2D                 : i32,
    sparseResidencyImage3D                 : i32,
    sparseResidency2Samples                : i32,
    sparseResidency4Samples                : i32,
    sparseResidency8Samples                : i32,
    sparseResidency16Samples               : i32,
    sparseResidencyAliased                 : i32,
    variableMultisampleRate                : i32,
    inheritedQueries                       : i32,
}

const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE : usize = 256;
const VK_UUID_SIZE : usize = 16;

#[repr(u32)]
pub enum VkSampleCountFlagBits {
    VK_SAMPLE_COUNT_1_BIT = 0x00000001,
    VK_SAMPLE_COUNT_2_BIT = 0x00000002,
    VK_SAMPLE_COUNT_4_BIT = 0x00000004,
    VK_SAMPLE_COUNT_8_BIT = 0x00000008,
    VK_SAMPLE_COUNT_16_BIT = 0x00000010,
    VK_SAMPLE_COUNT_32_BIT = 0x00000020,
    VK_SAMPLE_COUNT_64_BIT = 0x00000040,
}

type VkSampleCountFlags = u32;

#[repr(u32)]
enum VkPhysicalDeviceType {
    VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
}

#[repr(C)]
pub struct VkPhysicalDeviceLimits
{
    maxImageDimension2D                            : u32,
    maxImageDimension3D                            : u32,
    maxImageDimension1D                            : u32,
    maxImageDimensionCube                          : u32,
    maxImageArrayLayers                            : u32,
    maxTexelBufferElements                         : u32,
    maxUniformBufferRange                          : u32,
    maxStorageBufferRange                          : u32,
    maxPushConstantsSize                           : u32,
    maxMemoryAllocationCount                       : u32,
    maxSamplerAllocationCount                      : u32,
    bufferImageGranularity                         : VkDeviceSize,
    sparseAddressSpaceSize                         : VkDeviceSize,
    maxBoundDescriptorSets                         : u32,
    maxPerStageDescriptorSamplers                  : u32,
    maxPerStageDescriptorUniformBuffers            : u32,
    maxPerStageDescriptorStorageBuffers            : u32,
    maxPerStageDescriptorSampledImages             : u32,
    maxPerStageDescriptorStorageImages             : u32,
    maxPerStageDescriptorInputAttachments          : u32,
    maxPerStageResources                           : u32,
    maxDescriptorSetSamplers                       : u32,
    maxDescriptorSetUniformBuffers                 : u32,
    maxDescriptorSetUniformBuffersDynamic          : u32,
    maxDescriptorSetStorageBuffers                 : u32,
    maxDescriptorSetStorageBuffersDynamic          : u32,
    maxDescriptorSetSampledImages                  : u32,
    maxDescriptorSetStorageImages                  : u32,
    maxDescriptorSetInputAttachments               : u32,
    maxVertexInputAttributes                       : u32,
    maxVertexInputBindings                         : u32,
    maxVertexInputAttributeOffset                  : u32,
    maxVertexInputBindingStride                    : u32,
    maxVertexOutputComponents                      : u32,
    maxTessellationGenerationLevel                 : u32,
    maxTessellationPatchSize                       : u32,
    maxTessellationControlPerVertexInputComponents : u32,
    maxTessellationControlPerVertexOutputComponents: u32,
    maxTessellationControlPerPatchOutputComponents : u32,
    maxTessellationControlTotalOutputComponents    : u32,
    maxTessellationEvaluationInputComponents       : u32,
    maxTessellationEvaluationOutputComponents      : u32,
    maxGeometryShaderInvocations                   : u32,
    maxGeometryInputComponents                     : u32,
    maxGeometryOutputComponents                    : u32,
    maxGeometryOutputVertices                      : u32,
    maxGeometryTotalOutputComponents               : u32,
    maxFragmentInputComponents                     : u32,
    maxFragmentOutputAttachments                   : u32,
    maxFragmentDualSrcAttachments                  : u32,
    maxFragmentCombinedOutputResources             : u32,
    maxComputeSharedMemorySize                     : u32,
    maxComputeWorkGroupCount                       : [u32; 3],
    maxComputeWorkGroupInvocations                 : u32,
    maxComputeWorkGroupSize                        : [u32; 3],
    subPixelPrecisionBits                          : u32,
    subTexelPrecisionBits                          : u32,
    mipmapPrecisionBits                            : u32,
    maxDrawIndexedIndexValue                       : u32,
    maxDrawIndirectCount                           : u32,
    maxSamplerLodBias                              : f32,
    maxSamplerAnisotropy                           : f32,
    maxViewports                                   : u32,
    maxViewportDimensions                          : [u32; 2],
    viewportBoundsRange                            : [f32; 2],
    viewportSubPixelBits                           : u32,
    minMemoryMapAlignment                          : usize,
    minTexelBufferOffsetAlignment                  : VkDeviceSize,
    minUniformBufferOffsetAlignment                : VkDeviceSize,
    minStorageBufferOffsetAlignment                : VkDeviceSize,
    minTexelOffset                                 : i32,
    maxTexelOffset                                 : u32,
    minTexelGatherOffset                           : i32,
    maxTexelGatherOffset                           : u32,
    minInterpolationOffset                         : f32,
    maxInterpolationOffset                         : f32,
    subPixelInterpolationOffsetBits                : u32,
    maxFramebufferWidth                            : u32,
    maxFramebufferHeight                           : u32,
    maxFramebufferLayers                           : u32,
    framebufferColorSampleCounts                   : VkSampleCountFlags,
    framebufferDepthSampleCounts                   : VkSampleCountFlags,
    framebufferStencilSampleCounts                 : VkSampleCountFlags,
    framebufferNoAttachmentsSampleCounts           : VkSampleCountFlags,
    maxColorAttachments                            : u32,
    sampledImageColorSampleCounts                  : VkSampleCountFlags,
    sampledImageIntegerSampleCounts                : VkSampleCountFlags,
    sampledImageDepthSampleCounts                  : VkSampleCountFlags,
    sampledImageStencilSampleCounts                : VkSampleCountFlags,
    storageImageSampleCounts                       : VkSampleCountFlags,
    maxSampleMaskWords                             : u32,
    timestampComputeAndGraphics                    : bool32,
    timestampPeriod                                : f32,
    maxClipDistances                               : u32,
    maxCullDistances                               : u32,
    maxCombinedClipAndCullDistances                : u32,
    discreteQueuePriorities                        : u32,
    pointSizeRange                                 : [f32; 2],
    lineWidthRange                                 : [f32; 2],
    pointSizeGranularity                           : f32,
    lineWidthGranularity                           : f32,
    strictLines                                    : bool32,
    standardSampleLocations                        : bool32,
    optimalBufferCopyOffsetAlignment               : VkDeviceSize,
    optimalBufferCopyRowPitchAlignment             : VkDeviceSize,
    nonCoherentAtomSize                            : VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties
{
   apiVersion       : u32,
   driverVersion    : u32,
   vendorId         : u32,
   deviceId         : u32,
   deviceType       : VkPhysicalDeviceType,
   deviceName       : [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
   pipelineCacheUUID: [u8;     VK_UUID_SIZE],
   limits           : VkPhysicalDeviceLimits,
   sparseProperties : VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties
{
    residencyStandard2DBlockShape           : bool32,
    residencyStandard2DMultisampleBlockShape: bool32,
    residencyStandard3DBlockShape           : bool32,
    residencyAlignedMipSize                 : bool32,
    residencyNonResidentStrict              : bool32,
}

const VK_MAX_EXTENSION_NAME_SIZE : usize = 256;
const VK_MAX_DESCRIPTION_SIZE : usize  = 256;

#[repr(C)]
pub struct VkLayerProperties
{
    layerName : [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    specVersion : u32,
    implementationVersion : u32,
    description : [c_char; VK_MAX_DESCRIPTION_SIZE],
}

/*
extern {
   pub fn vkCreateInstance(pCreateInfo : *const VkInstanceCreateInfo, pAllocator : *const VkAllocationCallbacks, pInstance : * mut VkInstance) -> VkResult;
   pub fn vkDestroyInstance(pInstance : *mut VkInstance, pAllocator : *const VkAllocationCallbacks);
   pub fn vkEnumeratePhysicalDevices(pInstance:*const VkInstance, pPhysicalDeviceCount : *mut u32, pProperties : * mut VkPhysicalDevice) -> VkResult;
}*/

// struct vk_bootstrap
// {
//    pub vkCreateInstance : fn (pCreateInfo : *const VkInstanceCreateInfo, pAllocator : *const VkAllocationCallbacks, pInstance : * mut VkInstance) -> VkResult,
//    pub vkDestroyInstance : fn (pInstance : *mut VkInstance, pAllocator : *const VkAllocationCallbacks),
//    pub vkEnumeratePhysicalDevices : fn (pInstance:*const VkInstance, pPhysicalDeviceCount : *mut u32, pProperties : * mut VkPhysicalDevice) -> VkResult,

// }

// impl vk_bootstrap
// {
//     pub fn load(lib : libloading::Library)
//     {
//         vk_bootstrap
//         {
//             vkCreateInstance           : libloading::Symbol<fn(pCreateInfo : *const VkInstanceCreateInfo, pAllocator : *const VkAllocationCallbacks, pInstance : * mut VkInstance) -> VkResult> = lib.get(b"vkCreateInstance").unwrap(),
//             vkDestroyInstance          : libloading::Symbol<fn(pInstance : *mut VkInstance, pAllocator : *const VkAllocationCallbacks)> = lib.get(b"vkDestroyInstance").unwrap(),
//             vkEnumeratePhysicalDevices : libloading::Symbol<fn(pInstance:*const VkInstance, pPhysicalDeviceCount : *mut u32, pProperties : * mut VkPhysicalDevice) -> VkResult> = lib.get(b"vkEnumeratePhysicalDevices").unwrap(),
//         }
//     }
// }

}

fn resize_callback(width: u32, height: u32) {
    println!("Window resized to {}x{}", width, height);
}

fn main()
{
   // let window_width = 1920;
   // let window_height = 1080;

   let app_name = std::ffi::CString::new("VulkanTriangle").unwrap();
   let engine_name = std::ffi::CString::new("Obsidian").unwrap();

   let vk_lib = libloading::Library::new("libvulkan.so").unwrap();
   //let vk_boot = vulkan::vk_bootstrap::load(vk_lib);

   let app_info = vulkan::VkApplicationInfo {
                sType: vulkan::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: std::ptr::null(),
                // flags : 0, 
                pApplicationName: app_name.as_ptr(),
                applicationVersion: 1,
                pEngineName: engine_name.as_ptr(),
                engineVersion: 0,
                apiVersion: vulkan::vk_make_version(1, 0, 0),
            };
   let core_validation = std::ffi::CString::new("VK_LAYER_LUNARG_core_validation").unwrap();
   let layer_names = vec![core_validation.as_ptr()];
   let mut layer_count : u32 = 0;
   
   unsafe
   {
   
   let vkEnumerateInstanceLayerProperties : libloading::Symbol<unsafe extern fn(*mut u32, *mut vulkan::VkLayerProperties) -> vulkan::VkResult> = vk_lib.get(b"vkEnumerateInstanceLayerProperties").unwrap();
   vkEnumerateInstanceLayerProperties(&mut layer_count, std::ptr::null_mut());
   println!("layer_count is {}", layer_count as i32);
   
   }
   let create_info = vulkan::VkInstanceCreateInfo {
            sType: vulkan::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0, //Default::default(),
            pApplicationInfo: &app_info,
            ppEnabledLayerNames: layer_names.as_ptr(),//std::ptr::null(),
            enabledLayerCount: layer_names.len() as u32,
            ppEnabledExtensionNames: std::ptr::null(),
            enabledExtensionCount: 0
      };

   unsafe {
   let mut instance = std::ptr::null_mut();//vulkan::VkInstance::new();//{ ..Default::default()};

   
   let vkCreateInstance : libloading::Symbol<unsafe extern fn(pCreateInfo : *const vulkan::VkInstanceCreateInfo, pAllocator : *const vulkan::VkAllocationCallbacks, pInstance : * mut vulkan::VkInstance) -> vulkan::VkResult> = vk_lib.get(b"vkCreateInstance").unwrap();
   println!("Result is {:?}", vkCreateInstance);
   let mut result = vkCreateInstance(&create_info, std::ptr::null(), &mut instance);
   //let instance: Instance<V1_0> = entry.create_instance(&create_info, None).expect("Instance creation error");
   println!("Result is {}", result as i32);
      //instance.print();

   let mut gpu_count : u32 = 0;
   let vkEnumeratePhysicalDevices : libloading::Symbol<fn(pInstance:*const vulkan::VkInstance, pPhysicalDeviceCount : *mut u32, pProperties : * mut vulkan::VkPhysicalDevice) -> vulkan::VkResult> = vk_lib.get(b"vkEnumeratePhysicalDevices").unwrap();
   result = vkEnumeratePhysicalDevices(&instance, & mut gpu_count, std::ptr::null_mut());
   match result {
       vulkan::VkResult::VkSuccess => println!("Device count is {}", gpu_count),
       _ => println!("vkEnumeratePhysicalDevices failed: {}", result as i32),
   }
   //pProperties : * mut VkPhysicalDevice                  

   //vulkan::vkDestroyInstance(&mut instance, std::ptr::null());
   }
   let mut events_loop = winit::EventsLoop::new();
   let window = winit::Window::new(&events_loop).unwrap();

   events_loop.run_forever(|event| {
      match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
      }
   });
}
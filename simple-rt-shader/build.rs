fn main() {
    spirv_builder::SpirvBuilder::new("../shader", "spirv-unknown-vulkan1.2")
        .capability(spirv_builder::Capability::RayTracingKHR)
        .capability(spirv_builder::Capability::ImageQuery)
        .capability(spirv_builder::Capability::StorageImageWriteWithoutFormat)
        .extension("SPV_KHR_ray_tracing")
        .build()
        .unwrap();
}

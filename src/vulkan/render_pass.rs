use ash::{version::DeviceV1_0, vk};

use super::{device::VkDevice, swap_chain::VkSwapChain};

pub struct VkRenderPass {
    pub handle: vk::RenderPass,
}

impl VkRenderPass {
    pub fn new(device: &VkDevice, swap_chain: &VkSwapChain) -> VkRenderPass {
        log::info!("Creating render pass");

        let color_attachment_desc = vk::AttachmentDescription::builder()
            .format(swap_chain.format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();

        let attachment_descs = [color_attachment_desc];

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let color_attachment_refs = [color_attachment_ref];

        let subpass_desc = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_refs)
            .build();
        let subpass_descs = [subpass_desc];

        let subpass_dep = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .build();
        let subpass_deps = [subpass_dep];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachment_descs)
            .subpasses(&subpass_descs)
            .dependencies(&subpass_deps)
            .build();

        let handle = unsafe {
            device
                .create_render_pass(&render_pass_info, None)
                .expect("Unable to create render pass")
        };

        VkRenderPass { handle }
    }

    pub fn cleanup(&self, device: &VkDevice) {
        log::debug!("Dropping render pass");
        unsafe {
            device.handle.destroy_render_pass(self.handle, None);
        }
    }
}

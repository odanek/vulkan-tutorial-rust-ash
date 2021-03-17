use std::sync::Arc;

use ash::{version::DeviceV1_0, vk};

use super::device::VkDevice;

pub struct VkRenderPass {
    device: Arc<VkDevice>,
    pub handle: vk::RenderPass,
}

impl VkRenderPass {
    pub fn new(
        device: &Arc<VkDevice>,
        format: vk::Format,
        depth_format: vk::Format,
        msaa_samples: vk::SampleCountFlags,
    ) -> VkRenderPass {
        let color_attachment_desc = vk::AttachmentDescription::builder()
            .format(format)
            .samples(msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL) // For MSAA
            .build();

        let depth_attachment_desc = vk::AttachmentDescription::builder()
            .format(depth_format)
            .samples(msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();

        let color_attachment_resolve = vk::AttachmentDescription::builder()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::DONT_CARE)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build();

        let attachment_descs = [
            color_attachment_desc,
            depth_attachment_desc,
            color_attachment_resolve,
        ];

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();

        let depth_attachment_ref = vk::AttachmentReference::builder()
            .attachment(1)
            .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();

        let color_attachment_resolve_ref = vk::AttachmentReference::builder()
            .attachment(2)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();

        let color_attachment_refs = [color_attachment_ref];

        let subpass_desc = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_refs)
            .depth_stencil_attachment(&depth_attachment_ref)
            .resolve_attachments(&[color_attachment_resolve_ref])
            .build();
        let subpass_descs = [subpass_desc];

        let subpass_dep = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            )
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            )
            .dst_access_mask(
                vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .build();
        let subpass_deps = [subpass_dep];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachment_descs)
            .subpasses(&subpass_descs)
            .dependencies(&subpass_deps)
            .build();

        let handle = unsafe {
            device
                .handle
                .create_render_pass(&render_pass_info, None)
                .expect("Unable to create render pass")
        };

        VkRenderPass {
            device: Arc::clone(device),
            handle,
        }
    }
}

impl Drop for VkRenderPass {
    fn drop(&mut self) {
        log::debug!("Dropping render pass");
        unsafe {
            self.device.handle.destroy_render_pass(self.handle, None);
        }
    }
}

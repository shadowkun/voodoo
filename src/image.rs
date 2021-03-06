use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VdResult, Device, DeviceMemory, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ImageHandle(pub(crate) vks::VkImage);

impl ImageHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkImage {
        self.0
    }
}

unsafe impl Handle for ImageHandle {
    type Target = ImageHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: ImageHandle,
    memory_requirements: ::MemoryRequirements,
    device: Device,
    is_swapchain_image: bool,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            if !self.is_swapchain_image {
                self.device.destroy_image(self.handle, None);
            }
        }
    }
}


/// An image.
///
///
/// ### Destruction
/// 
/// Dropping this `Image` will cause `Device::destroy_image` to be called, 
/// automatically releasing any resources associated with it.
///
#[derive(Debug, Clone)]
pub struct Image {
    inner: Arc<Inner>,
}

impl Image {
    /// Returns a new `ImageBuilder`.
    pub fn builder<'b>() -> ImageBuilder<'b> {
        ImageBuilder::new()
    }

    pub(crate) unsafe fn from_handle(device: Device, handle: ImageHandle, is_swapchain_image: bool) -> Image {
        let memory_requirements = device.get_image_memory_requirements(handle);

        Image {
            inner: Arc::new(Inner {
                handle,
                memory_requirements: memory_requirements.into(),
                device,
                is_swapchain_image,
            })
        }
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> ImageHandle {
        self.inner.handle
    }

    /// Returns this image's memory requirements.
    pub fn memory_requirements(&self) -> &::MemoryRequirements {
        &self.inner.memory_requirements
    }

    /// Binds this image to device memory. `offset` is the start offset of the
    /// region of memory which is to be bound. The number of bytes returned in
    /// the VkMemoryRequirements::size member in memory, starting from
    /// memoryOffset bytes, will be bound to the specified image.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the bound memory is not in use when it is
    /// dropped.
    ///
    pub unsafe fn bind_memory(&self, memory: &DeviceMemory, offset_bytes: ::DeviceSize)
            -> VdResult<()> {
        self.inner.device.bind_image_memory(self.inner.handle, memory.handle(), offset_bytes)
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'i> Handle for &'i Image {
    type Target = ImageHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}


/// A builder for `Image`.
#[derive(Debug, Clone)]
pub struct ImageBuilder<'b> {
    create_info: ::ImageCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> ImageBuilder<'b> {
        ImageBuilder {
            create_info: ::ImageCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// flags is a bitmask of VkImageCreateFlagBits describing additional
    /// parameters of the image.
    pub fn flags<'s>(&'s mut self, flags: ::ImageCreateFlags)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// imageType is a VkImageType value specifying the basic dimensionality
    /// of the image. Layers in array textures do not count as a dimension for
    /// the purposes of the image type.
    pub fn image_type<'s>(&'s mut self, image_type: ::ImageType)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_image_type(image_type);
        self
    }

    /// format is a VkFormat describing the format and type of the data
    /// elements that will be contained in the image.
    pub fn format<'s>(&'s mut self, format: ::Format)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_format(format);
        self
    }

    /// extent is a VkExtent3D describing the number of data elements in each
    /// dimension of the base level.
    pub fn extent<'s>(&'s mut self, extent: ::Extent3d)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_extent(extent);
        self
    }

    /// mipLevels describes the number of levels of detail available for
    /// minified sampling of the image.
    pub fn mip_levels<'s>(&'s mut self, mip_levels: u32)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_mip_levels(mip_levels);
        self
    }

    /// arrayLayers is the number of layers in the image.
    pub fn array_layers<'s>(&'s mut self, array_layers: u32)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_array_layers(array_layers);
        self
    }

    /// samples is the number of sub-data element samples in the image as
    /// defined in VkSampleCountFlagBits. See Multisampling.
    pub fn samples<'s>(&'s mut self, samples: ::SampleCountFlags)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_samples(samples);
        self
    }

    /// tiling is a VkImageTiling value specifying the tiling arrangement of
    /// the data elements in memory.
    pub fn tiling<'s>(&'s mut self, tiling: ::ImageTiling)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_tiling(tiling);
        self
    }

    /// usage is a bitmask of VkImageUsageFlagBits describing the intended
    /// usage of the image.
    pub fn usage<'s>(&'s mut self, usage: ::ImageUsageFlags)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_usage(usage);
        self
    }

    /// sharingMode is a VkSharingMode value specifying the sharing mode of
    /// the image when it will be accessed by multiple queue families.
    pub fn sharing_mode<'s>(&'s mut self, sharing_mode: ::SharingMode)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_sharing_mode(sharing_mode);
        self
    }

    /// queueFamilyIndexCount is the number of entries in the
    /// pQueueFamilyIndices array.
    /// pQueueFamilyIndices is a list of queue families that will access this
    /// image (ignored if sharingMode is not VK_SHARING_MODE_CONCURRENT).
    pub fn queue_family_indices<'s, 'p>(&'s mut self,
            queue_family_indices: &'p [u32])
            -> &'s mut ImageBuilder<'b>
            where 'p: 'b {
        self.create_info.set_queue_family_indices(queue_family_indices);
        self
    }

    /// initialLayout is a VkImageLayout value specifying the initial
    /// VkImageLayout of all image subresources of the image. See Image
    /// Layouts.
    pub fn initial_layout<'s>(&'s mut self, initial_layout: ::ImageLayout)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.set_initial_layout(initial_layout);
        self
    }

    //// Creates and returns a new `Image`
    pub fn build(&self, device: Device) -> VdResult<Image> {
        unsafe {
            let handle = device.create_image(&self.create_info, None)?;
            Ok(Image::from_handle(device, handle, false))
        }
    }
}
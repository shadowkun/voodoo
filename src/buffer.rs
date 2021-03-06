use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VdResult, Device, DeviceMemory, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct BufferHandle(pub(crate) vks::VkBuffer);

impl BufferHandle {
    pub fn to_raw(&self) -> vks::VkBuffer {
        self.0
    }
}

unsafe impl Handle for BufferHandle {
    type Target = BufferHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: BufferHandle,
    memory_requirements: ::MemoryRequirements,
    device: Device,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_buffer(self.handle, None);
        }
    }
}


/// A buffer.
///
///
/// ### Destruction
/// 
/// Dropping this `Buffer` will cause `Device::destroy_buffer` to be called, 
/// automatically releasing any resources associated with it.
///
#[derive(Debug, Clone)]
pub struct Buffer {
    inner: Arc<Inner>,
}

impl Buffer {
    /// Returns a new `BufferBuilder`.
    pub fn builder<'b>() -> BufferBuilder<'b> {
        BufferBuilder::new()
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> BufferHandle {
        self.inner.handle
    }

    /// Returns this buffer's memory requirements.
    pub fn memory_requirements(&self) -> &::MemoryRequirements {
        &self.inner.memory_requirements
    }

    /// Binds this buffer to device memory. `offset` is the start offset of the
    /// region of memory which is to be bound. The number of bytes returned in
    /// the VkMemoryRequirements::size member in memory, starting from
    /// memoryOffset bytes, will be bound to the specified buffer.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the bound memory is not in use when it is
    /// dropped.
    ///
    pub unsafe fn bind_memory(&self, memory: &DeviceMemory, offset: ::DeviceSize)
            -> VdResult<()> {
        self.inner.device.bind_buffer_memory(self.inner.handle, memory.handle(), offset)
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'b> Handle for &'b Buffer {
    type Target = BufferHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}


/// A builder for `Buffer`.
#[derive(Debug, Clone)]
pub struct BufferBuilder<'b> {
    create_info: ::BufferCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> BufferBuilder<'b> {
        BufferBuilder {
            create_info: ::BufferCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies additional parameters of the buffer.
    pub fn flags<'s>(&'s mut self, flags: ::BufferCreateFlags)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies the size in bytes of the buffer to be created.
    pub fn size<'s>(&'s mut self, size: ::DeviceSize)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.set_size(size);
        self
    }

    /// Specifies allowed usages of the buffer.
    pub fn usage<'s>(&'s mut self, usage: ::BufferUsageFlags)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.set_usage(usage);
        self
    }

    /// Specifies the sharing mode of the buffer when it will be accessed by
    /// multiple queue families.
    pub fn sharing_mode<'s>(&'s mut self, sharing_mode: ::SharingMode)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.set_sharing_mode(sharing_mode);
        self
    }

    /// Specifies a list of queue families that will access this buffer
    /// (ignored if sharing_mode is not VK_SHARING_MODE_CONCURRENT).
    pub fn queue_family_indices<'s, 'p>(&'s mut self, queue_family_indices: &'p [u32])
            -> &'s mut BufferBuilder<'b>
            where 'p: 'b {
        self.create_info.set_queue_family_indices(queue_family_indices);
        self
    }

    /// Creates and returns a new `Buffer`
    pub fn build(&self, device: Device) -> VdResult<Buffer> {
        let handle = unsafe { device.create_buffer(&self.create_info, None)? };
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(handle) };

        Ok(Buffer {
            inner: Arc::new(Inner {
                handle,
                device,
                memory_requirements,
            })
        })
    }
}

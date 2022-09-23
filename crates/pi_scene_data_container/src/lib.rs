use bytemuck::Pod;
use pi_slotmap::DefaultKey;
use wgpu::util::DeviceExt;


pub trait FKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl FKey for DefaultKey {}
impl FKey for u8 {}
impl FKey for u16 {}
impl FKey for u32 {}
impl FKey for u64 {}
impl FKey for u128 {}
impl FKey for usize {}
impl FKey for i8 {}
impl FKey for i16 {}
impl FKey for i32 {}
impl FKey for i64 {}
impl FKey for i128 {}
impl FKey for isize {}
impl FKey for &str {}

pub trait FContainer<T: Clone, K: FKey> {
    fn insert(&mut self, value: T) -> K;
    fn remove(&mut self, key: &K) -> Option<T>;
    fn get(&mut self, key: &K) -> Option<&T>;
    fn get_mut(&mut self, key: &K) -> Option<&mut T>;
}

pub trait TextureID: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TextureID for DefaultKey {}
impl TextureID for u8 {}
impl TextureID for u16 {}
impl TextureID for u32 {}
impl TextureID for u64 {}
impl TextureID for usize {}
impl TextureID for u128 {}
impl TextureID for i8 {}
impl TextureID for i16 {}
impl TextureID for i32 {}
impl TextureID for i64 {}
impl TextureID for i128 {}
impl TextureID for isize {}
impl TextureID for &str {}

pub trait TexturePool<TID: TextureID> {
    fn get(&self, key: TID) -> Option<& wgpu::TextureView>;
}

pub trait TMaterialKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TMaterialKey for DefaultKey {}
impl TMaterialKey for u8 {}
impl TMaterialKey for u16 {}
impl TMaterialKey for u32 {}
impl TMaterialKey for u64 {}
impl TMaterialKey for u128 {}
impl TMaterialKey for usize {}
impl TMaterialKey for i8 {}
impl TMaterialKey for i16 {}
impl TMaterialKey for i32 {}
impl TMaterialKey for i64 {}
impl TMaterialKey for i128 {}
impl TMaterialKey for isize {}
impl TMaterialKey for &str {}

pub trait TMaterialBlockKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TMaterialBlockKindKey for DefaultKey {}
impl TMaterialBlockKindKey for u8 {}
impl TMaterialBlockKindKey for u16 {}
impl TMaterialBlockKindKey for u32 {}
impl TMaterialBlockKindKey for u64 {}
impl TMaterialBlockKindKey for u128 {}
impl TMaterialBlockKindKey for usize {}
impl TMaterialBlockKindKey for i8 {}
impl TMaterialBlockKindKey for i16 {}
impl TMaterialBlockKindKey for i32 {}
impl TMaterialBlockKindKey for i64 {}
impl TMaterialBlockKindKey for i128 {}
impl TMaterialBlockKindKey for isize {}
impl TMaterialBlockKindKey for &str {}

pub trait TVertexDataKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TVertexDataKindKey for DefaultKey {}
impl TVertexDataKindKey for u8 {}
impl TVertexDataKindKey for u16 {}
impl TVertexDataKindKey for u32 {}
impl TVertexDataKindKey for u64 {}
impl TVertexDataKindKey for usize {}
impl TVertexDataKindKey for u128 {}
impl TVertexDataKindKey for i8 {}
impl TVertexDataKindKey for i16 {}
impl TVertexDataKindKey for i32 {}
impl TVertexDataKindKey for i64 {}
impl TVertexDataKindKey for i128 {}
impl TVertexDataKindKey for isize {}
impl TVertexDataKindKey for &str {}

pub trait TVertexBufferKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TVertexBufferKindKey for DefaultKey {}
impl TVertexBufferKindKey for u8 {}
impl TVertexBufferKindKey for u16 {}
impl TVertexBufferKindKey for u32 {}
impl TVertexBufferKindKey for u64 {}
impl TVertexBufferKindKey for usize {}
impl TVertexBufferKindKey for u128 {}
impl TVertexBufferKindKey for i8 {}
impl TVertexBufferKindKey for i16 {}
impl TVertexBufferKindKey for i32 {}
impl TVertexBufferKindKey for i64 {}
impl TVertexBufferKindKey for i128 {}
impl TVertexBufferKindKey for isize {}
impl TVertexBufferKindKey for &str {}

pub trait TGeometryBufferID: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash {}
impl TGeometryBufferID for DefaultKey {}
impl TGeometryBufferID for u8 {}
impl TGeometryBufferID for u16 {}
impl TGeometryBufferID for u32 {}
impl TGeometryBufferID for u64 {}
impl TGeometryBufferID for usize {}
impl TGeometryBufferID for u128 {}
impl TGeometryBufferID for i8 {}
impl TGeometryBufferID for i16 {}
impl TGeometryBufferID for i32 {}
impl TGeometryBufferID for i64 {}
impl TGeometryBufferID for i128 {}
impl TGeometryBufferID for isize {}
impl TGeometryBufferID for &str {}

pub trait GeometryBufferPool<GBID: TGeometryBufferID> {
    fn insert<T: Clone + Copy + Pod>(&self, data: GeometryBuffer<T>) -> GBID;
    fn remove<T: Clone + Copy + Pod>(&self, key: &GBID) -> Option<GeometryBuffer<T>>;
    fn get<T: Clone + Copy + Pod>(&self, key: &GBID) -> Option<&GeometryBuffer<T>>;
    fn get_size(&self, key: &GBID) -> usize;
    fn get_mut<T: Clone + Copy + Pod>(&self, key: &GBID) -> Option<&mut GeometryBuffer<T>>;
    fn get_buffer(&self, key: &GBID) -> Option<&wgpu::Buffer>;
}

#[derive(Debug)]
pub struct GeometryBuffer<T: Clone + Copy + Pod> {
    dirty: bool,
    updateable: bool,
    data: Vec<T>,
    _size: usize,
    buffer: Option<wgpu::Buffer>,
}
impl<T: Clone + Copy + Pod> GeometryBuffer<T> {
    pub fn new(data: &[T], updateable: bool) -> Self {
        Self {
            dirty: true,
            updateable,
            data: data.to_vec(),
            _size: data.len(),
            buffer: None,
        }
    }
    pub fn reset(&mut self, data: &[T]) -> bool {
        if self.updateable {
            self.data = data.to_vec();
            self._size = self.data.len();
            self.dirty = true;
            true
        } else {
            false
        }
    }
    pub fn update_buffer(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        match self.buffer.as_ref() {
            Some(buffer) => {
                if self.updateable && self.dirty {
                    queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.data));
                }
            },
            None => {
                let usage = if self.updateable {
                    wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::INDEX
                };
                self.buffer = Some(
                    device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(&self.data),
                            usage,
                        }
                    )
                );
                // if !self.updateable {
                //     self.data.clear();
                // }
            },
        }
    }
    pub fn uninstall_buffer(&mut self) {
        self.buffer = None;
    }
    pub fn update(&mut self, data: &[T], offset: usize) -> bool {
        if self.updateable {
            let len = data.len();
            let old_size = self.data.len();
    
            let mut index = 0;
            if offset < old_size {
                for i in offset..old_size {
                    self.data[i] = data[index];
                    index += 1;
                    if len <= index  {
                        break;
                    }
                }
            }
    
            if index < len {
                for i in index..len {
                    self.data.push(data[i]);
                }
            }

            self._size = self.data.len();
            self.dirty = true;
    
            true
        } else {
            false
        }
    }
    pub fn get_buffer(&self) -> Option<&wgpu::Buffer> {
        self.buffer.as_ref()
    }
    pub fn size(&self) -> usize {
        self._size
    }
}

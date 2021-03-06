use crate::codegen::data_analyzer::SurfaceLayout;
use crate::codegen::ObjectCache;
use crate::codegen::TargetProperties;
use crate::mir::{BlockRef, InternalNodeRef, NodeData, SurfaceRef};
use std::os::raw::c_void;
use std::ptr::{null, null_mut};

pub type SurfacePtr = *mut c_void;
pub type NodePtr = *mut c_void;
pub type InitializedPtr = *mut c_void;
pub type BlockPtr = *mut c_void;
pub type ControlValuePtr = *mut c_void;
pub type ControlInitializedPtr = *mut c_void;
pub type ControlDataPtr = *mut c_void;
pub type ControlSharedPtr = *mut c_void;
pub type ControlUiPtr = *mut c_void;

#[repr(C)]
pub struct ControlPointers {
    pub value: ControlValuePtr,
    pub initialized: ControlInitializedPtr,
    pub data: ControlDataPtr,
    pub shared: ControlSharedPtr,
    pub ui: ControlUiPtr,
}

fn get_internal_node_ptr(
    target: &TargetProperties,
    layout: &SurfaceLayout,
    ptr: SurfacePtr,
    node: usize,
) -> NodePtr {
    let ptr_offset = layout.node_ptr_index(node);
    let byte_offset = target
        .machine
        .get_data()
        .offset_of_element(&layout.pointer_struct, ptr_offset as u32)
        .unwrap();
    unsafe { ptr.offset(byte_offset as isize) }
}

pub fn get_node_ptr(
    cache: &ObjectCache,
    surface: SurfaceRef,
    ptr: SurfacePtr,
    node: usize,
) -> NodePtr {
    let surface_mir = cache.surface_mir(surface).unwrap();
    let surface_layout = cache.surface_layout(surface).unwrap();
    match surface_mir.source_map.map_to_internal(node) {
        InternalNodeRef::Direct(node) => {
            get_internal_node_ptr(cache.target(), surface_layout, ptr, node)
        }
        InternalNodeRef::Surface(surface_node, node) => {
            let subsurface_ptr = get_surface_ptr(get_internal_node_ptr(
                cache.target(),
                surface_layout,
                ptr,
                surface_node,
            ));
            let subsurface_ref = match surface_mir.nodes[surface_node].data {
                NodeData::Group(subsurface_ref) => subsurface_ref,
                NodeData::ExtractGroup { surface, .. } => surface,
                _ => panic!("Sourcemap Surface reference points to a non-surface node"),
            };
            get_node_ptr(cache, subsurface_ref, subsurface_ptr, node)
        }
    }
}

pub fn get_node_active_bitmap_ptr(
    cache: &ObjectCache,
    surface: SurfaceRef,
    ptr: SurfacePtr,
    node: usize,
) -> *const u32 {
    let surface_mir = cache.surface_mir(surface).unwrap();
    let surface_layout = cache.surface_layout(surface).unwrap();
    match surface_mir.source_map.map_to_internal(node) {
        InternalNodeRef::Direct(_) => null(),
        InternalNodeRef::Surface(surface_node, _) => {
            let extract_node_ptr =
                get_internal_node_ptr(cache.target(), surface_layout, ptr, surface_node);
            let extract_node_layout = &surface_layout.node_layouts[surface_node];
            let bitmap_ptr_offset = cache
                .target()
                .machine
                .get_data()
                .offset_of_element(&extract_node_layout.pointer_struct, 3)
                .unwrap();
            let bitmap_ptr_ptr =
                unsafe { extract_node_ptr.offset(bitmap_ptr_offset as isize) } as *const *const u32;
            unsafe { *bitmap_ptr_ptr }
        }
    }
}

pub fn get_surface_ptr(ptr: NodePtr) -> SurfacePtr {
    ptr
}

pub fn get_control_ptrs(
    cache: &ObjectCache,
    block: BlockRef,
    ptr: NodePtr,
    control: usize,
) -> ControlPointers {
    let block_layout = cache.block_layout(block).unwrap();
    let control_offset = block_layout.control_index(control);

    // format of the data in the node pointer:
    //   {
    //     initializedData: [
    //        ...,   // one for each control
    //        ...,
    //        ...,
    //     ],
    //     controlData: [
    //        ...,  // one for each control, size may change
    //        ...,
    //        ...,
    //     ]
    //   }
    let machine_data = cache.target().machine.get_data();
    let initialized_offset = machine_data
        .offset_of_element(&block_layout.constant_struct, control_offset as u32)
        .unwrap();
    let control_initialized_ptr = unsafe { ptr.add(initialized_offset as usize) };

    let control_data_offset = machine_data.get_store_size(&block_layout.constant_struct);
    let node_controls_ptr = unsafe { ptr.add(control_data_offset as usize) };
    let controls_data_offset = cache
        .target()
        .machine
        .get_data()
        .offset_of_element(&block_layout.pointer_struct, control_offset as u32)
        .unwrap();
    let base_ptr =
        unsafe { node_controls_ptr.offset(controls_data_offset as isize) } as *mut *mut c_void;
    ControlPointers {
        value: unsafe { *base_ptr },
        data: unsafe { *base_ptr.offset(1) },
        shared: unsafe { *base_ptr.offset(2) },
        ui: if cache.target().include_ui {
            unsafe { *base_ptr.offset(3) }
        } else {
            null_mut()
        },

        initialized: control_initialized_ptr,
    }
}

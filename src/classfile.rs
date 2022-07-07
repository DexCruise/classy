use crate::{AttributeInfo, CpInfo, FieldInfo, MethodInfo};

#[derive(Debug)]
struct ClassFile<'a> {
  magic: u32,
  minor_version: u16,
  major_version: u16,
  constant_pool_count: u16,
  constant_pool: &'a [CpInfo],
  access_flags: u16,
  this_class: u16,
  super_class: u16,
  interfaces_count: u16,
  interfaces: &'a [u16],
  field_info: &'a [FieldInfo],
  methods_count: u16,
  methods: &'a [MethodInfo],
  attributes_count: u16,
  attributes: &'a [AttributeInfo]
}

impl ClassFile<'_> {
  fn new<'a>(
  minor: u16, major: u16, constant_pool: &'a [CpInfo],
  access_flags: u16, this_class: u16, super_class: u16,
  interfaces: &'a [u16], field_info: &'a [FieldInfo],
  methods: &'a [MethodInfo], attributes: &'a [AttributeInfo]
) -> ClassFile<'a> {
    ClassFile {
      magic: 0xCAFEBABE,
      minor_version: minor,
      major_version: major,
      constant_pool_count: constant_pool.len() as u16,
      constant_pool,
      access_flags,
      this_class,
      super_class,
      interfaces_count: interfaces.len() as u16,
      interfaces,
      field_info,
      methods_count: methods.len() as u16,
      methods,
      attributes_count: attributes.len() as u16,
      attributes,
    }
  }
}

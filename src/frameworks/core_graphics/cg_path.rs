/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGPath.h`

use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::core_foundation::cf_string::CFStringRef;
use crate::frameworks::core_foundation::{CFRelease, CFRetain, CFTypeRef};
use crate::frameworks::core_graphics::cg_affine_transform::CGAffineTransform;
use crate::frameworks::foundation::ns_string;
use crate::objc::{msg, objc_classes, ClassExports, HostObject};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// CGPath seems to be a CFType-based type, but in our implementation
// those are just Objective-C types, so we need a class for it, but its name is
// not visible anywhere.
@implementation _touchHLE_CGPath: NSObject
@end

};

pub(super) struct CGPathHostObject {

}
impl HostObject for CGPathHostObject {}

pub type CGPathRef = CFTypeRef;

pub fn CGPathCreateMutable(env: &mut Environment) -> CGPathRef {
    let isa = env
        .objc
        .get_known_class("_touchHLE_CGPath", &mut env.mem);
    env.objc.alloc_object(
        isa,
        Box::new(CGPathHostObject {
            
        }),
        &mut env.mem,
    )
}

pub fn CGPathAddLines(env: &mut Environment, m: CFTypeRef, points: CFTypeRef, count: i32) {
}

pub fn CGPathRelease(env: &mut Environment, path: CGPathRef) {
    if !path.is_null() {
        CFRelease(env, path);
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGPathCreateMutable()),
    export_c_func!(CGPathAddLines(_, _, _)),
    export_c_func!(CGPathRelease(_)),
];
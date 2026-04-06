/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSNumberFormatter`.

use crate::frameworks::core_foundation::time::CFAbsoluteTimeGetGregorianDate;
use crate::frameworks::foundation::ns_value::NSNumberHostObject;
use crate::frameworks::foundation::{ns_string, NSTimeInterval};
use crate::objc::{autorelease, id, msg, nil, objc_classes, ClassExports, HostObject, NSZonePtr};

struct NSNumberFormatterHostObject {
    style: Option<u32>,
}
impl HostObject for NSNumberFormatterHostObject {}

pub type CFNumberFormatterStyle = i32;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleNone: CFNumberFormatterStyle = 0;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleDecimal: CFNumberFormatterStyle = 1;
#[allow(dead_code)]
pub const kCFNumberFormatterStylePercent: CFNumberFormatterStyle = 2;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleScientific: CFNumberFormatterStyle = 3;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleSpellOut: CFNumberFormatterStyle = 4;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleOrdinal: CFNumberFormatterStyle = 5;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleCurrency: CFNumberFormatterStyle = 6;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleCurrencyAccounting: CFNumberFormatterStyle = 7;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleCurrencyISOCode: CFNumberFormatterStyle = 8;
#[allow(dead_code)]
pub const kCFNumberFormatterStyleCurrencyPlural: CFNumberFormatterStyle = 9;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSNumberFormatter: NSObject

+ (id)alloc {
    let host_object = Box::new(NSNumberFormatterHostObject {
        style: None,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())setNumberStyle:(u32)style {
    env.objc.borrow_mut::<NSNumberFormatterHostObject>(this).style = Some(style);
}

- (id)stringFromNumber:(id)number {
    // TODO: respect style
    let number = env.objc.borrow::<NSNumberHostObject>(number);
    let s = format!("{}", number.as_double());
    let res = ns_string::from_rust_string(env, s);
    autorelease(env, res)
}

@end

};

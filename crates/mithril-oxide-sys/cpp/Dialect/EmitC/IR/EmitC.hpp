#pragma once

#include <mlir/Dialect/EmitC/IR/EmitC.h>
#include <mlir/IR/MLIRContext.h>
#include <rust/cxx.h>


namespace mithril_oxide_sys::emitc {

using mlir::emitc::OpaqueType;
using mlir::emitc::PointerType;
using mlir::MLIRContext;


const void *OpaqueType_get(MLIRContext *ctx, rust::Str name);
const void *PointerType_get();

} // mithril_oxide_sys::emitc

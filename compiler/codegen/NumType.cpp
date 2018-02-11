#include "NumType.h"

#include "MaximContext.h"
#include "Num.h"

using namespace MaximCodegen;

NumType::NumType(MaximContext *context) : _context(context) {
    _vecType = llvm::VectorType::get(llvm::Type::getFloatTy(context->llvm()), 2);
    _formType = llvm::Type::getInt8Ty(context->llvm());
    _activeType = llvm::Type::getInt1Ty(context->llvm());
    _type = llvm::StructType::create(context->llvm(), {_vecType, _formType, _activeType}, "struct.num");
}

std::unique_ptr<Value> NumType::createInstance(llvm::Value *val, SourcePos startPos, SourcePos endPos) {
    return Num::create(_context, val, startPos, endPos);
}
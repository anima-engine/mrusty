// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include <stdlib.h>
#include <string.h>

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/class.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/proc.h>
#include <mruby/value.h>
#include <mruby/variable.h>
#include <mruby/throw.h>
#include <mruby/dump.h>

void* mrb_ext_get_ud(struct mrb_state* mrb) {
  return mrb->ud;
}

void mrb_ext_set_ud(struct mrb_state* mrb, void* ud) {
  mrb->ud = ud;
}

mrb_value mrb_ext_load_nstring_cxt_nothrow(mrb_state *mrb, const char *s, size_t len, mrbc_context *cxt) {
  mrb_value value;

  struct mrb_jmpbuf c_jmp;
  struct mrb_jmpbuf *pc_jmp_bak;
  pc_jmp_bak = mrb->jmp;

  MRB_TRY(&c_jmp) {
    mrb->jmp = &c_jmp;
    value = mrb_load_nstring_cxt(mrb, s, len, cxt);
  }
  MRB_CATCH(&c_jmp) {
    value = mrb_nil_value();
  }
  MRB_END_EXC(&c_jmp);

  mrb->jmp = pc_jmp_bak;

  return value;
}

// from) load.c:read_binary_header()
static int
read_binary_size(const uint8_t *bin, size_t *bin_size)
{
  const struct rite_binary_header *header = (const struct rite_binary_header *)bin;

  if (memcmp(header->binary_ident, RITE_BINARY_IDENT, sizeof(header->binary_ident)) == 0) {
	  // no-op
  } else if (memcmp(header->binary_ident, RITE_BINARY_IDENT_LIL, sizeof(header->binary_ident)) == 0) {
	  // no-op
  } else {
    return MRB_DUMP_INVALID_FILE_HEADER;
  }

  if (memcmp(header->binary_version, RITE_BINARY_FORMAT_VER, sizeof(header->binary_version)) != 0) {
    return MRB_DUMP_INVALID_FILE_HEADER;
  }

  *bin_size = (size_t)bin_to_uint32(header->binary_size);

  return MRB_DUMP_OK;
}
// from) load.c:irep_error()
static void
irep_error(mrb_state *mrb)
{
  mrb_exc_set(mrb, mrb_exc_new_str_lit(mrb, E_SCRIPT_ERROR, "irep load error"));
}

mrb_value mrb_ext_load_irep_cxt_suppress_alignment(mrb_state *mrb, const uint8_t *bin, mrbc_context *c) {
  size_t bin_size = 0;
  const size_t header_size = sizeof(struct rite_binary_header);

  int result = read_binary_size(bin, &bin_size);
  if (result != MRB_DUMP_OK || bin_size <= header_size) {
    irep_error(mrb);
    return mrb_nil_value();
  }

  // suppress alignment at run
  const uint8_t *cpy_bin = (const uint8_t *)mrb_malloc(mrb, bin_size);

  memcpy(cpy_bin, bin, bin_size);
  mrb_value value = mrb_load_irep_cxt(mrb, cpy_bin, c);
  mrb_free(mrb, cpy_bin);

  return value;
}

mrb_int mrb_ext_fixnum_to_cint(mrb_value value) {
  return mrb_fixnum(value);
}

mrb_float mrb_ext_float_to_cdouble(mrb_value value) {
  return mrb_float(value);
}

void* mrb_ext_ptr_to_ptr(mrb_value value) {
  return mrb_ptr(value);
}

mrb_sym mrb_ext_symbol_to_cuint(mrb_value value) {
 return mrb_symbol(value);
}

enum mrb_vtype mrb_ext_type(mrb_value value) {
	return mrb_type(value);
}

void* mrb_ext_data_ptr(mrb_value value) {
  return DATA_PTR(value);
}

mrb_value mrb_ext_nil() {
  return mrb_nil_value();
}

mrb_value mrb_ext_false() {
  return mrb_false_value();
}

mrb_value mrb_ext_true() {
  return mrb_true_value();
}

mrb_value mrb_ext_cint_to_fixnum(mrb_int value) {
  return mrb_fixnum_value(value);
}

mrb_value mrb_ext_cdouble_to_float(struct mrb_state* mrb, mrb_float value) {
  return mrb_float_value(mrb, value);
}

const char* mrb_ext_sym2name(struct mrb_state* mrb, mrb_value value) {
  return mrb_sym2name(mrb, mrb_symbol(value));
}

mrb_value mrb_ext_sym_new(struct mrb_state* mrb, const char* string,
                          size_t len) {
  mrb_value value;

  SET_SYM_VALUE(value, mrb_intern(mrb, string, len));

  return value;
}

void* mrb_ext_get_ptr(mrb_value value) {
    return mrb_cptr(value);
}

mrb_value mrb_ext_set_ptr(struct mrb_state* mrb, void* ptr) {
    mrb_value value;

    SET_CPTR_VALUE(mrb, value, ptr);

    return value;
}

mrb_data_type mrb_ext_data_type(const char* name, void (*dfree)(mrb_state *mrb, void*)) {
	mrb_data_type data_type = {
		.struct_name = name,
		.dfree = dfree
	};

	return data_type;
}

void mrb_ext_data_init(mrb_value* value, void* ptr, const mrb_data_type* type) {
  mrb_data_init(*value, ptr, type);
}

mrb_value mrb_ext_class_value(struct RClass* klass) {
  mrb_value value;

  mrb_ptr(value) = klass;
  mrb_type(value) = MRB_TT_CLASS;

  return value;
}

mrb_value mrb_ext_module_value(struct RClass* module) {
  mrb_value value;

  mrb_ptr(value) = module;
  mrb_type(value) = MRB_TT_MODULE;

  return value;
}

mrb_value mrb_ext_data_value(struct RData* data) {
  mrb_value value;

  mrb_ptr(value) = data;
  mrb_type(value) = MRB_TT_DATA;

  return value;
}

void mrb_ext_set_instance_tt(struct RClass* class, enum mrb_vtype type) {
  MRB_SET_INSTANCE_TT(class, type);
}

mrb_int mrb_ext_ary_len(struct mrb_state* mrb, mrb_value array) {
  return RARRAY_LEN(array);
}

mrb_sym mrb_ext_get_mid(struct mrb_state* mrb) {
  mrb_sym mid = mrb_get_mid(mrb);
  if (mid == mrb_intern_lit(mrb, "new")) {
    mid = mrb_intern_lit(mrb, "initialize");
  }

  return mid;
}

mrb_value mrb_ext_get_exc(struct mrb_state* mrb) {
  if (mrb->exc) {
    mrb_value exc = mrb_funcall(mrb, mrb_obj_value(mrb->exc), "inspect", 0);
    mrb_value backtrace = mrb_exc_backtrace(mrb, mrb_obj_value(mrb->exc));

    mrb_funcall(mrb, backtrace, "unshift", 1, exc);

    mrb->exc = NULL;

    return mrb_funcall(mrb, backtrace, "join", 1, mrb_str_new_cstr(mrb, "\n"));
  } else {
    return mrb_nil_value();
  }
}

void mrb_ext_raise_current(struct mrb_state* mrb) {
    if (mrb->exc) {
        mrb_exc_raise(mrb, mrb_obj_value(mrb->exc));
    }
}

mrb_value mrb_ext_exc_str(struct mrb_state* mrb, mrb_value exc) {
    return mrb_funcall(mrb, exc, "inspect", 0);
}

mrb_noreturn void mrb_ext_raise_nothrow(struct mrb_state* mrb, const char* eclass,
  const char* msg) {

  struct mrb_jmpbuf c_jmp;
  struct mrb_jmpbuf *pc_jmp_bak;
  pc_jmp_bak = mrb->jmp;

  MRB_TRY(&c_jmp) {
    mrb->jmp = &c_jmp;
    mrb_raise(mrb, mrb_class_get(mrb, eclass), msg);
  }
  MRB_CATCH(&c_jmp) {}
  MRB_END_EXC(&c_jmp);

  mrb->jmp = pc_jmp_bak;
}

mrb_bool mrb_ext_class_defined_under(struct mrb_state* mrb,
  struct RClass* outer, const char* name) {
  mrb_value sym = mrb_check_intern_cstr(mrb, name);

  if (mrb_nil_p(sym)) return FALSE;

  return mrb_const_defined(mrb, mrb_obj_value(outer), mrb_symbol(sym));
}

struct RClass* mrb_ext_class_ptr(mrb_value value) {
  return mrb_class_ptr(value);
}

struct RClass* mrb_ext_class(struct mrb_state* mrb, mrb_value value) {
  return mrb_class(mrb, value);
}

size_t mrb_ext_value_sizeof() {
	return sizeof(mrb_value);
}

size_t mrb_ext_data_type_sizeof() {
	return sizeof(mrb_data_type);
}

size_t mrb_ext_int_sizeof() {
	return sizeof(mrb_int);
}

size_t mrb_ext_float_sizeof() {
	return sizeof(mrb_float);
}

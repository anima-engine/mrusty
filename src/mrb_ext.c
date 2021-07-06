// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include <stdlib.h>

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/class.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/proc.h>
#include <mruby/value.h>
#include <mruby/variable.h>
#include <mruby/array.h>

void* mrb_ext_get_ud(struct mrb_state* mrb) {
  return mrb->ud;
}

void mrb_ext_set_ud(struct mrb_state* mrb, void* ud) {
  mrb->ud = ud;
}

int mrb_ext_fixnum_to_cint(mrb_value value) {
  return mrb_fixnum(value);
}

double mrb_ext_float_to_cdouble(mrb_value value) {
  return mrb_float(value);
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

mrb_value mrb_ext_cint_to_fixnum(int value) {
  return mrb_fixnum_value(value);
}

mrb_value mrb_ext_cdouble_to_float(struct mrb_state* mrb, double value) {
  return mrb_float_value(mrb, value);
}

mrb_value mrb_ext_proc_to_value(struct mrb_state* mrb, struct RProc* proc) {
  mrb_value value = mrb_cptr_value(mrb, proc);

  value.tt = MRB_TT_PROC;

  return value;
}

const char* mrb_ext_sym2name(struct mrb_state* mrb, mrb_value value) {
  return mrb_sym2name(mrb, mrb_symbol(value));
}

mrb_value mrb_ext_sym_new(struct mrb_state* mrb, const char* string,
                          size_t len) {
  mrb_value value;

  mrb_symbol(value) = mrb_intern(mrb, string, len);

  value.tt = MRB_TT_SYMBOL;

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

void mrb_ext_data_init(mrb_value* value, void* ptr, const mrb_data_type* type) {
  mrb_data_init(*value, ptr, type);
}

mrb_value mrb_ext_class_value(struct RClass* klass) {
  mrb_value value;

  value.value.p = klass;
  value.tt = MRB_TT_CLASS;

  return value;
}

mrb_value mrb_ext_module_value(struct RClass* module) {
  mrb_value value;

  value.value.p = module;
  value.tt = MRB_TT_MODULE;

  return value;
}

mrb_value mrb_ext_data_value(struct RData* data) {
  mrb_value value;

  value.value.p = data;
  value.tt = MRB_TT_DATA;

  return value;
}

void mrb_ext_set_instance_tt(struct RClass* class, enum mrb_vtype type) {
  MRB_SET_INSTANCE_TT(class, type);
}

int mrb_ext_ary_len(struct mrb_state* mrb, mrb_value array) {
  return RARRAY_LEN(array);
}

unsigned int mrb_ext_get_mid(struct mrb_state* mrb) {
  return mrb_get_mid(mrb);
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

mrb_noreturn void mrb_ext_raise(struct mrb_state* mrb, const char* eclass,
  const char* msg) {
  mrb_raise(mrb, mrb_class_get(mrb, eclass), msg);
}

mrb_bool mrb_ext_class_defined_under(struct mrb_state* mrb,
  struct RClass* outer, const char* name) {
  mrb_value sym = mrb_check_intern_cstr(mrb, name);

  if (mrb_nil_p(sym)) return FALSE;

  return mrb_const_defined(mrb, mrb_obj_value(outer), mrb_symbol(sym));
}

struct RClass* mrb_ext_get_class(mrb_value value) {
  return (struct RClass*) value.value.p;
}

struct RClass* mrb_ext_class(struct mrb_state* mrb, mrb_value value) {
  return mrb_class(mrb, value);
}

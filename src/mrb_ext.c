// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#include <stdlib.h>

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/class.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/value.h>
#include <mruby/proc.h>

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

void mrb_ext_data_init(mrb_value* value, void* ptr, const mrb_data_type* type) {
  mrb_data_init(*value, ptr, type);
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
  return mrb_ary_len(mrb, array);
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

mrb_noreturn void mrb_ext_raise(struct mrb_state* mrb, const char* msg) {
  mrb_raise(mrb, E_RUNTIME_ERROR, msg);
}

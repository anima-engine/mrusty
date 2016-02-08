#include <stdlib.h>

#include <mruby.h>
#include <mruby/error.h>
#include <mruby/value.h>
#include <mruby/proc.h>

typedef struct rust_type {
  const char* ptr;
  size_t size;
} rust_type;

int mrb_ext_fixnum_to_cint(mrb_value value) {
  return mrb_fixnum(value);
}

double mrb_ext_float_to_cdouble(mrb_value value) {
  return mrb_float(value);
}

struct RProc* mrb_ext_value_to_proc(mrb_value value) {
  return mrb_proc_ptr(value);
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

mrb_value mrb_ext_rust_to_ptr(struct mrb_state* mrb, const char* ptr, size_t size) {
  rust_type* obj = malloc(sizeof(rust_type));
  const char* new_ptr = malloc(size);

  memcpy(new_ptr, ptr, size);

  obj->ptr = new_ptr;
  obj->size = size;

  return mrb_cptr_value(mrb, obj);
}

rust_type mrb_ext_ptr_to_rust(mrb_value ptr) {
  rust_type* obj = mrb_ptr(ptr);

  return *obj;
}

void mrb_ext_free_rust(mrb_value ptr) {
  rust_type* obj = mrb_ptr(ptr);

  free(obj->ptr);
  free(obj);
}

mrb_value mrb_ext_get_exc(struct mrb_state* mrb) {
  mrb_value exc = mrb_funcall(mrb, mrb_obj_value(mrb->exc), "inspect", 0);
  mrb_value backtrace = mrb_exc_backtrace(mrb, mrb_obj_value(mrb->exc));

  mrb_funcall(mrb, backtrace, "unshift", 1, exc);

  return mrb_funcall(mrb, backtrace, "join", 1, mrb_str_new_cstr(mrb, "\n"));
}

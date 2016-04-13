# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# An assertion exception.
class AssertError < Exception
end

# Module containing the main describe method which runs the spec.
module Spec
  def self.describe(target, &block)
    Context.new(target, &block).run
  end
end

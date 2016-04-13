# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing method responding.
#
# it { is_expected.to respond_to :hi }
class RespondMatcher
  def initialize(_name, target)
    @target = target
  end

  def match(subject)
    fail AssertError, "#{subject} does not respond to #{@target}" unless
      subject.respond_to? @target
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} responds to #{@target}" if
      subject.respond_to? @target
  end

  def describe
    if @negative
      "to not respond to #{@target}"
    else
      "to respond to #{@target}"
    end
  end

  def self.match(method)
    method == :respond_to
  end
end

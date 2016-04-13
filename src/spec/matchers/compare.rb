# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing comparison.
#
# it { is_expected.to be < 2 }
class CompareMatcher
  def initialize(_name)
  end

  def match(subject)
    fail AssertError, "#{subject} is not #{@name} to #{@target}" unless
      subject.send @name, @target
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is #{@name} to #{@target}" if
      subject.send @name, @target
  end

  def describe
    if @negative
      "to not be #{@name} to #{@target}"
    else
      "to be #{@name} to #{@target}"
    end
  end

  def method_missing(method, *args)
    if [:<, :<=, :>, :>=].include? method
      @name = method
      @target = args[0]

      self
    else
      super
    end
  end

  def self.match(method)
    method == :be
  end
end

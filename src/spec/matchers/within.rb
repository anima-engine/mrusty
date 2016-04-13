# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing float values.
#
# it 'has to know its values' do
#   expect(3.14).to be_within(0.01).of Math::PI
# end
class WithinMatcher
  def initialize(_name, within)
    @within = within
  end

  def of(target)
    @target = target

    self
  end

  def match(subject)
    fail AssertError,
         "#{subject} is not within #{@within} of #{@target}" if
      (subject - @target).abs > @within
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is within #{@within} of #{@target}" if
      (subject - @target).abs <= @within
  end

  def describe
    if @negative
      "to not be within #{@within} of #{@target}"
    else
      "to be within #{@within} of #{@target}"
    end
  end

  def self.match(method)
    method == :be_within
  end
end

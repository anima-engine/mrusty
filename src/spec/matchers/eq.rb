# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing equality.
#
# it 'expects two integer to be equal' do
#   expect(1).to eq 1
# end
class EqMatcher
  def initialize(_name, target)
    @target = target
  end

  def match(subject)
    fail AssertError, "#{subject} is not equal to #{@target}" if
      subject != @target
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is equal to #{@target}" if
      subject == @target
  end

  def describe
    if @negative
      "to not be equal to #{@target}"
    else
      "to be equal to #{@target}"
    end
  end

  def self.match(method)
    method == :eq ||
      method == :eql ||
      method == :equal
  end
end

# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing #<name>? boolean-returning methods.
#
# it 'is empty' do
#   expect([]).to be_empty
# end
class BeMatcher
  def initialize(name)
    @name = name.to_s[3..-1]
  end

  def match(subject)
    fail AssertError, "#{subject} is not #{@name}" unless
      subject.send (@name + '?').to_sym
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is #{@name}" if
      subject.send (@name + '?').to_sym
  end

  def describe
    if @negative
      "to not be #{@name}"
    else
      "to be #{@name}"
    end
  end

  def self.match(method)
    method = method.to_s

    method[0..2] == 'be_' &&
      method[-1] != '?'
  end
end

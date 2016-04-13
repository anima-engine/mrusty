# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing has_<something>? methods.
#
# it 'has the key' do
#   expect({key: 42}).to have_key :key
# end
class HaveMatcher
  def initialize(name, target)
    @name = name.to_s[5..-1]
    @target = target
  end

  def match(subject)
    fail AssertError,
         "#{subject} does not have #{@name} #{@target}" unless
      subject.send(('has_' + @name + '?').to_sym, @target)
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} has #{@name} #{@target}" if
      subject.send(('has_' + @name + '?').to_sym, @target)
  end

  def describe
    if @negative
      "to not not have #{@name} #{@target}"
    else
      "to have #{@name} #{@target}"
    end
  end

  def self.match(method)
    method = method.to_s

    method[0..4] == 'have_' &&
      method[-1] != '?'
  end
end

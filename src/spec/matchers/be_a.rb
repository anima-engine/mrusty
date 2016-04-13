# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing types.
#
# it { is_expected.to be_an Integer }
class BeAMatcher
  def initialize(name, target)
    @name = name.to_s
    @target = target
  end

  def match(subject)
    article = @name.end_with?('an') ? 'an' : 'a'

    fail AssertError, "#{subject} is not #{article} #{@target}" unless
      subject.is_a? @target
  end

  def match_not(subject)
    @negative = true
    article = @name.end_with?('an') ? 'an' : 'a'

    fail AssertError, "#{subject} is #{article} #{@target}" if
      subject.is_a? @target
  end

  def describe
    article = @name.end_with?('an') ? 'an' : 'a'

    if @negative
      "to not be #{article} #{@target}"
    else
      "to be #{article} #{@target}"
    end
  end

  def self.match(method)
    method == :be_a ||
      method == :be_an
  end
end

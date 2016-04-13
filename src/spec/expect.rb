# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# An assert. They can be defined with the expect or is_expected methods.
#
# expect(something). to ...
# is_expected.not_to ...
class Expect
  def initialize(target, is = false)
    @target = target
    @is = is
    @failed = true
  end

  def to(matcher)
    @matcher = matcher

    exc = matcher.match @target

    @failed = false

    exc
  end

  def not_to(matcher)
    @matcher = matcher

    exc = matcher.match_not @target

    @failed = false

    exc
  end

  def describe
    @target = @target.inspect if @target.is_a? Exception

    expect = if @is
               "is expected #{@matcher.describe}"
             else
               "expect #{@target} #{@matcher.describe}"
             end

    expect + (@failed ? ' FAILED' : '')
  end
end

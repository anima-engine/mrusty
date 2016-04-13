# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# An example is a spec test. They can be defined with the it method.
#
# it 'description' do
#   ... # some expects
# end
class Example
  MATCHERS = [BeAMatcher, CompareMatcher, EqMatcher, HaveMatcher, FalseyMatcher,
              RaiseMatcher, RespondMatcher, TruthyMatcher, WithinMatcher,
              BeMatcher]

  def initialize(parent, description, &block)
    @parent = parent
    @description = description
    @block = block
    @expects = []
  end

  def expect(target = nil, &block)
    if block
      begin
        expect = Expect.new instance_eval(&block)
      rescue Exception => exception
        expect = Expect.new exception
      end
    else
      expect = Expect.new target
    end

    @expects << expect

    expect
  end

  def is_expected
    expect = Expect.new subject, true

    @expects << expect

    expect
  end

  def subject
    @parent.subject
  end

  def method_missing(method, *args)
    matcher = MATCHERS.find { |m| m.match method }

    if matcher
      matcher.new(method, *args)
    else
      super
    end
  end

  def describe(depth)
    it = '  ' * depth + 'it ' + @description

    if @expects.size == 0
      it
    elsif @expects.size == 1 && @description.empty?
      it + @expects[0].describe
    else
      it + "\n" + @expects.map { |e| '  ' * (depth + 1) + e.describe }.join("\n")
    end
  end

  def run(_depth)
    instance_eval(&@block)
  rescue Exception => exception
    exception
  end
end

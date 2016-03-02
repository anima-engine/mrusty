# mrusty. mruby bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

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

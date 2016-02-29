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

class Example
  MATCHERS = [EqMatcher]

  def initialize(parent, description, &block)
    @parent = parent
    @description = description
    @block = block
    @expects = []
  end

  def expect(target)
    expect = Expect.new target

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
    example = MATCHERS.inject(nil) { |_a, e| e.match(method) ? e : nil }

    if example
      example.new(*args)
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

  def run(depth)
    begin
      instance_eval(&@block)
    rescue Exception => exception
      exception
    end
  end
end

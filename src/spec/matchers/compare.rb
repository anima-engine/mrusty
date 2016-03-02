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

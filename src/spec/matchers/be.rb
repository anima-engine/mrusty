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

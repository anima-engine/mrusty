# mrusty. mruby safe bindings for Rust
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

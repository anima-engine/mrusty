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

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

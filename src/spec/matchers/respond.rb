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

# A matcher useful for testing method responding.
#
# it { is_expected.to respond_to :hi }
class RespondMatcher
  def initialize(_name, target)
    @target = target
  end

  def match(subject)
    fail AssertError, "#{subject} does not respond to #{@target}" unless
      subject.respond_to? @target
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} responds to #{@target}" if
      subject.respond_to? @target
  end

  def describe
    if @negative
      "to not respond to #{@target}"
    else
      "to respond to #{@target}"
    end
  end

  def self.match(method)
    method == :respond_to
  end
end

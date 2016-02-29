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

class Expect
  def initialize(target, is = false)
    @target = target
    @is = is
  end

  def to(matcher)
    @matcher = matcher

    matcher.match @target
  end

  def not_to(matcher)
    @matcher = matcher

    matcher.match_not @target
  end

  def describe
    if @is
      "is expected #{@matcher.describe}"
    else
      "expect #{@target} #{@matcher.describe}"
    end
  end
end

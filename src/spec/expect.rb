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
    @failed = true
  end

  def to(matcher)
    @matcher = matcher

    exc = matcher.match @target

    @failed = false

    exc
  end

  def not_to(matcher)
    @matcher = matcher

    exc = matcher.match_not @target

    @failed = false

    exc
  end

  def describe
    expect = if @is
               "is expected #{@matcher.describe}"
             else
               "expect #{@target} #{@matcher.describe}"
             end

    expect + (@failed ? ' FAILED' : '')
  end
end

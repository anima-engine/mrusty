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

class RaiseMatcher
  def initialize(_name, klass, message = nil)
    @klass = klass
    @message = message
  end

  def match(subject)
    fail AssertError, "#{subject.class} is not a a #{@klass}" unless
      subject.is_a? @klass

    if @message
      fail AssertError, "\"#{subject.message}\" is not \"#{@message}\"" if
        subject.message != @message
    end
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject.class} is a #{@klass}" if subject.is_a? @klass

    if @message
      fail AssertError, "\"#{subject.message}\" is \"#{@message}\"" if
        subject.message == @message
    end
  end

  def describe
    if @negative
      "to not raise error #{@klass}" + (@message ? ", #{@message}" : '')
    else
      "to raise error #{@klass}" + (@message ? ", #{@message}" : '')
    end
  end

  def self.match(method)
    method == :raise_error
  end
end

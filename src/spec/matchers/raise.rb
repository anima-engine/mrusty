# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing exception raising.
#
# it 'raises something' do
#   expect { raise Exception }.to raise_error Exception
#   expect { raise 'hi' }.to raise_error RuntimeError, 'hi'
# end
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

class MyException < Exception
end

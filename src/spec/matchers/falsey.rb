# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing falsey values.
#
# it 'knows a lie when it sees it' do
#   expect(false).to be_falsey
#   expect(nil).to be_falsey
# end
class FalseyMatcher
  def initialize(_name)
  end

  def match(subject)
    fail AssertError, "#{subject} is not falsey" if subject
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is falsey" unless subject
  end

  def describe
    if @negative
      'to not be falsey'
    else
      'to be falsey'
    end
  end

  def self.match(method)
    method == :be_falsey
  end
end

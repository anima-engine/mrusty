# mrusty. mruby safe bindings for Rust
# Copyright (C) 2016  Dragoș Tiselice
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# A matcher useful for testing truthy values.
#
# it 'knows a lie when it sees it' do
#   expect(true).to be_truthy
#   expect(0).to be_truthy
# end
class TruthyMatcher
  def initialize(_name)
  end

  def match(subject)
    fail AssertError, "#{subject} is not truthy" unless subject
  end

  def match_not(subject)
    @negative = true

    fail AssertError, "#{subject} is truthy" if subject
  end

  def describe
    if @negative
      'to not be truthy'
    else
      'to be truthy'
    end
  end

  def self.match(method)
    method == :be_truthy
  end
end

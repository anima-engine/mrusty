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

class Context
  def initialize(target, parent = nil, &block)
    @target = target
    @parent = parent
    @children = []

    instance_eval(&block)
  end

  def subject
    if block_given?
      @subject = yield
    else
      if @subject
        @subject
      else
        if @target.is_a? Class
          @target.new
        else
          @parent.subject
        end
      end
    end
  end

  def it(description = '', &block)
    @children << Example.new(self, description, &block)
  end

  def context(target, &block)
    @children << Context.new(target, self, &block)
  end

  def let(name)
    value = yield

    Example.send(:define_method, name) { value }
  end

  def describe(depth)
    context = '  ' * depth + @target.to_s

    context + "\n" + @children.map { |c| c.describe depth + 1 }.join("\n")
  end

  def run(depth = 0)
    tests = @children.map { |e| e.run depth + 1 }.flatten

    if depth == 0
      puts
      puts describe depth

      failures = tests.compact

      unless failures.empty?
        puts "\nFAILURES:\n\n"

        ok = tests.size - failures.size
        failed = failures.count { |e| e.is_a? AssertError }
        errors = tests.size - ok - failed

        failures = failures.each_with_index.map do |e, i|
          backtrace = e.backtrace.map do |l|
            '  ' + l.split('mruby-1.2.0/').last
          end
          backtrace = backtrace.join "\n"

          "  #{i + 1}) " + e.inspect + "\n\n" + backtrace
        end

        puts failures.join "\n\n"

        puts "\n#{ok} ok, #{failed} failed, #{errors} errors.\n\n"

        return false
      end

      puts "\n#{tests.size} ok, 0 failed, 0 errors.\n\n"

      true
    else
      tests
    end
  end
end

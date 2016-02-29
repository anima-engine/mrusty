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
        if @parent
          @parent.subject
        else
          target.new
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
    failures = @children.map { |e| e.run depth + 1 }.flatten.compact

    if depth == 0
      puts describe depth

      unless failures.empty?
        puts "\nFAILURES:\n\n"

        failures = failures.each_with_index.map do |e, i|
          backtrace = e.backtrace.map { |l| '  ' + l }.join "\n"

          "  #{i + 1}) " + e.inspect + "\n\n" + backtrace
        end

        puts failures.join "\n\n"

        return false
      end

      puts

      true
    else
      failures
    end
  end
end

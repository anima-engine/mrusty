// mrusty. mruby bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::mruby::*;

/// A `macro` useful to run mruby specs.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// spec!("
///     describe Class do
///       context 'when 1' do
///         subject { 1 }
///
///         it { is_expected.to eql 1 }
///       end
///
///       context 'when 1' do
///         subject { 1 }
///         let(:one) { 1 }
///
///         it 'won\\'t' do
///           expect(1).to eql one
///         end
///       end
///     end
/// ");
/// # }
/// ```
#[macro_export]
macro_rules! spec {
    ( $script:expr ) => {
        #[test]
        fn spec() {
            let spec = Spec::new($script);

            assert!(spec.run());
        }
    }
}

pub struct Spec<'a> {
    script: &'a str,
    mruby: MRubyType
}

impl<'a> Spec<'a> {
    pub fn new(script: &str) -> Spec {
        let mruby = MRuby::new();

        mruby.filename("spec");

        mruby.run(include_str!("spec/matchers/eq.rb")).unwrap();
        mruby.run(include_str!("spec/context.rb")).unwrap();
        mruby.run(include_str!("spec/example.rb")).unwrap();
        mruby.run(include_str!("spec/expect.rb")).unwrap();
        mruby.run(include_str!("spec/spec.rb")).unwrap();

        Spec {
            script: script,
            mruby: mruby
        }
    }

    pub fn run(&self) -> bool {
        self.mruby.run(self.script).unwrap().to_bool().unwrap()
    }
}

spec!("
    describe Class do
      context 'when 1' do
        subject { 1 }

        it { is_expected.to eql 1 }
      end

      context 'when 1' do
        subject { 1 }
        let(:one) { 1 }

        it 'won\\'t' do
          expect(1).to eql one
        end
      end
    end
");

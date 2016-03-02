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

use std::any::Any;

/// A `macro` useful to run mruby specs.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// struct Cont;
///
/// impl MRubyFile for Cont {
///     fn require(mruby: MRubyType) {
///         mruby.def_class::<Cont>("Container");
///     }
/// }
///
/// describe!(Cont, "
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
macro_rules! describe {
    ( $t:ty, $spec:expr ) => {
        #[test]
        fn spec() {
            let spec = Spec::new::<$t>($spec);

            assert!(spec.run());
        }
    }
}

pub struct Spec {
    script: String,
    target: String,
    mruby: MRubyType
}

impl Spec {
    pub fn new<T: MRubyFile + Any>(script: &str) -> Spec {
        let mruby = MRuby::new();

        T::require(mruby.clone());

        mruby.filename("matchers/be.rb");
        mruby.run(include_str!("spec/matchers/be.rb")).unwrap();

        mruby.filename("matchers/be_a.rb");
        mruby.run(include_str!("spec/matchers/be_a.rb")).unwrap();

        mruby.filename("matchers/compare.rb");
        mruby.run(include_str!("spec/matchers/compare.rb")).unwrap();

        mruby.filename("matchers/eq.rb");
        mruby.run(include_str!("spec/matchers/eq.rb")).unwrap();

        mruby.filename("matchers/falsey.rb");
        mruby.run(include_str!("spec/matchers/falsey.rb")).unwrap();

        mruby.filename("matchers/have.rb");
        mruby.run(include_str!("spec/matchers/have.rb")).unwrap();

        mruby.filename("matchers/raise.rb");
        mruby.run(include_str!("spec/matchers/raise.rb")).unwrap();

        mruby.filename("matchers/respond.rb");
        mruby.run(include_str!("spec/matchers/respond.rb")).unwrap();

        mruby.filename("matchers/truthy.rb");
        mruby.run(include_str!("spec/matchers/truthy.rb")).unwrap();

        mruby.filename("matchers/within.rb");
        mruby.run(include_str!("spec/matchers/within.rb")).unwrap();

        mruby.filename("context.rb");
        mruby.run(include_str!("spec/context.rb")).unwrap();

        mruby.filename("example.rb");
        mruby.run(include_str!("spec/example.rb")).unwrap();

        mruby.filename("expect.rb");
        mruby.run(include_str!("spec/expect.rb")).unwrap();

        mruby.filename("spec.rb");
        mruby.run(include_str!("spec/spec.rb")).unwrap();

        let name = mruby.class_name::<T>().unwrap();

        Spec {
            script: script.to_string(),
            target: name,
            mruby: mruby
        }
    }

    pub fn run(&self) -> bool {
        let describe = format!("
            describe {} do
              {}
            end
        ", self.target, self.script);

        self.mruby.run(&describe).unwrap().to_bool().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    struct Empty;

    impl MRubyFile for Empty {
        fn require(mruby: MRubyType) {
            mruby.def_class::<Empty>("Empty");
        }
    }

    describe!(Empty, "
      context Fixnum do
        context 'when 1' do
          subject { 1 }

          it { is_expected.to eq 1 }
          it { is_expected.not_to eq 2 }

          it { is_expected.to be_a Fixnum }
          it { is_expected.not_to be_a String }

          it { is_expected.to be_within(0).of(1) }
          it { is_expected.not_to be_within(0.0001).of(2) }

          it { is_expected.to be < 2 }
          it { is_expected.to be <= 2 }
          it { is_expected.not_to be > 2 }
          it { is_expected.not_to be >= 2 }

          it 'is different from 2' do
            expect(1 == 2).to be_falsey
            expect(1 == 2).not_to be_truthy
            expect(1 != 2).to be_truthy
            expect(1 != 2).not_to be_falsey
          end

          it 'does not concatenate with String' do
            expect { '' + 1 }.to raise_error TypeError, \"expected String\"
            expect { 1 + '' }.not_to raise_error Exception
          end

          it { is_expected.to respond_to :to_s }
          it { is_expected.not_to respond_to :to_sym }
        end
      end

      context Hash do
        context 'empty' do
          it { is_expected.to be_empty }
        end

        context 'when {a: 1}' do
          subject { {a: 1} }

          it { is_expected.to have_key :a }
          it { is_expected.not_to be_empty }
        end
      end
    ");
}

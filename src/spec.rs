// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::mruby::*;
use crate::{mrfn, mruby_class, mruby_defines};

/// A `macro` useful to run mruby specs. You can pass a tuple of `MrubyFile`s dependencies
/// as a second argument.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::{Mruby, MrubyFile, MrubyImpl, MrubyType};
///
/// # fn main() {
/// struct Cont;
///
/// impl MrubyFile for Cont {
///     fn require(mruby: MrubyType) {
///         mruby.def_class_for::<Cont>("Container");
///     }
/// }
///
/// describe!(Cont, (Cont, Cont), "
///   context 'when 1' do
///     subject { 1 }
///
///     it { is_expected.to eql 1 }
///   end
///
///   context 'when 1' do
///     subject { 1 }
///     let(:one) { 1 }
///
///     it 'won\\'t' do
///       expect(1).to eql one
///     end
///   end
/// ");
/// # }
/// ```
#[macro_export]
macro_rules! describe {
    ( $t:ty, $spec:expr ) => {
        #[test]
        fn spec() {
            let mruby = $crate::Mruby::new();

            <$t as $crate::MrubyFile>::require(mruby.clone());

            let name = $crate::MrubyImpl::class_name_for::<$t>(&mruby).unwrap();

            let spec = $crate::Spec::new(mruby, &name, $spec);

            assert!(spec.run());
        }
    };

    ( $t:ty, ( $( $ts:ty ),+ ), $spec:expr ) => {
        #[test]
        fn spec() {
            let mruby = $crate::Mruby::new();

            <$t as $crate::MrubyFile>::require(mruby.clone());
            $( <$ts as $crate::MrubyFile>::require(mruby.clone()); )*

            let name = $crate::MrubyImpl::class_name_for::<$t>(&mruby).unwrap();

            let spec = $crate::Spec::new(mruby, &name, $spec);

            assert!(spec.run());
        }
    }
}

/// A `struct` useful for mruby spec definition and running.
///
/// Available matchers:
///
/// * `be_a`, `be_an` - type testing
/// * `be_<somehow>` - test boolean-returning `<name>?` methods
/// * `be <`, `be <=`, `be >`, `be >=` - test relation
/// * `be_eq`, `be_eql`, `be_equal` - test equality
/// * `be_falsey` - test falsey things
/// * `be_truthy` - test truthy things
/// * `have_<something>` - test boolean-returning `has_<name>?` methods
/// * `raise_error` - test errors
/// * `respond_to` - test method responding
/// * `be_within(value).of` - test value
///
/// # Examples
///
/// ```
/// # use mrusty::{Mruby, MrubyFile, MrubyImpl, MrubyType, Spec};
/// struct Cont;
///
/// impl MrubyFile for Cont {
///     fn require(mruby: MrubyType) {
///         mruby.def_class_for::<Cont>("Container");
///     }
/// }
///
/// let mruby = Mruby::new();
/// Cont::require(mruby.clone());
///
/// let spec = Spec::new(mruby, "Container", "
///     context 'when 1' do
///       subject { 1 }
///
///       it { is_expected.to eql 1 }
///     end
///
///     context 'when 1' do
///       subject { 1 }
///       let(:one) { 1 }
///
///       it 'won\\'t' do
///         expect(1).to eql one
///       end
///     end
/// ");
///
/// assert_eq!(spec.run(), true);
/// ```
pub struct Spec {
    script: String,
    target: String,
    mruby: MrubyType,
}

impl Spec {
    /// Creates an mruby spec runner.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::{Mruby, MrubyFile, MrubyImpl, MrubyType, Spec};
    /// struct Cont;
    ///
    /// impl MrubyFile for Cont {
    ///     fn require(mruby: MrubyType) {
    ///         mruby.def_class_for::<Cont>("Container");
    ///     }
    /// }
    ///
    /// let mruby = Mruby::new();
    /// Cont::require(mruby.clone());
    ///
    /// let spec = Spec::new(mruby, "Container", "
    ///     context 'when 1' do
    ///       subject { 1 }
    ///
    ///       it { is_expected.to eql 1 }
    ///     end
    ///
    ///     context 'when 1' do
    ///       subject { 1 }
    ///       let(:one) { 1 }
    ///
    ///       it 'won\\'t' do
    ///         expect(1).to eql one
    ///       end
    ///     end
    /// ");
    /// ```
    pub fn new(mruby: MrubyType, name: &str, script: &str) -> Spec {
        mruby_class!(mruby, "Prelude", {
            def_self!("puts", |mruby, _rbself: Value, msg: (&str)| {
                println!("{}", msg);
                mruby.nil()
            });
        });

        mruby
            .run(
                "module Kernel
                   def puts(arg)
                     Prelude.puts arg
                   end
                 end",
            )
            .ok();

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

        Spec {
            script: script.to_owned(),
            target: name.to_owned(),
            mruby: mruby,
        }
    }

    /// Runs mruby specs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::{Mruby, MrubyFile, MrubyImpl, MrubyType, Spec};
    /// struct Cont;
    ///
    /// impl MrubyFile for Cont {
    ///     fn require(mruby: MrubyType) {
    ///         mruby.def_class_for::<Cont>("Container");
    ///     }
    /// }
    ///
    /// let mruby = Mruby::new();
    /// Cont::require(mruby.clone());
    ///
    /// let spec = Spec::new(mruby, "Container", "
    ///     context 'when 1' do
    ///       subject { 1 }
    ///
    ///       it { is_expected.to eql 1 }
    ///     end
    ///
    ///     context 'when 1' do
    ///       subject { 1 }
    ///       let(:one) { 1 }
    ///
    ///       it 'won\\'t' do
    ///         expect(1).to eql one
    ///       end
    ///     end
    /// ");
    ///
    /// assert_eq!(spec.run(), true);
    /// ```
    pub fn run(&self) -> bool {
        let describe = format!(
            "
            Spec.describe {} do
              {}
            end
        ",
            self.target, self.script
        );

        self.mruby.run(&describe).unwrap().to_bool().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    struct Empty;

    impl MrubyFile for Empty {
        fn require(mruby: MrubyType) {
            mruby.def_class_for::<Empty>("Empty");
        }
    }

    describe!(
        Empty,
        "
      context Fixnum do
        context 'when 1' do
          subject { 1 }
          let(:two) { 2 }

          it 'does irrelevant things' do
            a = 2
          end

          it { is_expected.to eq 1 }
          it { is_expected.not_to eq two }

          it { is_expected.to be_an Integer }
          it { is_expected.not_to be_a String }

          it { is_expected.to be_within(0).of(1) }
          it { is_expected.not_to be_within(0.0001).of(two) }

          it { is_expected.to be < two }
          it { is_expected.to be <= two }
          it { is_expected.not_to be > two }
          it { is_expected.not_to be >= two }

          it 'is different from 2' do
            expect(1 == two).to be_falsey
            expect(1 == two).not_to be_truthy
            expect(1 != two).to be_truthy
            expect(1 != two).not_to be_falsey
          end

          it 'does not concatenate with String' do
            expect { '' + 1 }.to raise_error TypeError, \"Integer cannot be converted to String\"
            expect { 1 + '' }.not_to raise_error MyException
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
    "
    );
}
